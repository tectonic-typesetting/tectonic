/* Copyright 2020 the Tectonic Project
 * Licensed under the MIT License.
 */

#include "tectonic_bridge_core.h"
#include "bibtex_bindings.h"

#include <stdio.h> /* EOF, snprintf */

#define TRY(exec) do { if (!(exec)) { longjmp(error_jmpbuf, 1); } } while (false)

/* hack: the name eof conflicts with other function declarations under mingw. */
#define eof tectonic_eof

/* (Re)Allocate N items of type T using xmalloc/xrealloc.  */
#define XTALLOC(n, t) (xcalloc (n, sizeof (t)))

#define BIB_XRETALLOC_NOSET(array_name, array_var, type, size_var, new_size) \
  (array_var) = (type *) xrealloc((array_var), (new_size + 1) * sizeof(type))

#define BIB_XRETALLOC(array_name, array_var, type, size_var, new_size) do { \
  BIB_XRETALLOC_NOSET(array_name, array_var, type, size_var, new_size); \
  size_var = new_size; \
} while (0)

#define BIB_XRETALLOC_STRING(array_name, array_var, length, size_var, new_size) \
  (array_var) = (ASCII_code *) xrealloc((array_var), (new_size) * (length + 1) * sizeof(ASCII_code))

/* duplicated from xetexd.h: */

#define xmalloc_array(type,size) (xmalloc((size+1)*sizeof(type)))

#include <setjmp.h>

static jmp_buf error_jmpbuf, recover_jmpbuf;

/*14:*/

#define hash_base 1 /*empty 1*/
#define quote_next_fn (hash_base - 1)
#define min_print_line 3
#define max_print_line 79
#define aux_stack_size 20
#define WIZ_FN_SPACE 3000
#define SINGLE_FN_SPACE 100
#define ENT_STR_SIZE 250
#define GLOB_STR_SIZE 20000
#define MAX_GLOB_STRS 10
#define MAX_FIELDS 17250
#define LIT_STK_SIZE 100

/*22: */

typedef unsigned char ASCII_code;
typedef int32_t buf_pointer;
typedef ASCII_code *buf_type;
typedef size_t pool_pointer;
typedef int32_t str_number;
typedef int32_t hash_loc;
typedef int32_t hash_pointer;
typedef unsigned char /*last_ilk */ str_ilk;
typedef unsigned char /*longest_pds */ pds_len;
typedef const char *pds_type;
typedef int32_t aux_number;
typedef int32_t bib_number;
typedef int32_t cite_number;
typedef unsigned char /*last_fn_class */ fn_class;
typedef int32_t wiz_fn_loc;
typedef int32_t int_ent_loc;
typedef int32_t str_ent_loc;
typedef int32_t field_loc;
typedef int32_t hash_ptr2;
typedef unsigned char /*last_lit_type */ stk_type;
typedef int32_t blt_in_range;

static int32_t wiz_fn_space;
static int32_t ent_str_size;
static int32_t glob_str_size;
static int32_t max_glob_strs;
static int32_t max_fields;
static int32_t hash_size;
static int32_t hash_prime;
static int32_t hash_max;
static int32_t end_of_def;
static int32_t undefined;
static int32_t bad;
static int32_t string_width;
static hash_pointer *hash_next;
static str_number *hash_text;
static str_ilk *hash_ilk;
static int32_t *ilk_info;
static int32_t hash_used;
static bool hash_found;
static str_number s_aux_extension;
static str_number s_log_extension;
static str_number s_bbl_extension;
static int32_t command_num;
static rust_output_handle_t bbl_file;
static bool bib_seen;
static bool bst_seen;
static cite_number entry_cite_ptr;
static cite_number num_cites;
static cite_number old_num_cites;
static bool citation_seen;
static hash_loc cite_loc;
static hash_loc lc_cite_loc;
static hash_loc lc_xcite_loc;
static bool all_entries;
static cite_number all_marker;
static int32_t bbl_line_num;
static hash_loc fn_loc;
static hash_loc wiz_loc;
static hash_loc literal_loc;
static hash_loc macro_name_loc;
static hash_loc macro_def_loc;
static fn_class *fn_type;
static wiz_fn_loc wiz_def_ptr;
static hash_ptr2 *wiz_functions;
static int_ent_loc int_ent_ptr;
static int32_t *entry_ints;
static int_ent_loc num_ent_ints;
static str_ent_loc str_ent_ptr;
static ASCII_code *entry_strs;
static str_ent_loc num_ent_strs;
static int32_t str_glb_ptr;
static str_number *glb_bib_str_ptr;
static ASCII_code *global_strs;
static int32_t *glb_str_end;
static int32_t num_glb_strs;
static field_loc field_ptr;
static field_loc field_parent_ptr, field_end_ptr;
static cite_number cite_parent_ptr, cite_xptr;
static str_number *field_info;
static field_loc num_fields;
static field_loc num_pre_defined_fields;
static field_loc crossref_num;
static bool entry_seen;
static bool read_seen;
static bool read_performed;
static bool reading_completed;
static int32_t impl_fn_num;
static hash_loc entry_type_loc;
static bool type_exists;
static bool store_entry;
static hash_loc field_name_loc;
static hash_loc field_val_loc;
static bool store_field;
static bool store_token;
static ASCII_code right_outer_delim;
static ASCII_code right_str_delim;
static hash_loc cur_macro_loc;
static bool cite_hash_found;
static int32_t bib_brace_level;
static int32_t ent_chr_ptr;
static int32_t glob_chr_ptr;
static cite_number sort_cite_ptr;
static str_ent_loc sort_key_num;
static int32_t brace_level;
static hash_loc b_equals;
static hash_loc b_greater_than;
static hash_loc b_less_than;
static hash_loc b_plus;
static hash_loc b_minus;
static hash_loc b_concatenate;
static hash_loc b_gets;
static hash_loc b_add_period;
static hash_loc b_call_type;
static hash_loc b_change_case;
static hash_loc b_chr_to_int;
static hash_loc b_cite;
static hash_loc b_duplicate;
static hash_loc b_empty;
static hash_loc b_format_name;
static hash_loc b_if;
static hash_loc b_int_to_chr;
static hash_loc b_int_to_str;
static hash_loc b_missing;
static hash_loc b_newline;
static hash_loc b_num_names;
static hash_loc b_pop;
static hash_loc b_preamble;
static hash_loc b_purify;
static hash_loc b_quote;
static hash_loc b_skip;
static hash_loc b_stack;
static hash_loc b_substring;
static hash_loc b_swap;
static hash_loc b_text_length;
static hash_loc b_text_prefix;
static hash_loc b_top_stack;
static hash_loc b_type;
static hash_loc b_warning;
static hash_loc b_while;
static hash_loc b_width;
static hash_loc b_write;
static hash_loc b_default;

static str_number s_null;
static str_number s_default;
static hash_loc control_seq_loc;
static int32_t num_names;
static ASCII_code *name_sep_char;
static buf_pointer cur_token, last_token;
static buf_pointer num_text_chars;
static unsigned char /*bad_conversion */ conversion_type;
static int verbose;

const BibtexConfig* bibtex_config;

/*:473*//*12: *//*3: */

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

static void bst_err_print_and_look_for_blank_line(BstCtx* ctx)
{
    putc_log('-');
    TRY(bst_ln_num_print(ctx));
    print_bad_input_line();
    while ((bib_buf_len(BUF_TY_BASE) != 0))
        if (!input_ln(ctx->bst_file))
            longjmp(recover_jmpbuf, 1);
        else
            ctx->bst_line_num = ctx->bst_line_num + 1;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_len(BUF_TY_BASE));
}

static void print_fn_class(hash_loc fn_loc)
{
    switch ((fn_type[fn_loc])) {
    case 0:
        puts_log("built-in");
        break;
    case 1:
        puts_log("wizard-defined");
        break;
    case 2:
        puts_log("integer-literal");
        break;
    case 3:
        puts_log("string-literal");
        break;
    case 4:
        puts_log("field");
        break;
    case 5:
        puts_log("integer-entry-variable");
        break;
    case 6:
        puts_log("string-entry-variable");
        break;
    case 7:
        puts_log("integer-global-variable");
        break;
    case 8:
        puts_log("string-global-variable");
        break;
    default:
        unknwn_function_class_confusion();
        longjmp(error_jmpbuf, 1);
        break;
    }
}

/*:159*//*160: */

static void already_seen_function_print(BstCtx* ctx, hash_loc seen_fn_loc)
{
    TRY(print_a_pool_str(hash_text[seen_fn_loc]));
    puts_log(" is already a type \"");
    print_fn_class(seen_fn_loc);
    puts_log("\" function name\n");
    bst_err_print_and_look_for_blank_line(ctx);
}

static void check_field_overflow(int32_t total_fields)
{
    field_loc f_ptr;
    field_loc start_fields;
    if (total_fields > max_fields) {
        start_fields = max_fields;
        BIB_XRETALLOC("field_info", field_info, str_number, max_fields, total_fields + MAX_FIELDS);
        {
            register int32_t for_end;
            f_ptr = start_fields;
            for_end = max_fields - 1;
            if (f_ptr <= for_end)
                do {
                    field_info[f_ptr] = 0 /*missing */ ;
                }
                while (f_ptr++ < for_end);
        }
    }
}

static void nonexistent_cross_reference_error(void)
{
    puts_log("A bad cross reference-");
    TRY(bad_cross_reference_print(field_info[field_ptr]));
    puts_log("\", which doesn't exist\n");
    mark_error();
}

static void output_bbl_line(void)
{
    if (bib_buf_len(BUF_TY_OUT) != 0) {
        while ((bib_buf_len(BUF_TY_OUT) > 0))
            if (LEX_CLASS[bib_buf_at(BUF_TY_OUT, bib_buf_len(BUF_TY_OUT) - 1)] == LEX_CLASS_WHITESPACE )
                bib_set_buf_len(BUF_TY_OUT, bib_buf_len(BUF_TY_OUT) - 1);
            else
                break;

        if (bib_buf_len(BUF_TY_OUT) == 0)
            return;
        bib_set_buf_offset(BUF_TY_OUT, 1, 0);

        while (bib_buf_offset(BUF_TY_OUT, 1) < bib_buf_len(BUF_TY_OUT)) {
            ttstub_output_putc (bbl_file, bib_buf_at(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1)));
            bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) + 1);
        }
    }

    ttstub_output_putc(bbl_file, '\n');
    bbl_line_num++;
    bib_set_buf_len(BUF_TY_OUT, 0);
}

static NameAndLen start_name(str_number file_name)
{
    pool_pointer p_ptr;
    ASCIICode* name_of_file = xmalloc_array(ASCII_code, (bib_str_start(file_name + 1) - bib_str_start(file_name)) + 1);
    int32_t name_ptr = 0;
    p_ptr = bib_str_start(file_name);
    while ((p_ptr < bib_str_start(file_name + 1))) {
        name_of_file[name_ptr] = bib_str_pool(p_ptr);
        name_ptr++;
        p_ptr++;
    }
    int32_t name_length = (bib_str_start(file_name + 1) - bib_str_start(file_name));
    name_of_file[name_length] = 0;
    return (NameAndLen) { .name_of_file = name_of_file, .name_length = name_length };
}

static void add_extension(NameAndLen* nal, str_number ext)
{
    pool_pointer p_ptr;
    int32_t name_ptr = nal->name_length;
    p_ptr = bib_str_start(ext);
    while ((p_ptr < bib_str_start(ext + 1))) {
        nal->name_of_file[name_ptr] = bib_str_pool(p_ptr);
        name_ptr++;
        p_ptr++;
    }
    nal->name_length += bib_str_start(ext + 1) - bib_str_start(ext);
    nal->name_of_file[nal->name_length] = 0;
}

static str_number make_string(void)
{
    if (bib_str_ptr() == bib_max_strings()) {
        print_overflow();
        printf_log("number of strings %ld\n", (long) bib_max_strings());
        longjmp(error_jmpbuf, 1);
    }
    bib_set_str_ptr(bib_str_ptr() + 1);
    bib_set_str_start(bib_str_ptr(), bib_pool_ptr());
    return bib_str_ptr() - 1;
}

static hash_loc str_lookup(buf_type buf, buf_pointer j, buf_pointer l, str_ilk ilk, bool insert_it)
{
    int32_t h;
    hash_loc p;
    buf_pointer k;
    str_number str_num;
    {
        h = 0;
        k = j;
        while ((k < j + l)) {

            h = h + h + buf[k];
            while ((h >= hash_prime))
                h = h - hash_prime;
            k = k + 1;
        }
    }
    p = h + hash_base;
    hash_found = false;
    str_num = 0;
    while (true) {

        {
            if (hash_text[p] > 0) {

                if (bib_str_eq_buf(hash_text[p], buf, j, l)) {

                    if (hash_ilk[p] == ilk) {
                        hash_found = true;
                        return p; /* str_found */
                    } else {

                        str_num = hash_text[p];
                    }
                }
            }
        }
        if (hash_next[p] == 0 /*empty */ ) {
            if (!insert_it)
                return p; /* str_not_found */
            {
                if (hash_text[p] > 0) {
                    do {
                        if (hash_used == hash_base) {
                            print_overflow();
                            printf_log("hash size %ld\n", (long) hash_size);
                            longjmp(error_jmpbuf, 1);
                        }
                        hash_used = hash_used - 1;
                    } while (!((hash_text[hash_used] == 0)));
                    hash_next[p] = hash_used;
                    p = hash_used;
                }
                if (str_num > 0)
                    hash_text[p] = str_num;
                else {

                    {
                        while ((bib_pool_ptr() + l > bib_pool_size()))
                            pool_overflow();
                    }
                    k = j;
                    while ((k < j + l)) {

                        {
                            bib_set_str_pool(bib_pool_ptr(), buf[k]);
                            bib_set_pool_ptr(bib_pool_ptr() + 1);
                        }
                        k = k + 1;
                    }
                    hash_text[p] = make_string();
                }
                hash_ilk[p] = ilk;
            }
            return p; /* str_found */
        }
        p = hash_next[p];
    }
}

static hash_loc pre_define(pds_type pds, pds_len len, str_ilk ilk)
{
    pds_len i;
    {
        register int32_t for_end;
        i = 1;
        for_end = len;
        if (i <= for_end)
            do
                bib_buf(BUF_TY_BASE)[i] = (unsigned char) pds[i - 1];
            while (i++ < for_end);
    }
    return str_lookup(bib_buf(BUF_TY_BASE), 1, len, ilk, true);
}

static cite_number add_database_cite(cite_number new_cite)
{
    check_cite_overflow(new_cite);
    check_field_overflow(num_fields * (new_cite + 1));
    set_cite_list(new_cite, hash_text[cite_loc]);
    ilk_info[cite_loc] = new_cite;
    ilk_info[lc_cite_loc] = cite_loc;
    return new_cite + 1;
}

static bool find_cite_locs_for_this_cite_key(str_number cite_str)
{
    buf_pointer tmp_ptr;
    buf_pointer tmp_end_ptr;

    bib_set_buf_offset(BUF_TY_EX, 1, 0);
    tmp_ptr = bib_str_start(cite_str);
    tmp_end_ptr = bib_str_start(cite_str + 1);
    while ((tmp_ptr < tmp_end_ptr)) {
        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(tmp_ptr);
        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
        tmp_ptr = tmp_ptr + 1;
    }
    cite_loc = str_lookup(bib_buf(BUF_TY_EX), 0, (bib_str_start(cite_str + 1) - bib_str_start(cite_str)), 9 /*cite_ilk */ , false);
    cite_hash_found = hash_found;
    lower_case(bib_buf(BUF_TY_EX), 0, (bib_str_start(cite_str + 1) - bib_str_start(cite_str)));
    lc_cite_loc = str_lookup(bib_buf(BUF_TY_EX), 0, (bib_str_start(cite_str + 1) - bib_str_start(cite_str)), 10 /*lc_cite_ilk */ , false);
    return hash_found;
}

static bool less_than(cite_number arg1, cite_number arg2)
{
    int32_t char_ptr;
    str_ent_loc ptr1, ptr2;
    ASCII_code char1, char2;
    ptr1 = arg1 * num_ent_strs + sort_key_num;
    ptr2 = arg2 * num_ent_strs + sort_key_num;
    char_ptr = 0;
    while (true) {

        char1 = entry_strs[(ptr1) * (ent_str_size + 1) + (char_ptr)];
        char2 = entry_strs[(ptr2) * (ent_str_size + 1) + (char_ptr)];
        if (char1 == 127 /*end_of_string */ ) {

            if (char2 == 127 /*end_of_string */ ) {

                if (arg1 < arg2) {
                    return true;
                } else if (arg1 > arg2) {
                    return false;
                } else {
                    puts_log("Duplicate sort key");
                    print_confusion();
                    longjmp(error_jmpbuf, 1);
                }
            } else {
                return true;
            }
        } else if (char2 == 127 /*end_of_string */ ) {
            return false;
        } else if (char1 < char2) {
            return true;
        } else if (char1 > char2) {
            return false;
        }
        char_ptr = char_ptr + 1;
    }
}

static void build_in(pds_type pds, pds_len len, hash_loc * fn_hash_loc, blt_in_range blt_in_num)
{
    *fn_hash_loc = pre_define(pds, len, 11 /*bst_fn_ilk */ );
    fn_type[*fn_hash_loc] = 0 /*built_in */ ;
    ilk_info[*fn_hash_loc] = blt_in_num;
    ;

}

static void pre_def_certain_strings(void) {
    hash_loc pre_def_loc;
    pre_def_loc = pre_define(".aux        ", 4, 7 /*file_ext_ilk */ );
    s_aux_extension = hash_text[pre_def_loc];
    pre_def_loc = pre_define(".bbl        ", 4, 7 /*file_ext_ilk */ );
    s_bbl_extension = hash_text[pre_def_loc];
    pre_def_loc = pre_define(".blg        ", 4, 7 /*file_ext_ilk */ );
    s_log_extension = hash_text[pre_def_loc];
    pre_def_loc = pre_define("\\citation   ", 9, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_aux_citation */ ;
    pre_def_loc = pre_define("\\bibdata    ", 8, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_aux_bibdata */ ;
    pre_def_loc = pre_define("\\bibstyle   ", 9, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_aux_bibstyle */ ;
    pre_def_loc = pre_define("\\@input     ", 7, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_aux_input */ ;
    pre_def_loc = pre_define("entry       ", 5, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_bst_entry */ ;
    pre_def_loc = pre_define("execute     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_bst_execute */ ;
    pre_def_loc = pre_define("function    ", 8, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_bst_function */ ;
    pre_def_loc = pre_define("integers    ", 8, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_bst_integers */ ;
    pre_def_loc = pre_define("iterate     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 4 /*n_bst_iterate */ ;
    pre_def_loc = pre_define("macro       ", 5, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 5 /*n_bst_macro */ ;
    pre_def_loc = pre_define("read        ", 4, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 6 /*n_bst_read */ ;
    pre_def_loc = pre_define("reverse     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 7 /*n_bst_reverse */ ;
    pre_def_loc = pre_define("sort        ", 4, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 8 /*n_bst_sort */ ;
    pre_def_loc = pre_define("strings     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 9 /*n_bst_strings */ ;
    pre_def_loc = pre_define("comment     ", 7, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_bib_comment */ ;
    pre_def_loc = pre_define("preamble    ", 8, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_bib_preamble */ ;
    pre_def_loc = pre_define("string      ", 6, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_bib_string */ ;
    build_in("=           ", 1, &b_equals, 0 /*n_equals */ );
    build_in(">           ", 1, &b_greater_than, 1 /*n_greater_than */ );
    build_in("<           ", 1, &b_less_than, 2 /*n_less_than */ );
    build_in("+           ", 1, &b_plus, 3 /*n_plus */ );
    build_in("-           ", 1, &b_minus, 4 /*n_minus */ );
    build_in("*           ", 1, &b_concatenate, 5 /*n_concatenate */ );
    build_in(":=          ", 2, &b_gets, 6 /*n_gets */ );
    build_in("add.period$ ", 11, &b_add_period, 7 /*n_add_period */ );
    build_in("call.type$  ", 10, &b_call_type, 8 /*n_call_type */ );
    build_in("change.case$", 12, &b_change_case, 9 /*n_change_case */ );
    build_in("chr.to.int$ ", 11, &b_chr_to_int, 10 /*n_chr_to_int */ );
    build_in("cite$       ", 5, &b_cite, 11 /*n_cite */ );
    build_in("duplicate$  ", 10, &b_duplicate, 12 /*n_duplicate */ );
    build_in("empty$      ", 6, &b_empty, 13 /*n_empty */ );
    build_in("format.name$", 12, &b_format_name, 14 /*n_format_name */ );
    build_in("if$         ", 3, &b_if, 15 /*n_if */ );
    build_in("int.to.chr$ ", 11, &b_int_to_chr, 16 /*n_int_to_chr */ );
    build_in("int.to.str$ ", 11, &b_int_to_str, 17 /*n_int_to_str */ );
    build_in("missing$    ", 8, &b_missing, 18 /*n_missing */ );
    build_in("newline$    ", 8, &b_newline, 19 /*n_newline */ );
    build_in("num.names$  ", 10, &b_num_names, 20 /*n_num_names */ );
    build_in("pop$        ", 4, &b_pop, 21 /*n_pop */ );
    build_in("preamble$   ", 9, &b_preamble, 22 /*n_preamble */ );
    build_in("purify$     ", 7, &b_purify, 23 /*n_purify */ );
    build_in("quote$      ", 6, &b_quote, 24 /*n_quote */ );
    build_in("skip$       ", 5, &b_skip, 25 /*n_skip */ );
    build_in("stack$      ", 6, &b_stack, 26 /*n_stack */ );
    build_in("substring$  ", 10, &b_substring, 27 /*n_substring */ );
    build_in("swap$       ", 5, &b_swap, 28 /*n_swap */ );
    build_in("text.length$", 12, &b_text_length, 29 /*n_text_length */ );
    build_in("text.prefix$", 12, &b_text_prefix, 30 /*n_text_prefix */ );
    build_in("top$        ", 4, &b_top_stack, 31 /*n_top_stack */ );
    build_in("type$       ", 5, &b_type, 32 /*n_type */ );
    build_in("warning$    ", 8, &b_warning, 33 /*n_warning */ );
    build_in("while$      ", 6, &b_while, 34 /*n_while */ );
    build_in("width$      ", 6, &b_width, 35 /*n_width */ );
    build_in("write$      ", 6, &b_write, 36 /*n_write */ );
    pre_def_loc = pre_define("            ", 0, 0 /*text_ilk */ );
    s_null = hash_text[pre_def_loc];
    fn_type[pre_def_loc] = 3 /*str_literal */ ;
    pre_def_loc = pre_define("default.type", 12, 0 /*text_ilk */ );
    s_default = hash_text[pre_def_loc];
    fn_type[pre_def_loc] = 3 /*str_literal */ ;
    b_default = b_skip;
    pre_def_loc = pre_define("i           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_i */ ;
    pre_def_loc = pre_define("j           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_j */ ;
    pre_def_loc = pre_define("oe          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_oe */ ;
    pre_def_loc = pre_define("OE          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_oe_upper */ ;
    pre_def_loc = pre_define("ae          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 4 /*n_ae */ ;
    pre_def_loc = pre_define("AE          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 5 /*n_ae_upper */ ;
    pre_def_loc = pre_define("aa          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 6 /*n_aa */ ;
    pre_def_loc = pre_define("AA          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 7 /*n_aa_upper */ ;
    pre_def_loc = pre_define("o           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 8 /*n_o */ ;
    pre_def_loc = pre_define("O           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 9 /*n_o_upper */ ;
    pre_def_loc = pre_define("l           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 10 /*n_l */ ;
    pre_def_loc = pre_define("L           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 11 /*n_l_upper */ ;
    pre_def_loc = pre_define("ss          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 12 /*n_ss */ ;
    pre_def_loc = pre_define("crossref    ", 8, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 4 /*field */ ;
    ilk_info[pre_def_loc] = num_fields;
    crossref_num = num_fields;
    num_fields = num_fields + 1;
    num_pre_defined_fields = num_fields;
    pre_def_loc = pre_define("sort.key$   ", 9, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 6 /*str_entry_var */ ;
    ilk_info[pre_def_loc] = num_ent_strs;
    sort_key_num = num_ent_strs;
    num_ent_strs = num_ent_strs + 1;
    pre_def_loc = pre_define("entry.max$  ", 10, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 7 /*int_global_var */ ;
    ilk_info[pre_def_loc] = ent_str_size;
    pre_def_loc = pre_define("global.max$ ", 11, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 7 /*int_global_var */ ;
    ilk_info[pre_def_loc] = glob_str_size;
}

static bool eat_bst_white_space(BstCtx* ctx)
{
    while (true) {

        if (scan_white_space()) {

            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 37 /*comment */ ) {
                return true;
            }
        }
        if (!input_ln(ctx->bst_file)) {
            return false;
        }
        ctx->bst_line_num = ctx->bst_line_num + 1;
        bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    }
    return false;
}

static void skip_token_print(BstCtx* ctx)
{
    putc_log('-');
    TRY(bst_ln_num_print(ctx));
    mark_error();
    scan2_white(125 /*right_brace */ , 37 /*comment */);
}

static void print_recursion_illegal(BstCtx* ctx)
{
    puts_log("Curse you, wizard, before you recurse me:\n");
    puts_log("function ");
    print_a_token();
    puts_log(" is illegal in its own definition\n");
    skip_token_print(ctx);
}

static void skp_token_unknown_function_print(BstCtx* ctx)
{
    print_a_token();
    puts_log(" is an unknown function");
    skip_token_print(ctx);
}

static void skip_illegal_stuff_after_token_print(BstCtx* ctx)
{
    printf_log("\"%c\" can't follow a literal", bib_buf_at_offset(BUF_TY_BASE, 2));
    skip_token_print(ctx);
}

static void scan_fn_def(BstCtx* ctx, hash_loc fn_hash_loc)
{
    typedef int32_t fn_def_loc;
    hash_ptr2 *singl_function;
    int32_t single_fn_space;
    fn_def_loc single_ptr;
    fn_def_loc copy_ptr;
    buf_pointer end_of_num;
    hash_loc impl_fn_loc;
    single_fn_space = SINGLE_FN_SPACE;
    singl_function = XTALLOC(single_fn_space + 1, hash_ptr2);
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("function");
            bst_err_print_and_look_for_blank_line(ctx);
            goto exit;
        }
    }
    single_ptr = 0;
    while ((bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ )) {

        switch ((bib_buf_at_offset(BUF_TY_BASE, 2))) {
        case 35:
            {
                int32_t token_value = 0;
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                if (!scan_integer(&token_value)) {
                    puts_log("Illegal integer in integer literal");
                    skip_token_print(ctx);
                    goto lab25;
                };

                literal_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 1 /*integer_ilk */ , true);
                if (!hash_found) {
                    fn_type[literal_loc] = 2 /*int_literal */ ;
                    ilk_info[literal_loc] = token_value;
                }
                if ((bib_buf_offset(BUF_TY_BASE, 2) < bib_buf_len(BUF_TY_BASE)) && (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] != LEX_CLASS_WHITESPACE )
                    && (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) && (bib_buf_at_offset(BUF_TY_BASE, 2) != 37 /*comment */ )) {
                    skip_illegal_stuff_after_token_print(ctx);
                    goto lab25;
                }
                {
                    singl_function[single_ptr] = literal_loc;
                    if (single_ptr == single_fn_space) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
            }
            break;
        case 34:
            {
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                if (!scan1(34 /*double_quote */)) {
                    printf_log("No `\"' to end string literal");
                    skip_token_print(ctx);
                    goto lab25;
                };

                literal_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 0 /*text_ilk */ , true);
                fn_type[literal_loc] = 3 /*str_literal */ ;
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                if ((bib_buf_offset(BUF_TY_BASE, 2) < bib_buf_len(BUF_TY_BASE)) && (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] != LEX_CLASS_WHITESPACE )
                    && (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) && (bib_buf_at_offset(BUF_TY_BASE, 2) != 37 /*comment */ )) {
                    skip_illegal_stuff_after_token_print(ctx);
                    goto lab25;
                }
                {
                    singl_function[single_ptr] = literal_loc;
                    if (single_ptr == single_fn_space) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
            }
            break;
        case 39:
            {
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                scan2_white(125 /*right_brace */ , 37 /*comment */);

                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , false);
                if (!hash_found) {
                    skp_token_unknown_function_print(ctx);
                    goto lab25;
                } else {        /*194: */

                    if (fn_loc == wiz_loc) {
                        print_recursion_illegal(ctx);
                        goto lab25;
                    } else {

                        ;

                        {
                            singl_function[single_ptr] = quote_next_fn;
                            if (single_ptr == single_fn_space) {
                                BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                              single_fn_space + SINGLE_FN_SPACE);
                            }
                            single_ptr = single_ptr + 1;
                        }
                        {
                            singl_function[single_ptr] = fn_loc;
                            if (single_ptr == single_fn_space) {
                                BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                              single_fn_space + SINGLE_FN_SPACE);
                            }
                            single_ptr = single_ptr + 1;
                        }
                    }
                }
            }
            break;
        case 123:
            {
                bib_buf(BUF_TY_EX)[0] = 39 /*single_quote */ ;
                end_of_num = int_to_ascii(impl_fn_num, BUF_TY_EX, 1);
                impl_fn_loc = str_lookup(bib_buf(BUF_TY_EX), 0, end_of_num, 11 /*bst_fn_ilk */ , true);
                if (hash_found) {
                    puts_log("Already encountered implicit function");
                    print_confusion();
                    longjmp(error_jmpbuf, 1);
                };

                impl_fn_num = impl_fn_num + 1;
                fn_type[impl_fn_loc] = 1 /*wiz_defined */ ;
                {
                    singl_function[single_ptr] = quote_next_fn;
                    if (single_ptr == single_fn_space) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
                {
                    singl_function[single_ptr] = impl_fn_loc;
                    if (single_ptr == single_fn_space) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                scan_fn_def(ctx, impl_fn_loc);
            }
            break;
        default:
            {
                scan2_white(125 /*right_brace */ , 37 /*comment */);
                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , false);
                if (!hash_found) {
                    skp_token_unknown_function_print(ctx);
                    goto lab25;
                } else if (fn_loc == wiz_loc) {
                    print_recursion_illegal(ctx);
                    goto lab25;
                } else {

                    ;

                    {
                        singl_function[single_ptr] = fn_loc;
                        if (single_ptr == single_fn_space) {
                            BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                          single_fn_space + SINGLE_FN_SPACE);
                        }
                        single_ptr = single_ptr + 1;
                    }
                }
            }
            break;
        }
 lab25:                        /*next_token */  {

            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                goto exit;
            }
        }
    }
    {
        {
            singl_function[single_ptr] = end_of_def;
            if (single_ptr == single_fn_space) {
                BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                              single_fn_space + SINGLE_FN_SPACE);
            }
            single_ptr = single_ptr + 1;
        }
        while ((single_ptr + wiz_def_ptr > wiz_fn_space)) {

            BIB_XRETALLOC("wiz_functions", wiz_functions, hash_ptr2, wiz_fn_space, wiz_fn_space + WIZ_FN_SPACE);
        }
        ilk_info[fn_hash_loc] = wiz_def_ptr;
        copy_ptr = 0;
        while ((copy_ptr < single_ptr)) {

            wiz_functions[wiz_def_ptr] = singl_function[copy_ptr];
            copy_ptr = copy_ptr + 1;
            wiz_def_ptr = wiz_def_ptr + 1;
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
 exit: free(singl_function);
}

static bool eat_bib_white_space(void)
{
    while ((!scan_white_space())) {

        if (!input_ln(cur_bib_file())) {
            return false;
        }
        set_bib_line_num(bib_line_num() + 1);
        bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    }
    return true;
}

static bool compress_bib_white(bool at_bib_command)
{
    {
        if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size()) {
            ttstub_fprintf(bib_log_file(), "Field filled up at ' ', reallocating.\n");
            buffer_overflow();
        }

        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 32 /*space */ ;
        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
    }
    while ((!scan_white_space())) {

        if (!input_ln(cur_bib_file())) {
            TRY(eat_bib_print(at_bib_command));
            return false;
        }
        set_bib_line_num(bib_line_num() + 1);
        bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    }
    return true;
}

static bool scan_balanced_braces(bool at_bib_command)
{
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    {
        if ((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE))) {

            if (!compress_bib_white(at_bib_command)) {
                return false;
            }
        }
    }
    if (bib_buf_offset(BUF_TY_EX, 1) > 1) {

        if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] == 32 /*space */ ) {

            if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 2] == 32 /*space */ )
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
        }
    }
    bib_brace_level = 0;
    if (store_field) {        /*257: */
        while ((bib_buf_at_offset(BUF_TY_BASE, 2) != right_str_delim))
            switch ((bib_buf_at_offset(BUF_TY_BASE, 2))) {
            case 123: /*'{'*/
                {
                    bib_brace_level = bib_brace_level + 1;
                    {
                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                            ttstub_fprintf(bib_log_file(), "Field filled up at '{', reallocating.\n");
                            buffer_overflow();
                        }

                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 123 /*left_brace */ ;
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                    }
                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                    {
                        if ((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE))) {

                            if (!compress_bib_white(at_bib_command))
                                return false;
                        }
                    }
                    {
                        while (true)
                            switch ((bib_buf_at_offset(BUF_TY_BASE, 2))) {
                            case 125: /*'}'*/
                                {
                                    bib_brace_level = bib_brace_level - 1;
                                    {
                                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                            ttstub_fprintf(bib_log_file(), "Field filled up at '}', reallocating.\n");
                                            buffer_overflow();
                                        }

                                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 125 /*right_brace */ ;
                                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    }
                                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                                    {
                                        if (((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE )
                                             || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE)))) {

                                            if (!compress_bib_white(at_bib_command))
                                                return false;
                                        }
                                    }
                                    if (bib_brace_level == 0)
                                        goto loop_exit;
                                }
                                break;
                            case 123: /*'{'*/
                                {
                                    bib_brace_level = bib_brace_level + 1;
                                    {
                                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                            ttstub_fprintf(bib_log_file(), "Field filled up at '{', reallocating.\n");
                                            buffer_overflow();
                                        }

                                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 123 /*left_brace */ ;
                                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    }
                                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                                    {
                                        if (((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE )
                                             || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE)))) {

                                            if (!compress_bib_white(at_bib_command))
                                                return false;
                                        }
                                    }
                                }
                                break;
                            default:
                                {
                                    {
                                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                            ttstub_fprintf(bib_log_file(), "Field filled up at %ld, reallocating.\n", (long) bib_buf_at_offset(BUF_TY_BASE, 2));
                                            buffer_overflow();
                                        }

                                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at_offset(BUF_TY_BASE, 2);
                                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    }
                                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                                    {
                                        if (((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE )
                                             || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE)))) {

                                            if (!compress_bib_white(at_bib_command))
                                                return false;
                                        }
                                    }
                                }
                                break;
                            }
                    loop_exit:
                        ;
                    }
                }
                break;
            case 125:
                {
                    TRY(bib_unbalanced_braces_print(at_bib_command));
                    return false;
                }
                break;
            default:
                {
                    {
                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                            ttstub_fprintf(bib_log_file(), "Field filled up at %ld, reallocating.\n", (long) bib_buf_at_offset(BUF_TY_BASE, 2));
                            buffer_overflow();
                        }

                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at_offset(BUF_TY_BASE, 2);
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                    }
                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                    {
                        if ((LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) || (bib_buf_offset(BUF_TY_BASE, 2) == bib_buf_len(BUF_TY_BASE))) {

                            if (!compress_bib_white(at_bib_command))
                                return false;
                        }
                    }
                }
                break;
            }
    } else {                    /*255: */

        while ((bib_buf_at_offset(BUF_TY_BASE, 2) != right_str_delim))
            if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ ) {
                bib_brace_level = bib_brace_level + 1;
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                {
                    if (!eat_bib_white_space()) {
                        TRY(eat_bib_print(at_bib_command));
                        return false;
                    }
                }
                while ((bib_brace_level > 0)) { /*256: */

                    if (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ ) {
                        bib_brace_level = bib_brace_level - 1;
                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                        {
                            if (!eat_bib_white_space()) {
                                TRY(eat_bib_print(at_bib_command));
                                return false;
                            }
                        }
                    } else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ ) {
                        bib_brace_level = bib_brace_level + 1;
                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                        {
                            if (!eat_bib_white_space()) {
                                TRY(eat_bib_print(at_bib_command));
                                return false;
                            }
                        }
                    } else {

                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                        if (!scan2(125 /*right_brace */ , 123 /*left_brace */)) {
                            if (!eat_bib_white_space()) {
                                TRY(eat_bib_print(at_bib_command));
                                return false;
                            }
                        }
                    }
                }
            } else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ ) {
                TRY(bib_unbalanced_braces_print(at_bib_command));
                return false;
            } else {

                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                if (!scan3(right_str_delim, 123 /*left_brace */ , 125 /*right_brace */)) {
                    if (!eat_bib_white_space()) {
                        TRY(eat_bib_print(at_bib_command));
                        return false;
                    }
                }
            }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    return true;
}

static bool scan_a_field_token_and_eat_white(bool at_bib_command)
{
    buf_pointer tmp_ptr, tmp_end_ptr;

    switch ((bib_buf_at_offset(BUF_TY_BASE, 2))) {
    case 123:
        {
            right_str_delim = 125 /*right_brace */ ;
            if (!scan_balanced_braces(at_bib_command))
                return false;
        }
        break;
    case 34:
        {
            right_str_delim = 34 /*double_quote */ ;
            if (!scan_balanced_braces(at_bib_command))
                return false;
        }
        break;
    case 48:
    case 49:
    case 50:
    case 51:
    case 52:
    case 53:
    case 54:
    case 55:
    case 56:
    case 57:
        {
            if (!scan_nonneg_integer()) {
                puts_log("A digit disappeared");
                print_confusion();
                longjmp(error_jmpbuf, 1);
            }
            if (store_field) {
                tmp_ptr = bib_buf_offset(BUF_TY_BASE, 1);
                while ((tmp_ptr < bib_buf_offset(BUF_TY_BASE, 2))) {

                    {
                        if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                            ttstub_fprintf(bib_log_file(), "Field filled up at %ld, reallocating.\n", (long) bib_buf_at(BUF_TY_BASE, tmp_ptr));
                            buffer_overflow();
                        }

                        bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at(BUF_TY_BASE, tmp_ptr);
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                    }
                    tmp_ptr = tmp_ptr + 1;
                }
            }
        }
        break;
    default:
        {
            ScanRes scan_result = scan_identifier(44 /*comma */ , right_outer_delim, 35 /*concat_char */);
            {
                if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
                else {
                    TRY(bib_id_print(scan_result));
                    puts_log("a field part");
                    TRY(bib_err_print(at_bib_command));
                    return false;
                }
            }
            if (store_field) {
                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                macro_name_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 13 /*macro_ilk */ , false);
                store_token = true;
                if (at_bib_command) {

                    if (command_num == 2 /*n_bib_string */ ) {

                        if (macro_name_loc == cur_macro_loc) {
                            store_token = false;
                            macro_warn_print();
                            puts_log("used in its own definition\n");
                            TRY(bib_warn_print());
                        }
                    }
                }
                if (!hash_found) {
                    store_token = false;
                    macro_warn_print();
                    puts_log("undefined\n");
                    TRY(bib_warn_print());
                }
                if (store_token) {    /*261: */
                    tmp_ptr = bib_str_start(ilk_info[macro_name_loc]);
                    tmp_end_ptr = bib_str_start(ilk_info[macro_name_loc] + 1);
                    if (bib_buf_offset(BUF_TY_EX, 1) == 0) {

                        if ((tmp_ptr < tmp_end_ptr) && (LEX_CLASS[bib_str_pool(tmp_ptr)] == LEX_CLASS_WHITESPACE )) {
                            {
                                if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                    ttstub_fprintf(bib_log_file(), "Field filled up at ' ', reallocating.\n");
                                    buffer_overflow();
                                }

                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 32 /*space */ ;
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            }
                            tmp_ptr = tmp_ptr + 1;
                            while ((tmp_ptr < tmp_end_ptr) && (LEX_CLASS[bib_str_pool(tmp_ptr)] == LEX_CLASS_WHITESPACE ))
                                tmp_ptr = tmp_ptr + 1;
                        }
                    }
                    while ((tmp_ptr < tmp_end_ptr)) {

                        if (LEX_CLASS[bib_str_pool(tmp_ptr)] != LEX_CLASS_WHITESPACE ) {
                            if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                ttstub_fprintf(bib_log_file(), "Field filled up at %ld, reallocating.\n", (long) bib_str_pool(tmp_ptr));
                                buffer_overflow();
                            }

                            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(tmp_ptr);
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                        } else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] != 32 /*space */ ) {
                            if (bib_buf_offset(BUF_TY_EX, 1) >= bib_buf_size()) {
                                ttstub_fprintf(bib_log_file(), "Field filled up at ' ', reallocating.\n");
                                buffer_overflow();
                            }

                            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 32 /*space */ ;
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                        }
                        tmp_ptr = tmp_ptr + 1;
                    }
                }
            }
        }
        break;
    }
    {
        if (!eat_bib_white_space()) {
            TRY(eat_bib_print(at_bib_command));
            return false;
        }
    }
    return true;
}

static bool scan_and_store_the_field_value_and_eat_white(bool at_bib_command)
{
    buf_pointer tmp_ptr;

    bib_set_buf_offset(BUF_TY_EX, 1, 0);
    if (!scan_a_field_token_and_eat_white(at_bib_command))
        return false;
    while (bib_buf_at_offset(BUF_TY_BASE, 2) == 35 /*concat_char */ ) {

        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        {
            if (!eat_bib_white_space()) {
                TRY(eat_bib_print(at_bib_command));
                return false;
            }
        }
        if (!scan_a_field_token_and_eat_white(at_bib_command))
            return false;
    }
    if (store_field) {        /*262: */
        if (!at_bib_command) {

            if (bib_buf_offset(BUF_TY_EX, 1) > 0) {

                if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] == 32 /*space */ )
                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
            }
        }
        buf_pointer ex_buf_xptr = 0;
        if ((!at_bib_command) && (bib_buf(BUF_TY_EX)[0] == 32 /*space */ ) && (bib_buf_offset(BUF_TY_EX, 1) > 0))
            ex_buf_xptr = 1;
        else
            ex_buf_xptr = 0;
        field_val_loc = str_lookup(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr, 0 /*text_ilk */ , true);
        fn_type[field_val_loc] = 3 /*str_literal */ ;
        ;

        if (at_bib_command) { /*263: */
            switch ((command_num)) {
            case 1:
                add_preamble(hash_text[field_val_loc]);
                break;
            case 2:
                ilk_info[cur_macro_loc] = hash_text[field_val_loc];
                break;
            default:
                bib_cmd_confusion();
                longjmp(error_jmpbuf, 1);
                break;
            }
        } else {                /*264: */

            field_ptr = entry_cite_ptr * num_fields + ilk_info[field_name_loc];
            if (field_ptr >= max_fields) {
                puts_log("field_info index is out of range");
                print_confusion();
                longjmp(error_jmpbuf, 1);
            }
            if (field_info[field_ptr] != 0 /*missing */ ) {
                puts_log("Warning--I'm ignoring ");
                TRY(print_a_pool_str(cite_list(entry_cite_ptr)));
                puts_log("'s extra \"");
                TRY(print_a_pool_str(hash_text[field_name_loc]));
                puts_log("\" field\n");
                TRY(bib_warn_print());
            } else {

                field_info[field_ptr] = hash_text[field_val_loc];
                if ((ilk_info[field_name_loc] == crossref_num) && (!all_entries)) {   /*265: */
                    tmp_ptr = ex_buf_xptr;
                    while (tmp_ptr < bib_buf_offset(BUF_TY_EX, 1)) {
                        bib_buf(BUF_TY_OUT)[tmp_ptr] = bib_buf(BUF_TY_EX)[tmp_ptr];
                        tmp_ptr = tmp_ptr + 1;
                    }
                    lower_case(bib_buf(BUF_TY_OUT), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                    lc_cite_loc =
                        str_lookup(bib_buf(BUF_TY_OUT), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr, 10 /*lc_cite_ilk */ , true);
                    if (hash_found) {
                        cite_loc = ilk_info[lc_cite_loc];
                        if (ilk_info[cite_loc] >= old_num_cites)
                            set_cite_info(ilk_info[cite_loc], cite_info(ilk_info[cite_loc]) + 1);
                    } else {

                        cite_loc = str_lookup(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr, 9 /*cite_ilk */ , true);
                        if (hash_found) {
                            hash_cite_confusion();
                            longjmp(error_jmpbuf, 1);
                        }
                        set_cite_ptr(add_database_cite(cite_ptr()));
                        set_cite_info(ilk_info[cite_loc], 1);
                    }
                }
            }
        }
    }
    return true;
}

static void decr_brace_level(ExecCtx* ctx, str_number pop_lit_var)
{
    if (brace_level == 0)
        TRY(braces_unbalanced_complaint(ctx, pop_lit_var));
    else
        brace_level = brace_level - 1;
}

static void check_brace_level(ExecCtx* ctx, str_number pop_lit_var)
{
    if (brace_level > 0)
        TRY(braces_unbalanced_complaint(ctx, pop_lit_var));
}

static void name_scan_for_and(ExecCtx* ctx, str_number pop_lit_var)
{
    brace_level = 0;
    bool preceding_white = false;
    bool and_found = false;
    while ((!and_found) && (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)))
        switch ((bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)])) {
        case 97:
        case 65:
            {
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                if (preceding_white) {        /*387: */
                    if (bib_buf_offset(BUF_TY_EX, 1) <= (bib_buf_len(BUF_TY_EX) - 3)) {

                        if ((bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 'n' ) || (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 'N' )) {

                            if ((bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 1] == 'd' ) || (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 1] == 'D' )) {

                                if (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 2]] == LEX_CLASS_WHITESPACE ) {
                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 2);
                                    and_found = true;
                                }
                            }
                        }
                    }
                }
                preceding_white = false;
            }
            break;
        case 123:
            {
                brace_level = brace_level + 1;
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                while ((brace_level > 0) && (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))) {

                    if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ )
                        brace_level = brace_level - 1;
                    else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ )
                        brace_level = brace_level + 1;
                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                }
                preceding_white = false;
            }
            break;
        case 125:
            {
                decr_brace_level(ctx, pop_lit_var);
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                preceding_white = false;
            }
            break;
        default:
            if (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_WHITESPACE ) {
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                preceding_white = true;
            } else {

                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                preceding_white = false;
            }
            break;
        }
    check_brace_level(ctx, pop_lit_var);
}

static bool von_token_found(buf_pointer* name_bf_ptr, buf_pointer name_bf_xptr)
{
    int32_t nm_brace_level = 0;
    while (*name_bf_ptr < name_bf_xptr)
        if ((bib_buf_at(BUF_TY_SV, *name_bf_ptr) >= 'A' ) && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) <= 'Z' ))
            return false;
        else if ((bib_buf_at(BUF_TY_SV, *name_bf_ptr) >= 'a' ) && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) <= 'z' )) {
            return true;
        } else if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 123 /*left_brace */ ) {
            nm_brace_level = nm_brace_level + 1;
            *name_bf_ptr = *name_bf_ptr + 1;
            if ((*name_bf_ptr + 2 < name_bf_xptr) && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 92 /*backslash */ )) { /*399: */
                *name_bf_ptr = *name_bf_ptr + 1;
                buf_pointer name_bf_yptr = *name_bf_ptr;
                while ((*name_bf_ptr < name_bf_xptr) && (LEX_CLASS[bib_buf_at(BUF_TY_SV, *name_bf_ptr)] == LEX_CLASS_ALPHA ))
                    *name_bf_ptr = *name_bf_ptr + 1;
                control_seq_loc =
                    str_lookup(bib_buf(BUF_TY_SV), name_bf_yptr, *name_bf_ptr - name_bf_yptr, 14 /*control_seq_ilk */ , false);
                if (hash_found) {     /*400: */
                    switch ((ilk_info[control_seq_loc])) {
                    case 3:
                    case 5:
                    case 7:
                    case 9:
                    case 11:
                        return false;
                    case 0:
                    case 1:
                    case 2:
                    case 4:
                    case 6:
                    case 8:
                    case 10:
                    case 12:
                        return true;
                    default:
                        puts_log("Control-sequence hash error");
                        print_confusion();
                        longjmp(error_jmpbuf, 1);
                        break;
                    }
                }
                while ((*name_bf_ptr < name_bf_xptr) && (nm_brace_level > 0)) {

                    if ((bib_buf_at(BUF_TY_SV, *name_bf_ptr) >= 'A' ) && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) <= 'Z' ))
                        return false;
                    else if ((bib_buf_at(BUF_TY_SV, *name_bf_ptr) >= 'a' ) && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) <= 'z' )) {
                        return true;
                    } else if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 125 /*right_brace */ )
                        nm_brace_level = nm_brace_level - 1;
                    else if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 123 /*left_brace */ )
                        nm_brace_level = nm_brace_level + 1;
                    *name_bf_ptr = *name_bf_ptr + 1;
                }
                return false;
            } else /*401: */
                while ((nm_brace_level > 0) && (*name_bf_ptr < name_bf_xptr)) {

                    if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 125 /*right_brace */ )
                        nm_brace_level = nm_brace_level - 1;
                    else if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 123 /*left_brace */ )
                        nm_brace_level = nm_brace_level + 1;
                    *name_bf_ptr = *name_bf_ptr + 1;
                }
        } else
            *name_bf_ptr = *name_bf_ptr + 1;
    return false;
}

static void von_name_ends_and_last_name_starts_stuff(buf_pointer last_end, buf_pointer von_start, buf_pointer* von_end, buf_pointer* name_bf_ptr, buf_pointer* name_bf_xptr)
{
    *von_end = last_end - 1;
    while (*von_end > von_start) {

        *name_bf_ptr = bib_buf_at(BUF_TY_NAME_TOK, *von_end - 1);
        *name_bf_xptr = bib_buf_at(BUF_TY_NAME_TOK, *von_end);
        if (von_token_found(name_bf_ptr, *name_bf_xptr))
            return;
        *von_end = *von_end - 1;
    }
}

static pool_pointer skip_stuff_at_sp_brace_level_greater_than_one(pool_pointer sp_ptr, pool_pointer sp_end, int32_t* sp_brace_level)
{
    while ((*sp_brace_level > 1) && (sp_ptr < sp_end)) {

        if (bib_str_pool(sp_ptr) == 125 /*right_brace */ )
            *sp_brace_level = *sp_brace_level - 1;
        else if (bib_str_pool(sp_ptr) == 123 /*left_brace */ )
            *sp_brace_level = *sp_brace_level + 1;
        sp_ptr = sp_ptr + 1;
    }
    return sp_ptr;
}

static void brace_lvl_one_letters_complaint(ExecCtx* ctx)
{
    puts_log("The format string \"");
    TRY(print_a_pool_str(ctx->pop1.lit));
    puts_log("\" has an illegal brace-level-1 letter");
    TRY(bst_ex_warn_print(ctx));
}

static bool enough_text_chars(buf_pointer enough_chars, buf_pointer ex_buf_xptr)
{
    num_text_chars = 0;
    buf_pointer ex_buf_yptr = ex_buf_xptr;
    while ((ex_buf_yptr < bib_buf_offset(BUF_TY_EX, 1)) && (num_text_chars < enough_chars)) {

        ex_buf_yptr = ex_buf_yptr + 1;
        if (bib_buf(BUF_TY_EX)[ex_buf_yptr - 1] == 123 /*left_brace */ ) {
            brace_level = brace_level + 1;
            if ((brace_level == 1) && (ex_buf_yptr < bib_buf_offset(BUF_TY_EX, 1))) {

                if (bib_buf(BUF_TY_EX)[ex_buf_yptr] == 92 /*backslash */ ) {
                    ex_buf_yptr = ex_buf_yptr + 1;
                    while ((ex_buf_yptr < bib_buf_offset(BUF_TY_EX, 1)) && (brace_level > 0)) {

                        if (bib_buf(BUF_TY_EX)[ex_buf_yptr] == 125 /*right_brace */ )
                            brace_level = brace_level - 1;
                        else if (bib_buf(BUF_TY_EX)[ex_buf_yptr] == 123 /*left_brace */ )
                            brace_level = brace_level + 1;
                        ex_buf_yptr = ex_buf_yptr + 1;
                    }
                }
            }
        } else if (bib_buf(BUF_TY_EX)[ex_buf_yptr - 1] == 125 /*right_brace */ )
            brace_level = brace_level - 1;
        num_text_chars = num_text_chars + 1;
    }
    return num_text_chars >= enough_chars;
}

static void figure_out_the_formatted_name(
        ExecCtx* ctx,
        buf_pointer first_start,
        buf_pointer first_end,
        buf_pointer last_end,
        buf_pointer von_start,
        buf_pointer von_end,
        buf_pointer* name_bf_ptr,
        buf_pointer* name_bf_xptr,
        buf_pointer jr_end
) {
    pool_pointer sp_xptr1 = 0;
    bib_set_buf_offset(BUF_TY_EX, 1, 0);
    int32_t sp_brace_level = 0;
    pool_pointer sp_ptr = bib_str_start(ctx->pop1.lit);
    pool_pointer sp_end = bib_str_start(ctx->pop1.lit + 1);
    while (sp_ptr < sp_end)
        if (bib_str_pool(sp_ptr) == 123 /*left_brace */ ) {
            sp_brace_level = sp_brace_level + 1;
            sp_ptr = sp_ptr + 1;
            sp_xptr1 = sp_ptr;
            bool alpha_found = false;
            bool double_letter = false;
            bool end_of_group = false;
            bool to_be_written = true;
            while ((!end_of_group) && (sp_ptr < sp_end))
                if (LEX_CLASS[bib_str_pool(sp_ptr)] == LEX_CLASS_ALPHA ) {
                    sp_ptr = sp_ptr + 1;
                    if (alpha_found) {
                        brace_lvl_one_letters_complaint(ctx);
                        to_be_written = false;
                    } else {

                        switch ((bib_str_pool(sp_ptr - 1))) {
                        case 102:
                        case 70:
                            {
                                cur_token = first_start;
                                last_token = first_end;
                                if (cur_token == last_token)
                                    to_be_written = false;
                                if (((bib_str_pool(sp_ptr) == 'f' )
                                     || (bib_str_pool(sp_ptr) == 'F' )))
                                    double_letter = true;
                            }
                            break;
                        case 118:
                        case 86:
                            {
                                cur_token = von_start;
                                last_token = von_end;
                                if (cur_token == last_token)
                                    to_be_written = false;
                                if (((bib_str_pool(sp_ptr) == 'v' )
                                     || (bib_str_pool(sp_ptr) == 'V' )))
                                    double_letter = true;
                            }
                            break;
                        case 108:
                        case 76:
                            {
                                cur_token = von_end;
                                last_token = last_end;
                                if (cur_token == last_token)
                                    to_be_written = false;
                                if (((bib_str_pool(sp_ptr) == 'l' )
                                     || (bib_str_pool(sp_ptr) == 'L' )))
                                    double_letter = true;
                            }
                            break;
                        case 106:
                        case 74:
                            {
                                cur_token = last_end;
                                last_token = jr_end;
                                if (cur_token == last_token)
                                    to_be_written = false;
                                if (((bib_str_pool(sp_ptr) == 'j' )
                                     || (bib_str_pool(sp_ptr) == 'J' )))
                                    double_letter = true;
                            }
                            break;
                        default:
                            {
                                brace_lvl_one_letters_complaint(ctx);
                                to_be_written = false;
                            }
                            break;
                        }
                        if (double_letter)
                            sp_ptr = sp_ptr + 1;
                    }
                    alpha_found = true;
                } else if (bib_str_pool(sp_ptr) == 125 /*right_brace */ ) {
                    sp_brace_level = sp_brace_level - 1;
                    sp_ptr = sp_ptr + 1;
                    end_of_group = true;
                } else if (bib_str_pool(sp_ptr) == 123 /*left_brace */ ) {
                    sp_brace_level = sp_brace_level + 1;
                    sp_ptr = skip_stuff_at_sp_brace_level_greater_than_one(sp_ptr + 1, sp_end, &sp_brace_level);
                } else
                    sp_ptr = sp_ptr + 1;
            if ((end_of_group) && (to_be_written)) {  /*412: */
                buf_pointer ex_buf_xptr = bib_buf_offset(BUF_TY_EX, 1);
                sp_ptr = sp_xptr1;
                sp_brace_level = 1;
                while (sp_brace_level > 0)
                    if ((LEX_CLASS[bib_str_pool(sp_ptr)] == LEX_CLASS_ALPHA ) && (sp_brace_level == 1)) {
                        sp_ptr = sp_ptr + 1;
                        {
                            if (double_letter)
                                sp_ptr = sp_ptr + 1;
                            bool use_default = true;
                            pool_pointer sp_xptr2 = sp_ptr;
                            if (bib_str_pool(sp_ptr) == 123 /*left_brace */ ) {
                                use_default = false;
                                sp_brace_level = sp_brace_level + 1;
                                sp_ptr = sp_ptr + 1;
                                sp_xptr1 = sp_ptr;
                                sp_ptr = skip_stuff_at_sp_brace_level_greater_than_one(sp_ptr, sp_end, &sp_brace_level);
                                sp_xptr2 = sp_ptr - 1;
                            }
                            while (cur_token < last_token) {

                                if (double_letter) {  /*415: */
                                    *name_bf_ptr = bib_buf_at(BUF_TY_NAME_TOK, cur_token);
                                    *name_bf_xptr = bib_buf_at(BUF_TY_NAME_TOK, cur_token + 1);
                                    if (bib_buf_len(BUF_TY_EX) + (*name_bf_xptr - *name_bf_ptr) > bib_buf_size())
                                        buffer_overflow();
                                    while (*name_bf_ptr < *name_bf_xptr) {

                                        {
                                            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at(BUF_TY_SV, *name_bf_ptr);
                                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                        }
                                        *name_bf_ptr = *name_bf_ptr + 1;
                                    }
                                } else {        /*416: */

                                    *name_bf_ptr = bib_buf_at(BUF_TY_NAME_TOK, cur_token);
                                    *name_bf_xptr = bib_buf_at(BUF_TY_NAME_TOK, cur_token + 1);
                                    while (*name_bf_ptr < *name_bf_xptr) {

                                        if (LEX_CLASS[bib_buf_at(BUF_TY_SV, *name_bf_ptr)] == LEX_CLASS_ALPHA ) {
                                            {
                                                if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                    buffer_overflow();
                                                {
                                                    bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at(BUF_TY_SV, *name_bf_ptr);
                                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                                }
                                            }
                                            goto loop_exit;
                                        } else if ((*name_bf_ptr + 1 < *name_bf_xptr)
                                                && (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 123 /*left_brace */ )) {

                                            if (bib_buf_at(BUF_TY_SV, *name_bf_ptr + 1) == 92 /*backslash */ ) {   /*417: */
                                                if (bib_buf_offset(BUF_TY_EX, 1) + 2 > bib_buf_size())
                                                    buffer_overflow();
                                                {
                                                    bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 123 /*left_brace */ ;
                                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                                }
                                                {
                                                    bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 92 /*backslash */ ;
                                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                                }
                                                *name_bf_ptr = *name_bf_ptr + 2;
                                                int32_t nm_brace_level = 1;
                                                while ((*name_bf_ptr < *name_bf_xptr) && (nm_brace_level > 0)) {

                                                    if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 125 /*right_brace */ )
                                                        nm_brace_level = nm_brace_level - 1;
                                                    else if (bib_buf_at(BUF_TY_SV, *name_bf_ptr) == 123 /*left_brace */ )
                                                        nm_brace_level = nm_brace_level + 1;
                                                    {
                                                        if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                            buffer_overflow();
                                                        {
                                                            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_buf_at(BUF_TY_SV, *name_bf_ptr);
                                                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                                        }
                                                    }
                                                    *name_bf_ptr = *name_bf_ptr + 1;
                                                }
                                                goto loop_exit;
                                            }
                                        }
                                        *name_bf_ptr = *name_bf_ptr + 1;
                                    }
                                loop_exit:
                                    ;
                                }
                                cur_token = cur_token + 1;
                                if (cur_token < last_token) { /*418: */
                                    if (use_default) {
                                        if (!double_letter) {
                                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                buffer_overflow();
                                            {
                                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 46 /*period */ ;
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            }
                                        }
                                        if (LEX_CLASS[name_sep_char[cur_token]] == LEX_CLASS_SEP ) {
                                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                buffer_overflow();
                                            {
                                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = name_sep_char[cur_token];
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            }
                                        } else
                                            if (((cur_token == last_token - 1)
                                                 || (!enough_text_chars(3 /*long_token */, ex_buf_xptr)))) {
                                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                buffer_overflow();
                                            {
                                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 126 /*tie */ ;
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            }
                                        } else {

                                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                                buffer_overflow();
                                            {
                                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 32 /*space */ ;
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            }
                                        }
                                    } else {

                                        if (bib_buf_len(BUF_TY_EX) + (sp_xptr2 - sp_xptr1) > bib_buf_size())
                                            buffer_overflow();
                                        sp_ptr = sp_xptr1;
                                        while (sp_ptr < sp_xptr2) {

                                            {
                                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(sp_ptr);
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            }
                                            sp_ptr = sp_ptr + 1;
                                        }
                                    }
                                }
                            }
                            if (!use_default)
                                sp_ptr = sp_xptr2 + 1;
                        }
                    } else if (bib_str_pool(sp_ptr) == 125 /*right_brace */ ) {
                        sp_brace_level = sp_brace_level - 1;
                        sp_ptr = sp_ptr + 1;
                        if (sp_brace_level > 0) {
                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                buffer_overflow();
                            {
                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 125 /*right_brace */ ;
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            }
                        }
                    } else if (bib_str_pool(sp_ptr) == 123 /*left_brace */ ) {
                        sp_brace_level = sp_brace_level + 1;
                        sp_ptr = sp_ptr + 1;
                        {
                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                buffer_overflow();
                            {
                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 123 /*left_brace */ ;
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            }
                        }
                    } else {

                        {
                            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                                buffer_overflow();
                            {
                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(sp_ptr);
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            }
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                if (bib_buf_offset(BUF_TY_EX, 1) > 0) {

                    if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] == 126 /*tie */ ) {    /*420: */
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                        if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] == 126 /*tie */ ) ;
                        else if (!enough_text_chars(3 /*long_name */, ex_buf_xptr))
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                        else {

                            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = 32 /*space */ ;
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                        }
                    }
                }
            }
        } else if (bib_str_pool(sp_ptr) == 125 /*right_brace */ ) {
            TRY(braces_unbalanced_complaint(ctx, ctx->pop1.lit));
            sp_ptr = sp_ptr + 1;
        } else {
            if (bib_buf_offset(BUF_TY_EX, 1) == bib_buf_size())
                buffer_overflow();
            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(sp_ptr);
            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
            sp_ptr = sp_ptr + 1;
        }
    if (sp_brace_level > 0)
        TRY(braces_unbalanced_complaint(ctx, ctx->pop1.lit));
    bib_set_buf_len(BUF_TY_EX, bib_buf_offset(BUF_TY_EX, 1));
}

static void pop_top_and_print(ExecCtx* ctx)
{
    ExecVal val;
    pop_lit_stk(ctx, &val);
    if (val.typ == STK_TYPE_ILLEGAL) {
        puts_log("Empty literal\n");
    } else {
        TRY(print_lit(hash_text, val));
    }
}

static void pop_whole_stack(ExecCtx* ctx)
{
    while (ctx->lit_stk_ptr > 0)
        pop_top_and_print(ctx);
}

static void init_command_execution(ExecCtx* ctx)
{
    ctx->lit_stk_ptr = 0;
    ctx->bib_str_ptr = bib_str_ptr();
}

static void check_command_execution(ExecCtx* ctx)
{
    if (ctx->lit_stk_ptr != 0) {
        printf_log("ptr=%ld, stack=\n", (long) ctx->lit_stk_ptr);
        pop_whole_stack(ctx);
        puts_log("---the literal stack isn't empty");
        TRY(bst_ex_warn_print(ctx));
    }
    if (ctx->bib_str_ptr != bib_str_ptr()) {
        puts_log("Nonempty empty string stack");
        print_confusion();
        longjmp(error_jmpbuf, 1);
    }
}

static void add_pool_buf_and_push(ExecCtx* ctx)
{
    {
        while (bib_pool_ptr() + bib_buf_len(BUF_TY_EX) > bib_pool_size())
            pool_overflow();
    }
    bib_set_buf_offset(BUF_TY_EX, 1, 0);
    while (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) {

        {
            bib_set_str_pool(bib_pool_ptr(), bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]);
            bib_set_pool_ptr(bib_pool_ptr() + 1);
        }
        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
    }
    push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING} );
}

static void add_buf_pool(str_number p_str)
{
    pool_pointer p_ptr1 = bib_str_start(p_str);
    pool_pointer p_ptr2 = bib_str_start(p_str + 1);
    if (bib_buf_len(BUF_TY_EX) + (p_ptr2 - p_ptr1) > bib_buf_size())
        buffer_overflow();
    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_len(BUF_TY_EX));
    while (p_ptr1 < p_ptr2) {

        {
            bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(p_ptr1);
            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
        }
        p_ptr1 = p_ptr1 + 1;
    }
    bib_set_buf_len(BUF_TY_EX, bib_buf_offset(BUF_TY_EX, 1));
}

static void add_out_pool(str_number p_str)
{
    buf_pointer tmp_ptr;
    buf_pointer break_ptr;
    buf_pointer end_ptr;

    bool break_pt_found;
    bool unbreakable_tail;
    pool_pointer p_ptr1 = bib_str_start(p_str);
    pool_pointer p_ptr2 = bib_str_start(p_str + 1);
    while (bib_buf_len(BUF_TY_OUT) + (p_ptr2 - p_ptr1) > bib_buf_size())
        buffer_overflow();
    bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_len(BUF_TY_OUT));
    while (p_ptr1 < p_ptr2) {

        bib_buf(BUF_TY_OUT)[bib_buf_offset(BUF_TY_OUT, 1)] = bib_str_pool(p_ptr1);
        p_ptr1 = p_ptr1 + 1;
        bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) + 1);
    }
    bib_set_buf_len(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1));
    unbreakable_tail = false;
    while ((bib_buf_len(BUF_TY_OUT) > max_print_line) && (!unbreakable_tail)) {        /*324: */

        end_ptr = bib_buf_len(BUF_TY_OUT);
        bib_set_buf_offset(BUF_TY_OUT, 1, max_print_line);
        break_pt_found = false;
        while ((LEX_CLASS[bib_buf_at(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1))] != LEX_CLASS_WHITESPACE ) && (bib_buf_offset(BUF_TY_OUT, 1) >= min_print_line))
            bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) - 1);
        if (bib_buf_offset(BUF_TY_OUT, 1) == min_print_line - 1) {      /*325: */
            bib_set_buf_offset(BUF_TY_OUT, 1, max_print_line + 1);
            while (bib_buf_offset(BUF_TY_OUT, 1) < end_ptr)
                if (LEX_CLASS[bib_buf_at(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1))] != LEX_CLASS_WHITESPACE )
                    bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) + 1);
                else
                    goto lab16;
 lab16:                        /*loop1_exit */ if (bib_buf_offset(BUF_TY_OUT, 1) == end_ptr)
                unbreakable_tail = true;
            else {

                break_pt_found = true;
                while (bib_buf_offset(BUF_TY_OUT, 1) + 1 < end_ptr)
                    if (LEX_CLASS[bib_buf_at(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1) + 1)] == LEX_CLASS_WHITESPACE )
                        bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) + 1);
                    else
                        goto lab17;
 lab17:                        /*loop2_exit */ ;
            }
        } else
            break_pt_found = true;
        if (break_pt_found) {
            bib_set_buf_len(BUF_TY_OUT, bib_buf_offset(BUF_TY_OUT, 1));
            break_ptr = bib_buf_len(BUF_TY_OUT) + 1;
            output_bbl_line();
            bib_buf(BUF_TY_OUT)[0] = 32 /*space */ ;
            bib_buf(BUF_TY_OUT)[1] = 32 /*space */ ;
            bib_set_buf_offset(BUF_TY_OUT, 1, 2);
            tmp_ptr = break_ptr;
            while (tmp_ptr < end_ptr) {
                bib_buf(BUF_TY_OUT)[bib_buf_offset(BUF_TY_OUT, 1)] = bib_buf_at(BUF_TY_OUT, tmp_ptr);
                bib_set_buf_offset(BUF_TY_OUT, 1, bib_buf_offset(BUF_TY_OUT, 1) + 1);
                tmp_ptr = tmp_ptr + 1;
            }
            bib_set_buf_len(BUF_TY_OUT, end_ptr - break_ptr + 2);
        }
    }
}

static void x_equals(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != ctx->pop2.typ) {
        if ((ctx->pop1.typ != 4 /*stk_empty */ ) && (ctx->pop2.typ != 4 /*stk_empty */ )) {
            TRY(print_stk_lit(hash_text, ctx->pop1));
            puts_log(", ");
            TRY(print_stk_lit(hash_text, ctx->pop2));
            putc_log('\n');
            puts_log("---they aren't the same literal types");
            TRY(bst_ex_warn_print(ctx));
        }
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER});
    } else if ((ctx->pop1.typ != 0 /*stk_int */ ) && (ctx->pop1.typ != 1 /*stk_str */ )) {
        if (ctx->pop1.typ != 4 /*stk_empty */ ) {
            TRY(print_stk_lit(hash_text, ctx->pop1));
            puts_log(", not an integer or a string,");
            TRY(bst_ex_warn_print(ctx));
        }
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop1.typ == STK_TYPE_INTEGER) {

        if (ctx->pop2.lit == ctx->pop1.lit)
            push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
        else
            push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (bib_str_eq_str(ctx->pop2.lit, ctx->pop1.lit))
        push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
    else
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
}

static void x_greater_than(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.lit > ctx->pop1.lit)
        push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
    else
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
}

static void x_less_than(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.lit < ctx->pop1.lit)
        push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
    else
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
}

static void x_plus(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop2.lit + ctx->pop1.lit, .typ = STK_TYPE_INTEGER });
}

static void x_minus(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop2.lit - ctx->pop1.lit, .typ = STK_TYPE_INTEGER });
}

static void x_concatenate(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop2.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {                    /*352: */

        if (ctx->pop2.lit >= ctx->bib_str_ptr) {

            if (ctx->pop1.lit >= ctx->bib_str_ptr) {
                bib_set_str_start(ctx->pop1.lit, bib_str_start(ctx->pop1.lit + 1));
                {
                    bib_set_str_ptr(bib_str_ptr() + 1);
                    bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
                }
                ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
            } else if ((bib_str_start(ctx->pop2.lit + 1) - bib_str_start(ctx->pop2.lit)) == 0)
                push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = STK_TYPE_STRING });
            else {

                bib_set_pool_ptr(bib_str_start(ctx->pop2.lit + 1));
                {
                    while (bib_pool_ptr() + (bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) > bib_pool_size())
                        pool_overflow();
                }
                sp_ptr = bib_str_start(ctx->pop1.lit);
                sp_end = bib_str_start(ctx->pop1.lit + 1);
                while (sp_ptr < sp_end) {

                    {
                        bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                        bib_set_pool_ptr(bib_pool_ptr() + 1);
                    }
                    sp_ptr = sp_ptr + 1;
                }
                push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
            }
        } else {                /*353: */

            if (ctx->pop1.lit >= ctx->bib_str_ptr) {

                if ((bib_str_start(ctx->pop2.lit + 1) - bib_str_start(ctx->pop2.lit)) == 0) {
                    {
                        bib_set_str_ptr(bib_str_ptr() + 1);
                        bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
                    }
                    ctx->lit_stack[ctx->lit_stk_ptr].lit = ctx->pop1.lit;
                    ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
                } else if ((bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) == 0)
                    ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
                else {

                    pool_pointer sp_length = (bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit));
                    pool_pointer sp2_length = (bib_str_start(ctx->pop2.lit + 1) - bib_str_start(ctx->pop2.lit));
                    {
                        while (bib_pool_ptr() + sp_length + sp2_length > bib_pool_size())
                            pool_overflow();
                    }
                    sp_ptr = bib_str_start(ctx->pop1.lit + 1);
                    sp_end = bib_str_start(ctx->pop1.lit);
                    pool_pointer sp_xptr1 = sp_ptr + sp2_length;
                    while (sp_ptr > sp_end) {

                        sp_ptr = sp_ptr - 1;
                        sp_xptr1 = sp_xptr1 - 1;
                        bib_set_str_pool(sp_xptr1, bib_str_pool(sp_ptr));
                    }
                    sp_ptr = bib_str_start(ctx->pop2.lit);
                    sp_end = bib_str_start(ctx->pop2.lit + 1);
                    while (sp_ptr < sp_end) {

                        {
                            bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                            bib_set_pool_ptr(bib_pool_ptr() + 1);
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    bib_set_pool_ptr(bib_pool_ptr() + sp_length);
                    push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
                }
            } else {            /*354: */

                if ((bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) == 0)
                    ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
                else if ((bib_str_start(ctx->pop2.lit + 1) - bib_str_start(ctx->pop2.lit)) == 0)
                    push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = STK_TYPE_STRING });
                else {

                    {
                        while ((bib_pool_ptr() + (bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) +
                                (bib_str_start(ctx->pop2.lit + 1) - bib_str_start(ctx->pop2.lit)) > bib_pool_size()))
                            pool_overflow();
                    }
                    sp_ptr = bib_str_start(ctx->pop2.lit);
                    sp_end = bib_str_start(ctx->pop2.lit + 1);
                    while (sp_ptr < sp_end) {

                        {
                            bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                            bib_set_pool_ptr(bib_pool_ptr() + 1);
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    sp_ptr = bib_str_start(ctx->pop1.lit);
                    sp_end = bib_str_start(ctx->pop1.lit + 1);
                    while (sp_ptr < sp_end) {

                        {
                            bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                            bib_set_pool_ptr(bib_pool_ptr() + 1);
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
                }
            }
        }
    }
}

static void x_gets(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 2 /*stk_fn */ )
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 2 /*stk_fn */ );
    else if (((!ctx->mess_with_entries)
              && ((fn_type[ctx->pop1.lit] == 6 /*str_entry_var */ ) || (fn_type[ctx->pop1.lit] == 5 /*int_entry_var */ ))))
        TRY(bst_cant_mess_with_entries_print(ctx));
    else
        switch ((fn_type[ctx->pop1.lit])) {
        case 5:
            /*
               356: */ if (ctx->pop2.typ != 0 /*stk_int */ )
                print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
            else
                entry_ints[cite_ptr() * num_ent_ints + ilk_info[ctx->pop1.lit]] = /*:356 */ ctx->pop2.lit;
            break;
        case 6:
            {
                if (ctx->pop2.typ != 1 /*stk_str */ )
                    print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 1 /*stk_str */ );
                else {

                    str_ent_ptr = cite_ptr() * num_ent_strs + ilk_info[ctx->pop1.lit];
                    ent_chr_ptr = 0;
                    sp_ptr = bib_str_start(ctx->pop2.lit);
                    pool_pointer sp_xptr1 = bib_str_start(ctx->pop2.lit + 1);
                    if (sp_xptr1 - sp_ptr > ent_str_size) {
                        {
                            bst_1print_string_size_exceeded();
                            printf_log("%ld, the entry", (long) ent_str_size);
                            TRY(bst_2print_string_size_exceeded(ctx));
                        }
                        sp_xptr1 = sp_ptr + ent_str_size;
                    }
                    while (sp_ptr < sp_xptr1) {

                        entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)] = bib_str_pool(sp_ptr);
                        ent_chr_ptr = ent_chr_ptr + 1;
                        sp_ptr = sp_ptr + 1;
                    }
                    entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)] = 127 /*end_of_string */ ;
                }
            }
            break;
        case 7:
            if (ctx->pop2.typ != 0 /*stk_int */ )
                print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
            else
                ilk_info[ctx->pop1.lit] = /*:359 */ ctx->pop2.lit;
            break;
        case 8:
            {
                if (ctx->pop2.typ != 1 /*stk_str */ )
                    print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 1 /*stk_str */ );
                else {

                    str_glb_ptr = ilk_info[ctx->pop1.lit];
                    if (ctx->pop2.lit < ctx->bib_str_ptr)
                        glb_bib_str_ptr[str_glb_ptr] = ctx->pop2.lit;
                    else {

                        glb_bib_str_ptr[str_glb_ptr] = 0;
                        glob_chr_ptr = 0;
                        sp_ptr = bib_str_start(ctx->pop2.lit);
                        sp_end = bib_str_start(ctx->pop2.lit + 1);
                        if (sp_end - sp_ptr > glob_str_size) {
                            {
                                bst_1print_string_size_exceeded();
                                printf_log("%ld, the global", (long) glob_str_size);
                                TRY(bst_2print_string_size_exceeded(ctx));
                            }
                            sp_end = sp_ptr + glob_str_size;
                        }
                        while (sp_ptr < sp_end) {

                            global_strs[(str_glb_ptr) * (glob_str_size + 1) + (glob_chr_ptr)] = bib_str_pool(sp_ptr);
                            glob_chr_ptr = glob_chr_ptr + 1;
                            sp_ptr = sp_ptr + 1;
                        }
                        glb_str_end[str_glb_ptr] = glob_chr_ptr;
                    }
                }
            }
            break;
        default:
            puts_log("You can't assign to type ");
            print_fn_class(ctx->pop1.lit);
            puts_log(", a nonvariable function class");
            TRY(bst_ex_warn_print(ctx));
            break;
        }
}

static void x_add_period(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if ((bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) == 0)
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    else {                      /*362: */

        sp_ptr = bib_str_start(ctx->pop1.lit + 1);
        sp_end = bib_str_start(ctx->pop1.lit);
        while (sp_ptr > sp_end) {

            sp_ptr = sp_ptr - 1;
            if (bib_str_pool(sp_ptr) != 125 /*right_brace */ )
                goto loop_exit;
        }
 loop_exit:
        switch ((bib_str_pool(sp_ptr))) {
        case 46:
        case 63:
        case 33:
            {
                if (ctx->lit_stack[ctx->lit_stk_ptr].lit >= ctx->bib_str_ptr) {
                    bib_set_str_ptr(bib_str_ptr() + 1);
                    bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
                }
                ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
            }
            break;
        default:
            {
                if (ctx->pop1.lit < ctx->bib_str_ptr) {
                    {
                        while (bib_pool_ptr() + (bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) + 1 > bib_pool_size())
                            pool_overflow();
                    }
                    sp_ptr = bib_str_start(ctx->pop1.lit);
                    sp_end = bib_str_start(ctx->pop1.lit + 1);
                    while (sp_ptr < sp_end) {

                        {
                            bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                            bib_set_pool_ptr(bib_pool_ptr() + 1);
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                } else {

                    bib_set_pool_ptr(bib_str_start(ctx->pop1.lit + 1));
                    {
                        while (bib_pool_ptr() + 1 > bib_pool_size())
                            pool_overflow();
                    }
                }
                {
                    bib_set_str_pool(bib_pool_ptr(), 46 /*period */ );
                    bib_set_pool_ptr(bib_pool_ptr() + 1);
                }
                push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
            }
            break;
        }
    }
}

static void x_change_case(ExecCtx* ctx)
{
    buf_pointer tmp_ptr;
    bool prev_colon = false;

    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop2.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {

        {
            switch ((bib_str_pool(bib_str_start(ctx->pop1.lit)))) {
            case 116:
            case 84:
                conversion_type = 0 /*title_lowers */ ;
                break;
            case 108:
            case 76:
                conversion_type = 1 /*all_lowers */ ;
                break;
            case 117:
            case 85:
                conversion_type = 2 /*all_uppers */ ;
                break;
            default:
                conversion_type = 3 /*bad_conversion */ ;
                break;
            }
            if (((bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) != 1) || (conversion_type == 3 /*bad_conversion */ )) {
                conversion_type = 3 /*bad_conversion */ ;
                TRY(print_a_pool_str(ctx->pop1.lit));
                puts_log(" is an illegal case-conversion string");
                TRY(bst_ex_warn_print(ctx));
            }
        }
        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop2.lit);
        {
            brace_level = 0;
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            while (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) {

                if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ ) {
                    brace_level = brace_level + 1;
                    if (brace_level != 1)
                        goto lab21;
                    if (bib_buf_offset(BUF_TY_EX, 1) + 4 > bib_buf_len(BUF_TY_EX))
                        goto lab21;
                    else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 1] != 92 /*backslash */ )
                        goto lab21;
                    if (conversion_type == 0 /*title_lowers */ ) {

                        if (bib_buf_offset(BUF_TY_EX, 1) == 0)
                            goto lab21;
                        else if ((prev_colon) && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1]] == LEX_CLASS_WHITESPACE ))
                            goto lab21;
                    }
                    {
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                        while ((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)) {

                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            buf_pointer ex_buf_xptr = bib_buf_offset(BUF_TY_EX, 1);
                            while ((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_ALPHA ))
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            control_seq_loc =
                                str_lookup(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr, 14 /*control_seq_ilk */ ,
                                           false);
                            if (hash_found) { /*373: */
                                switch ((conversion_type)) {
                                case 0:
                                case 1:
                                    switch ((ilk_info[control_seq_loc])) {
                                    case 11:
                                    case 9:
                                    case 3:
                                    case 5:
                                    case 7:
                                        lower_case(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                                        break;
                                    default:
                                        ;
                                        break;
                                    }
                                    break;
                                case 2:
                                    switch ((ilk_info[control_seq_loc])) {
                                    case 10:
                                    case 8:
                                    case 2:
                                    case 4:
                                    case 6:
                                        upper_case(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                                        break;
                                    case 0:
                                    case 1:
                                    case 12:
                                        {
                                            upper_case(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                                            while (ex_buf_xptr < bib_buf_offset(BUF_TY_EX, 1)) {

                                                bib_buf(BUF_TY_EX)[ex_buf_xptr - 1] = bib_buf(BUF_TY_EX)[ex_buf_xptr];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            ex_buf_xptr = ex_buf_xptr - 1;
                                            while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))
                                                    && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_WHITESPACE )))
                                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                            tmp_ptr = bib_buf_offset(BUF_TY_EX, 1);
                                            while (tmp_ptr < bib_buf_len(BUF_TY_EX)) {

                                                bib_buf(BUF_TY_EX)[tmp_ptr - (bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr)] = bib_buf(BUF_TY_EX)[tmp_ptr];
                                                tmp_ptr = tmp_ptr + 1;
                                            }
                                            bib_set_buf_len(BUF_TY_EX, tmp_ptr - (bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr));
                                            bib_set_buf_offset(BUF_TY_EX, 1, ex_buf_xptr);
                                        }
                                        break;
                                    default:
                                        ;
                                        break;
                                    }
                                    break;
                                case 3:
                                    ;
                                    break;
                                default:
                                    case_conversion_confusion();
                                    longjmp(error_jmpbuf, 1);
                                    break;
                                }
                            }
                            ex_buf_xptr = bib_buf_offset(BUF_TY_EX, 1);
                            while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)
                                    && (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] != 92 /*backslash */ ))) {

                                if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ )
                                    brace_level = brace_level - 1;
                                else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ )
                                    brace_level = brace_level + 1;
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            }
                            {
                                switch ((conversion_type)) {
                                case 0:
                                case 1:
                                    lower_case(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                                    break;
                                case 2:
                                    upper_case(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr);
                                    break;
                                case 3:
                                    ;
                                    break;
                                default:
                                    case_conversion_confusion();
                                    longjmp(error_jmpbuf, 1);
                                    break;
                                }
                            }
                        }
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                    }
 lab21:                        /*ok_pascal_i_give_up */ prev_colon = false;
                } else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ ) {
                    decr_brace_level(ctx, ctx->pop2.lit);
                    prev_colon = false;
                } else if (brace_level == 0) {        /*377: */
                    switch ((conversion_type)) {
                    case 0:
                        {
                            if (bib_buf_offset(BUF_TY_EX, 1) == 0) ;
                            else if ((prev_colon) && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1]] == LEX_CLASS_WHITESPACE )) ;
                            else
                                lower_case(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_EX, 1), 1);
                            if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 58 /*colon */ )
                                prev_colon = true;
                            else if (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] != LEX_CLASS_WHITESPACE )
                                prev_colon = false;
                        }
                        break;
                    case 1:
                        lower_case(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_EX, 1), 1);
                        break;
                    case 2:
                        upper_case(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_EX, 1), 1);
                        break;
                    case 3:
                        ;
                        break;
                    default:
                        case_conversion_confusion();
                        longjmp(error_jmpbuf, 1);
                        break;
                    }
                }
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
            }
            check_brace_level(ctx, ctx->pop2.lit);
        }
        add_pool_buf_and_push(ctx);
    }
}

static void x_chr_to_int(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if ((bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) != 1) {
        putc_log('"');
        TRY(print_a_pool_str(ctx->pop1.lit));
        puts_log("\" isn't a single character");
        TRY(bst_ex_warn_print(ctx));
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else
        push_lit_stk(ctx, (ExecVal) { .lit = bib_str_pool(bib_str_start(ctx->pop1.lit)), .typ = STK_TYPE_INTEGER });
}

static void x_cite(ExecCtx* ctx)
{
    if (!ctx->mess_with_entries)
        TRY(bst_cant_mess_with_entries_print(ctx));
    else
        push_lit_stk(ctx, (ExecVal) { .lit = cite_list(cite_ptr()), .typ = STK_TYPE_STRING });
}

static void x_duplicate(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = ctx->pop1.typ });
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = ctx->pop1.typ });
    } else {

        {
            if (ctx->lit_stack[ctx->lit_stk_ptr].lit >= ctx->bib_str_ptr) {
                bib_set_str_ptr(bib_str_ptr() + 1);
                bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
            }
            ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
        }
        if (ctx->pop1.lit < ctx->bib_str_ptr)
            push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = ctx->pop1.typ });
        else {

            {
                while (bib_pool_ptr() + (bib_str_start(ctx->pop1.lit + 1) - bib_str_start(ctx->pop1.lit)) > bib_pool_size())
                    pool_overflow();
            }
            pool_pointer sp_ptr = bib_str_start(ctx->pop1.lit);
            pool_pointer sp_end = bib_str_start(ctx->pop1.lit + 1);
            while (sp_ptr < sp_end) {

                {
                    bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                    bib_set_pool_ptr(bib_pool_ptr() + 1);
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
        }
    }
}

static void x_empty(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    switch ((ctx->pop1.typ)) {
    case 1:
        {
            pool_pointer sp_ptr = bib_str_start(ctx->pop1.lit);
            pool_pointer sp_end = bib_str_start(ctx->pop1.lit + 1);
            while (sp_ptr < sp_end) {

                if (LEX_CLASS[bib_str_pool(sp_ptr)] != LEX_CLASS_WHITESPACE ) {
                    push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
                    return;
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
        }
        break;
    case 3:
        push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
        break;
    case 4:
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
        break;
    default:
        TRY(print_stk_lit(hash_text, ctx->pop1));
        puts_log(", not a string or missing field,");
        TRY(bst_ex_warn_print(ctx));
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
        break;
    }
}

static void x_format_name(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    pop_lit_stk(ctx, &ctx->pop3);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop3.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop3, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {
        buf_pointer ex_buf_xptr = 0;
        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop3.lit);
        {
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            num_names = 0;
            while ((num_names < ctx->pop2.lit) && (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))) {

                num_names = num_names + 1;
                ex_buf_xptr = bib_buf_offset(BUF_TY_EX, 1);
                name_scan_for_and(ctx, ctx->pop3.lit);
            }
            if (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 4);
            if (num_names < ctx->pop2.lit) {
                if (ctx->pop2.lit == 1) {
                    puts_log("There is no name in \"");
                } else {
                    printf_log("There aren't %ld names in \"", (long) ctx->pop2.lit);
                }
                TRY(print_a_pool_str(ctx->pop3.lit));
                {
                    putc_log('"');
                    TRY(bst_ex_warn_print(ctx));
                }
            }
        }
        buf_pointer num_tokens = 0, comma1 = 0, comma2 = 0, num_commas = 0, name_bf_ptr = 0;
        {
            {
                while (bib_buf_offset(BUF_TY_EX, 1) > ex_buf_xptr)
                    switch ((LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1]])) {
                    case LEX_CLASS_WHITESPACE:
                    case LEX_CLASS_SEP:
                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                        break;
                    default:
                        if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) - 1] == 44 /*comma */ ) {
                            printf_log("Name %ld in \"", (long) ctx->pop2.lit);
                            TRY(print_a_pool_str(ctx->pop3.lit));
                            puts_log("\" has a comma at the end");
                            TRY(bst_ex_warn_print(ctx));
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                        } else
                            goto lab16;
                        break;
                    }
 lab16:                        /*loop1_exit */ ;
            }
            bool token_starting = true;
            while (ex_buf_xptr < bib_buf_offset(BUF_TY_EX, 1))
                switch ((bib_buf(BUF_TY_EX)[ex_buf_xptr])) {
                case 44:
                    {
                        if (num_commas == 2) {
                            printf_log("Too many commas in name %ld of \"", (long) ctx->pop2.lit);
                            TRY(print_a_pool_str(ctx->pop3.lit));
                            putc_log('"');
                            TRY(bst_ex_warn_print(ctx));
                        } else {

                            num_commas = num_commas + 1;
                            if (num_commas == 1)
                                comma1 = num_tokens;
                            else
                                comma2 = num_tokens;
                            name_sep_char[num_tokens] = 44 /*comma */ ;
                        }
                        ex_buf_xptr = ex_buf_xptr + 1;
                        token_starting = true;
                    }
                    break;
                case 123:
                    {
                        brace_level = brace_level + 1;
                        if (token_starting) {
                            bib_buf(BUF_TY_NAME_TOK)[num_tokens] = name_bf_ptr;
                            num_tokens = num_tokens + 1;
                        }
                        bib_buf(BUF_TY_SV)[name_bf_ptr] = bib_buf(BUF_TY_EX)[ex_buf_xptr];
                        name_bf_ptr = name_bf_ptr + 1;
                        ex_buf_xptr = ex_buf_xptr + 1;
                        while ((brace_level > 0) && (ex_buf_xptr < bib_buf_offset(BUF_TY_EX, 1))) {

                            if (bib_buf(BUF_TY_EX)[ex_buf_xptr] == 125 /*right_brace */ )
                                brace_level = brace_level - 1;
                            else if (bib_buf(BUF_TY_EX)[ex_buf_xptr] == 123 /*left_brace */ )
                                brace_level = brace_level + 1;
                            bib_buf(BUF_TY_SV)[name_bf_ptr] = bib_buf(BUF_TY_EX)[ex_buf_xptr];
                            name_bf_ptr = name_bf_ptr + 1;
                            ex_buf_xptr = ex_buf_xptr + 1;
                        }
                        token_starting = false;
                    }
                    break;
                case 125:
                    {
                        if (token_starting) {
                            bib_buf(BUF_TY_NAME_TOK)[num_tokens] = name_bf_ptr;
                            num_tokens = num_tokens + 1;
                        }

                        printf_log("Name %ld of \"", (long) ctx->pop2.lit);
                        TRY(print_a_pool_str(ctx->pop3.lit));
                        puts_log("\" isn't brace balanced");
                        TRY(bst_ex_warn_print(ctx));
                        ex_buf_xptr = ex_buf_xptr + 1;
                        token_starting = false;
                    }
                    break;
                default:
                    switch ((LEX_CLASS[bib_buf(BUF_TY_EX)[ex_buf_xptr]])) {
                    case LEX_CLASS_WHITESPACE:
                        {
                            if (!token_starting)
                                name_sep_char[num_tokens] = 32 /*space */ ;
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = true;
                        }
                        break;
                    case LEX_CLASS_SEP:
                        {
                            if (!token_starting)
                                name_sep_char[num_tokens] = bib_buf(BUF_TY_EX)[ex_buf_xptr];
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = true;
                        }
                        break;
                    default:
                        {
                            if (token_starting) {
                                bib_buf(BUF_TY_NAME_TOK)[num_tokens] = name_bf_ptr;
                                num_tokens = num_tokens + 1;
                            }
                            bib_buf(BUF_TY_SV)[name_bf_ptr] = bib_buf(BUF_TY_EX)[ex_buf_xptr];
                            name_bf_ptr = name_bf_ptr + 1;
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = false;
                        }
                        break;
                    }
                    break;
                }
            bib_buf(BUF_TY_NAME_TOK)[num_tokens] = name_bf_ptr;
        }
        buf_pointer first_start = 0, first_end = 0, last_end = 0, von_start = 0, von_end = 0, jr_end = 0, name_bf_xptr = 0;
        {
            if (num_commas == 0) {
                first_start = 0;
                last_end = num_tokens;
                jr_end = last_end;
                {
                    von_start = 0;
                    while (von_start < last_end - 1) {

                        name_bf_ptr = bib_buf_at(BUF_TY_NAME_TOK, von_start);
                        name_bf_xptr = bib_buf_at(BUF_TY_NAME_TOK, von_start + 1);
                        if (von_token_found(&name_bf_ptr, name_bf_xptr)) {
                            von_name_ends_and_last_name_starts_stuff(last_end, von_start, &von_end, &name_bf_ptr, &name_bf_xptr);
                            goto lab52;
                        }
                        von_start = von_start + 1;
                    }
                    while (von_start > 0) {

                        if (((LEX_CLASS[name_sep_char[von_start]] != LEX_CLASS_SEP )
                             || (name_sep_char[von_start] == 126 /*tie */ )))
                            goto lab17;
                        von_start = von_start - 1;
                    }
 lab17:                        /*loop2_exit */ von_end = von_start;
 lab52:                        /*von_found */ first_end = von_start;
                }
            } else if (num_commas == 1) {
                von_start = 0;
                last_end = comma1;
                jr_end = last_end;
                first_start = jr_end;
                first_end = num_tokens;
                von_name_ends_and_last_name_starts_stuff(last_end, von_start, &von_end, &name_bf_ptr, &name_bf_xptr);
            } else if (num_commas == 2) {
                von_start = 0;
                last_end = comma1;
                jr_end = comma2;
                first_start = jr_end;
                first_end = num_tokens;
                von_name_ends_and_last_name_starts_stuff(last_end, von_start, &von_end, &name_bf_ptr, &name_bf_xptr);
            } else {
                puts_log("Illegal number of comma,s");
                print_confusion();
                longjmp(error_jmpbuf, 1);
            }
        }
        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop1.lit);
        figure_out_the_formatted_name(ctx, first_start, first_end, last_end, von_start, von_end, &name_bf_ptr, &name_bf_xptr, jr_end);
        add_pool_buf_and_push(ctx);
    }
}

static void x_int_to_chr(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if ((ctx->pop1.lit < 0) || (ctx->pop1.lit > 127)) {
        printf_log("%ld isn't valid ASCII", (long) ctx->pop1.lit);
        TRY(bst_ex_warn_print(ctx));
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {

        {
            while (bib_pool_ptr() + 1 > bib_pool_size())
                pool_overflow();
        }
        {
            bib_set_str_pool(bib_pool_ptr(), ctx->pop1.lit);
            bib_set_pool_ptr(bib_pool_ptr() + 1);
        }
        push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
    }
}

static void x_int_to_str(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {
        bib_set_buf_len(BUF_TY_EX, int_to_ascii(ctx->pop1.lit, BUF_TY_EX, 0));
        add_pool_buf_and_push(ctx);
    }
}

static void x_missing(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (!ctx->mess_with_entries)
        TRY(bst_cant_mess_with_entries_print(ctx));
    else if ((ctx->pop1.typ != 1 /*stk_str */ ) && (ctx->pop1.typ != 3 /*stk_field_missing */ )) {
        if (ctx->pop1.typ != 4 /*stk_empty */ ) {
            TRY(print_stk_lit(hash_text, ctx->pop1));
            puts_log(", not a string or missing field,");
            TRY(bst_ex_warn_print(ctx));
        }
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else if (ctx->pop1.typ == STK_TYPE_MISSING)
        push_lit_stk(ctx, (ExecVal) { .lit = 1, .typ = STK_TYPE_INTEGER });
    else
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
}

static void x_num_names(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else {

        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop1.lit);
        {
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            num_names = 0;
            while (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) {

                name_scan_for_and(ctx, ctx->pop1.lit);
                num_names = num_names + 1;
            }
        }
        push_lit_stk(ctx, (ExecVal) { .lit = num_names, .typ = STK_TYPE_INTEGER });
    }
}

static void x_preamble(ExecCtx* ctx)
{
    bib_set_buf_len(BUF_TY_EX, 0);
    set_preamble_ptr(0);
    while (preamble_ptr() < ctx->bst_ctx->num_preamble_strings) {
        add_buf_pool(cur_preamble());
        set_preamble_ptr(preamble_ptr() + 1);
    }
    add_pool_buf_and_push(ctx);
}

static void x_purify(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {

        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop1.lit);
        {
            brace_level = 0;
            buf_pointer ex_buf_xptr = 0;
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            while (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) {

                switch ((LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]])) {
                case LEX_CLASS_WHITESPACE:
                case LEX_CLASS_SEP:
                    {
                        bib_buf(BUF_TY_EX)[ex_buf_xptr] = 32 /*space */ ;
                        ex_buf_xptr = ex_buf_xptr + 1;
                    }
                    break;
                case LEX_CLASS_ALPHA:
                case LEX_CLASS_NUMERIC:
                    {
                        bib_buf(BUF_TY_EX)[ex_buf_xptr] = bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)];
                        ex_buf_xptr = ex_buf_xptr + 1;
                    }
                    break;
                default:
                    if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ ) {
                        brace_level = brace_level + 1;
                        if ((brace_level == 1) && (bib_buf_offset(BUF_TY_EX, 1) + 1 < bib_buf_len(BUF_TY_EX))) {

                            if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 1] == 92 /*backslash */ ) {       /*433: */
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                while ((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)) {

                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    buf_pointer ex_buf_yptr = bib_buf_offset(BUF_TY_EX, 1);
                                    while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))
                                            && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_ALPHA )))
                                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    control_seq_loc =
                                        str_lookup(bib_buf(BUF_TY_EX), ex_buf_yptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_yptr,
                                                   14 /*control_seq_ilk */ , false);
                                    if (hash_found) { /*434: */
                                        bib_buf(BUF_TY_EX)[ex_buf_xptr] = bib_buf(BUF_TY_EX)[ex_buf_yptr];
                                        ex_buf_xptr = ex_buf_xptr + 1;
                                        switch ((ilk_info[control_seq_loc])) {
                                        case 2:
                                        case 3:
                                        case 4:
                                        case 5:
                                        case 12:
                                            {
                                                bib_buf(BUF_TY_EX)[ex_buf_xptr] = bib_buf(BUF_TY_EX)[ex_buf_yptr + 1];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            break;
                                        default:
                                            ;
                                            break;
                                        }
                                    }
                                    while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)
                                            && (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] != 92 /*backslash */ ))) {

                                        switch ((LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]])) {
                                        case LEX_CLASS_ALPHA:
                                        case LEX_CLASS_NUMERIC:
                                            {
                                                bib_buf(BUF_TY_EX)[ex_buf_xptr] = bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            break;
                                        default:
                                            if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ )
                                                brace_level = brace_level - 1;
                                            else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ )
                                                brace_level = brace_level + 1;
                                            break;
                                        }
                                        bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                    }
                                }
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                            }
                        }
                    } else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ ) {

                        if (brace_level > 0)
                            brace_level = brace_level - 1;
                    }
                    break;
                }
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
            }
            bib_set_buf_len(BUF_TY_EX, ex_buf_xptr);
        }
        add_pool_buf_and_push(ctx);
    }
}

static void x_quote(ExecCtx* ctx)
{
    {
        while (bib_pool_ptr() + 1 > bib_pool_size())
            pool_overflow();
    }
    {
        bib_set_str_pool(bib_pool_ptr(), 34 /*double_quote */ );
        bib_set_pool_ptr(bib_pool_ptr() + 1);
    }
    push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
}

static void x_substring(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;

    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    pop_lit_stk(ctx, &ctx->pop3);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop2.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop3.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop3, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {

        pool_pointer sp_length = (bib_str_start(ctx->pop3.lit + 1) - bib_str_start(ctx->pop3.lit));
        if (ctx->pop1.lit >= sp_length) {

            if ((ctx->pop2.lit == 1) || (ctx->pop2.lit == -1)) {
                {
                    if (ctx->lit_stack[ctx->lit_stk_ptr].lit >= ctx->bib_str_ptr) {
                        bib_set_str_ptr(bib_str_ptr() + 1);
                        bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
                    }
                    ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
                }
                return;
            }
        }
        if ((ctx->pop1.lit <= 0) || (ctx->pop2.lit == 0) || (ctx->pop2.lit > sp_length) || (ctx->pop2.lit < -(int32_t) sp_length)) {
            push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
            return;
        } else {                /*439: */

            if (ctx->pop2.lit > 0) {
                if (ctx->pop1.lit > sp_length - (ctx->pop2.lit - 1))
                    ctx->pop1.lit = sp_length - (ctx->pop2.lit - 1);
                sp_ptr = bib_str_start(ctx->pop3.lit) + (ctx->pop2.lit - 1);
                sp_end = sp_ptr + ctx->pop1.lit;
                if (ctx->pop2.lit == 1) {

                    if (ctx->pop3.lit >= ctx->bib_str_ptr) {
                        bib_set_str_start(ctx->pop3.lit + 1, sp_end);
                        {
                            bib_set_str_ptr(bib_str_ptr() + 1);
                            bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
                        }
                        ctx->lit_stk_ptr = ctx->lit_stk_ptr + 1;
                        return;
                    }
                }
            } else {

                ctx->pop2.lit = -(int32_t) ctx->pop2.lit;
                if (ctx->pop1.lit > sp_length - (ctx->pop2.lit - 1))
                    ctx->pop1.lit = sp_length - (ctx->pop2.lit - 1);
                sp_end = bib_str_start(ctx->pop3.lit + 1) - (ctx->pop2.lit - 1);
                sp_ptr = sp_end - ctx->pop1.lit;
            }
            {
                while (bib_pool_ptr() + sp_end - sp_ptr > bib_pool_size())
                    pool_overflow();
            }
            while (sp_ptr < sp_end) {

                {
                    bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                    bib_set_pool_ptr(bib_pool_ptr() + 1);
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
        }
    }
}

static void x_swap(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if ((ctx->pop1.typ != STK_TYPE_STRING ) || (ctx->pop1.lit < ctx->bib_str_ptr)) {
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = ctx->pop1.typ });
        if ((ctx->pop2.typ == STK_TYPE_STRING ) && (ctx->pop2.lit >= ctx->bib_str_ptr)) {
            bib_set_str_ptr(bib_str_ptr() + 1);
            bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
        }
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop2.lit, .typ = ctx->pop2.typ });
    } else if ((ctx->pop2.typ != STK_TYPE_STRING ) || (ctx->pop2.lit < ctx->bib_str_ptr)) {
        {
            bib_set_str_ptr(bib_str_ptr() + 1);
            bib_set_pool_ptr(bib_str_start(bib_str_ptr()));
        }
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop1.lit, .typ = STK_TYPE_STRING });
        push_lit_stk(ctx, (ExecVal) { .lit = ctx->pop2.lit, .typ = ctx->pop2.typ });
    } else {                    /*441: */

        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop2.lit);
        pool_pointer sp_ptr = bib_str_start(ctx->pop1.lit);
        pool_pointer sp_end = bib_str_start(ctx->pop1.lit + 1);
        while (sp_ptr < sp_end) {

            {
                bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                bib_set_pool_ptr(bib_pool_ptr() + 1);
            }
            sp_ptr = sp_ptr + 1;
        }
        push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
        add_pool_buf_and_push(ctx);
    }
}

static void x_text_length(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;

    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else {

        num_text_chars = 0;
        {
            sp_ptr = bib_str_start(ctx->pop1.lit);
            sp_end = bib_str_start(ctx->pop1.lit + 1);
            int32_t sp_brace_level = 0;
            while (sp_ptr < sp_end) {

                sp_ptr = sp_ptr + 1;
                if (bib_str_pool(sp_ptr - 1) == 123 /*left_brace */ ) {
                    sp_brace_level = sp_brace_level + 1;
                    if ((sp_brace_level == 1) && (sp_ptr < sp_end)) {

                        if (bib_str_pool(sp_ptr) == 92 /*backslash */ ) {
                            sp_ptr = sp_ptr + 1;
                            while ((sp_ptr < sp_end) && (sp_brace_level > 0)) {

                                if (bib_str_pool(sp_ptr) == 125 /*right_brace */ )
                                    sp_brace_level = sp_brace_level - 1;
                                else if (bib_str_pool(sp_ptr) == 123 /*left_brace */ )
                                    sp_brace_level = sp_brace_level + 1;
                                sp_ptr = sp_ptr + 1;
                            }
                            num_text_chars = num_text_chars + 1;
                        }
                    }
                } else if (bib_str_pool(sp_ptr - 1) == 125 /*right_brace */ ) {
                    if (sp_brace_level > 0)
                        sp_brace_level = sp_brace_level - 1;
                } else
                    num_text_chars = num_text_chars + 1;
            }
        }
        push_lit_stk(ctx, (ExecVal) { .lit = num_text_chars, .typ = STK_TYPE_INTEGER });
    }
}

static void x_text_prefix(ExecCtx* ctx)
{
    pool_pointer sp_ptr, sp_end;

    pop_lit_stk(ctx, &ctx->pop1);
    pop_lit_stk(ctx, &ctx->pop2);
    if (ctx->pop1.typ != 0 /*stk_int */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop2.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    } else if (ctx->pop1.lit <= 0) {
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
        return;
    } else {                    /*445: */
        int32_t sp_brace_level = 0;
        sp_ptr = bib_str_start(ctx->pop2.lit);
        sp_end = bib_str_start(ctx->pop2.lit + 1);
        {
            num_text_chars = 0;
            pool_pointer sp_xptr1 = sp_ptr;
            while ((sp_xptr1 < sp_end) && (num_text_chars < ctx->pop1.lit)) {

                sp_xptr1 = sp_xptr1 + 1;
                if (bib_str_pool(sp_xptr1 - 1) == 123 /*left_brace */ ) {
                    sp_brace_level = sp_brace_level + 1;
                    if ((sp_brace_level == 1) && (sp_xptr1 < sp_end)) {

                        if (bib_str_pool(sp_xptr1) == 92 /*backslash */ ) {
                            sp_xptr1 = sp_xptr1 + 1;
                            while ((sp_xptr1 < sp_end) && (sp_brace_level > 0)) {

                                if (bib_str_pool(sp_xptr1) == 125 /*right_brace */ )
                                    sp_brace_level = sp_brace_level - 1;
                                else if (bib_str_pool(sp_xptr1) == 123 /*left_brace */ )
                                    sp_brace_level = sp_brace_level + 1;
                                sp_xptr1 = sp_xptr1 + 1;
                            }
                            num_text_chars = num_text_chars + 1;
                        }
                    }
                } else if (bib_str_pool(sp_xptr1 - 1) == 125 /*right_brace */ ) {
                    if (sp_brace_level > 0)
                        sp_brace_level = sp_brace_level - 1;
                } else
                    num_text_chars = num_text_chars + 1;
            }
            sp_end = sp_xptr1;
        }
        {
            while (bib_pool_ptr() + sp_brace_level + sp_end - sp_ptr > bib_pool_size())
                pool_overflow();
        }
        if (ctx->pop2.lit >= ctx->bib_str_ptr)
            bib_set_pool_ptr(sp_end);
        else
            while (sp_ptr < sp_end) {

                {
                    bib_set_str_pool(bib_pool_ptr(), bib_str_pool(sp_ptr));
                    bib_set_pool_ptr(bib_pool_ptr() + 1);
                }
                sp_ptr = sp_ptr + 1;
            }
        while (sp_brace_level > 0) {

            {
                bib_set_str_pool(bib_pool_ptr(), 125 /*right_brace */ );
                bib_set_pool_ptr(bib_pool_ptr() + 1);
            }
            sp_brace_level = sp_brace_level - 1;
        }
        push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
    }
}

static void x_type(ExecCtx* ctx)
{
    if (!ctx->mess_with_entries)
        TRY(bst_cant_mess_with_entries_print(ctx));
    else if ((type_list(cite_ptr()) == undefined) || (type_list(cite_ptr()) == 0 /*empty */ ))
        push_lit_stk(ctx, (ExecVal) { .lit = s_null, .typ = STK_TYPE_STRING });
    else
        push_lit_stk(ctx, (ExecVal) { .lit = hash_text[type_list(cite_ptr())], .typ = STK_TYPE_STRING });
}

static void x_warning(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ )
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
    else {
        puts_log("Warning--");
        TRY(print_lit(hash_text, ctx->pop1));
        mark_warning();
    }
}

static void x_width(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ ) {
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
        push_lit_stk(ctx, (ExecVal) { .lit = 0, .typ = STK_TYPE_INTEGER });
    } else {

        bib_set_buf_len(BUF_TY_EX, 0);
        add_buf_pool(ctx->pop1.lit);
        string_width = 0;
        {
            brace_level = 0;
            bib_set_buf_offset(BUF_TY_EX, 1, 0);
            while (bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) {

                if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ ) {
                    brace_level = brace_level + 1;
                    if ((brace_level == 1) && (bib_buf_offset(BUF_TY_EX, 1) + 1 < bib_buf_len(BUF_TY_EX))) {

                        if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1) + 1] == 92 /*backslash */ ) {   /*453: */
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                            while ((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)) {

                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                buf_pointer ex_buf_xptr = bib_buf_offset(BUF_TY_EX, 1);
                                while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))
                                        && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_ALPHA )))
                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                if ((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (bib_buf_offset(BUF_TY_EX, 1) == ex_buf_xptr))
                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                else {

                                    control_seq_loc =
                                        str_lookup(bib_buf(BUF_TY_EX), ex_buf_xptr, bib_buf_offset(BUF_TY_EX, 1) - ex_buf_xptr,
                                                   14 /*control_seq_ilk */ , false);
                                    if (hash_found) { /*454: */
                                        switch ((ilk_info[control_seq_loc])) {
                                        case 12:
                                            string_width = string_width + 500;
                                            break;
                                        case 4:
                                            string_width = string_width + 722;
                                            break;
                                        case 2:
                                            string_width = string_width + 778;
                                            break;
                                        case 5:
                                            string_width = string_width + 903;
                                            break;
                                        case 3:
                                            string_width = string_width + 1014;
                                            break;
                                        default:
                                            string_width = string_width + CHAR_WIDTH[bib_buf(BUF_TY_EX)[ex_buf_xptr]];
                                            break;
                                        }
                                    }
                                }
                                while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX))
                                        && (LEX_CLASS[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]] == LEX_CLASS_WHITESPACE )))
                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                while (((bib_buf_offset(BUF_TY_EX, 1) < bib_buf_len(BUF_TY_EX)) && (brace_level > 0)
                                        && (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] != 92 /*backslash */ ))) {

                                    if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ )
                                        brace_level = brace_level - 1;
                                    else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 123 /*left_brace */ )
                                        brace_level = brace_level + 1;
                                    else
                                        string_width = string_width + CHAR_WIDTH[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]];
                                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                }
                            }
                            bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) - 1);
                        } else
                            string_width = string_width + CHAR_WIDTH[123 /*left_brace */ ];
                    } else
                        string_width = string_width + CHAR_WIDTH[123 /*left_brace */ ];
                } else if (bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] == 125 /*right_brace */ ) {
                    decr_brace_level(ctx, ctx->pop1.lit);
                    string_width = string_width + CHAR_WIDTH[125 /*right_brace */ ];
                } else
                    string_width = string_width + CHAR_WIDTH[bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)]];
                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
            }
            check_brace_level(ctx, ctx->pop1.lit);
        }
        push_lit_stk(ctx, (ExecVal) { .lit = string_width, .typ = STK_TYPE_INTEGER });
    }
}

static void x_write(ExecCtx* ctx)
{
    pop_lit_stk(ctx, &ctx->pop1);
    if (ctx->pop1.typ != 1 /*stk_str */ )
        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 1 /*stk_str */ );
    else
        add_out_pool(ctx->pop1.lit);
}

static void execute_fn(ExecCtx* ctx, hash_loc ex_fn_loc)
{
    ExecVal r_pop1, r_pop2;
    wiz_fn_loc wiz_ptr;
    ;

    switch ((fn_type[ex_fn_loc])) {
    case 0:
        {
            ;

            switch ((ilk_info[ex_fn_loc])) {
            case 0:
                x_equals(ctx);
                break;
            case 1:
                x_greater_than(ctx);
                break;
            case 2:
                x_less_than(ctx);
                break;
            case 3:
                x_plus(ctx);
                break;
            case 4:
                x_minus(ctx);
                break;
            case 5:
                x_concatenate(ctx);
                break;
            case 6:
                x_gets(ctx);
                break;
            case 7:
                x_add_period(ctx);
                break;
            case 8:
                {
                    if (!ctx->mess_with_entries)
                        TRY(bst_cant_mess_with_entries_print(ctx));
                    else if (type_list(cite_ptr()) == undefined)
                        execute_fn(ctx, b_default);
                    else if (type_list(cite_ptr()) == 0 /*empty */ ) ;
                    else
                        execute_fn(ctx, type_list(cite_ptr()));
                }
                break;
            case 9:
                x_change_case(ctx);
                break;
            case 10:
                x_chr_to_int(ctx);
                break;
            case 11:
                x_cite(ctx);
                break;
            case 12:
                x_duplicate(ctx);
                break;
            case 13:
                x_empty(ctx);
                break;
            case 14:
                x_format_name(ctx);
                break;
            case 15:
                {
                    pop_lit_stk(ctx, &ctx->pop1);
                    pop_lit_stk(ctx, &ctx->pop2);
                    pop_lit_stk(ctx, &ctx->pop3);
                    if (ctx->pop1.typ != 2 /*stk_fn */ )
                        print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 2 /*stk_fn */ );
                    else if (ctx->pop2.typ != 2 /*stk_fn */ )
                        print_wrong_stk_lit(hash_text, ctx, ctx->pop2, 2 /*stk_fn */ );
                    else if (ctx->pop3.typ != 0 /*stk_int */ )
                        print_wrong_stk_lit(hash_text, ctx, ctx->pop3, 0 /*stk_int */ );
                    else if (ctx->pop3.lit > 0)
                        execute_fn(ctx, ctx->pop2.lit);
                    else
                        execute_fn(ctx, ctx->pop1.lit);
                }
                break;
            case 16:
                x_int_to_chr(ctx);
                break;
            case 17:
                x_int_to_str(ctx);
                break;
            case 18:
                x_missing(ctx);
                break;
            case 19:
                {
                    output_bbl_line();
                }
                break;
            case 20:
                x_num_names(ctx);
                break;
            case 21:
                {
                    pop_lit_stk(ctx, &ctx->pop1);
                }
                break;
            case 22:
                x_preamble(ctx);
                break;
            case 23:
                x_purify(ctx);
                break;
            case 24:
                x_quote(ctx);
                break;
            case 25:
                {
                    ;
                }
                break;
            case 26:
                {
                    pop_whole_stack(ctx);
                }
                break;
            case 27:
                x_substring(ctx);
                break;
            case 28:
                x_swap(ctx);
                break;
            case 29:
                x_text_length(ctx);
                break;
            case 30:
                x_text_prefix(ctx);
                break;
            case 31:
                {
                    pop_top_and_print(ctx);
                }
                break;
            case 32:
                x_type(ctx);
                break;
            case 33:
                x_warning(ctx);
                break;
            case 34:
                {
                    pop_lit_stk(ctx, &r_pop1);
                    pop_lit_stk(ctx, &r_pop2);
                    if (r_pop1.typ != 2 /*stk_fn */ )
                        print_wrong_stk_lit(hash_text, ctx, r_pop1, 2 /*stk_fn */ );
                    else if (r_pop2.typ != 2 /*stk_fn */ )
                        print_wrong_stk_lit(hash_text, ctx, r_pop2, 2 /*stk_fn */ );
                    else
                        while (true) {

                            execute_fn(ctx, r_pop2.lit);
                            pop_lit_stk(ctx, &ctx->pop1);
                            if (ctx->pop1.typ != 0 /*stk_int */ ) {
                                print_wrong_stk_lit(hash_text, ctx, ctx->pop1, 0 /*stk_int */ );
                                goto lab51;
                            } else if (ctx->pop1.lit > 0)
                                execute_fn(ctx, r_pop1.lit);
                            else
                                goto lab51;
                        }
 lab51:                        /*end_while */ ;
                }
                break;
            case 35:
                x_width(ctx);
                break;
            case 36:
                x_write(ctx);
                break;
            default:
                puts_log("Unknown built-in function");
                print_confusion();
                longjmp(error_jmpbuf, 1);
                break;
            }
        }
        break;
    case 1:
        {
            wiz_ptr = ilk_info[ex_fn_loc];
            while (wiz_functions[wiz_ptr] != end_of_def) {

                if (wiz_functions[wiz_ptr] != quote_next_fn)
                    execute_fn(ctx, wiz_functions[wiz_ptr]);
                else {

                    wiz_ptr = wiz_ptr + 1;
                    push_lit_stk(ctx, (ExecVal) { .lit = wiz_functions[wiz_ptr], .typ = STK_TYPE_FUNCTION  });
                }
                wiz_ptr = wiz_ptr + 1;
            }
        }
        break;
    case 2:
        push_lit_stk(ctx, (ExecVal) { .lit = ilk_info[ex_fn_loc], .typ = STK_TYPE_INTEGER });
        break;
    case 3:
        push_lit_stk(ctx, (ExecVal) { .lit = hash_text[ex_fn_loc], .typ = STK_TYPE_STRING });
        break;
    case 4:
        {
            if (!ctx->mess_with_entries)
                TRY(bst_cant_mess_with_entries_print(ctx));
            else {

                field_ptr = cite_ptr() * num_fields + ilk_info[ex_fn_loc];
                if (field_ptr >= max_fields) {
                    puts_log("field_info index is out of range");
                    print_confusion();
                    longjmp(error_jmpbuf, 1);
                }
                if (field_info[field_ptr] == 0 /*missing */ )
                    push_lit_stk(ctx, (ExecVal) { .lit = hash_text[ex_fn_loc], .typ = STK_TYPE_MISSING  });
                else
                    push_lit_stk(ctx, (ExecVal) { .lit = field_info[field_ptr], .typ = STK_TYPE_STRING });
            }
        }
        break;
    case 5:
        {
            if (!ctx->mess_with_entries)
                TRY(bst_cant_mess_with_entries_print(ctx));
            else
                push_lit_stk(ctx, (ExecVal) { .lit = entry_ints[cite_ptr() * num_ent_ints + ilk_info[ex_fn_loc]], .typ = STK_TYPE_INTEGER });
        }
        break;
    case 6:
        {
            if (!ctx->mess_with_entries)
                TRY(bst_cant_mess_with_entries_print(ctx));
            else {

                str_ent_ptr = cite_ptr() * num_ent_strs + ilk_info[ex_fn_loc];
                bib_set_buf_offset(BUF_TY_EX, 1, 0);
                while (entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (bib_buf_offset(BUF_TY_EX, 1))] != 127 /*end_of_string */ ) {

                    bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (bib_buf_offset(BUF_TY_EX, 1))];
                    bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                }
                bib_set_buf_len(BUF_TY_EX, bib_buf_offset(BUF_TY_EX, 1));
                add_pool_buf_and_push(ctx);
            }
        }
        break;
    case 7:
        push_lit_stk(ctx, (ExecVal) { .lit = ilk_info[ex_fn_loc], .typ = STK_TYPE_INTEGER });
        break;
    case 8:
        {
            str_glb_ptr = ilk_info[ex_fn_loc];
            if (glb_bib_str_ptr[str_glb_ptr] > 0)
                push_lit_stk(ctx, (ExecVal) { .lit = glb_bib_str_ptr[str_glb_ptr], .typ = STK_TYPE_STRING });
            else {

                {
                    while (bib_pool_ptr() + glb_str_end[str_glb_ptr] > bib_pool_size())
                        pool_overflow();
                }
                glob_chr_ptr = 0;
                while (glob_chr_ptr < glb_str_end[str_glb_ptr]) {

                    {
                        bib_set_str_pool(bib_pool_ptr(), global_strs[(str_glb_ptr) * (glob_str_size + 1) + (glob_chr_ptr)]);
                        bib_set_pool_ptr(bib_pool_ptr() + 1);
                    }
                    glob_chr_ptr = glob_chr_ptr + 1;
                }
                push_lit_stk(ctx, (ExecVal) { .lit = make_string(), .typ = STK_TYPE_STRING });
            }
        }
        break;
    default:
        unknwn_function_class_confusion();
        longjmp(error_jmpbuf, 1);
        break;
    }
}


static int
get_the_top_level_aux_file_name(const char *aux_file_name)
{
    NameAndLen nal;
    nal.name_of_file = xmalloc_array(ASCII_code, strlen(aux_file_name) + 1);
    strcpy((char *) nal.name_of_file, aux_file_name);
    int32_t aux_name_length = strlen((char *) nal.name_of_file);
    aux_name_length -= 4; /* strip off the (assumed) ".aux" for subsequent futzing */
    nal.name_length = aux_name_length;

    /* this code used to auto-add the .aux extension if needed; we don't */

    set_aux_ptr(0);
    PeekableInput* aux_file = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_TEX);
    if (aux_file == NULL) {
        sam_wrong_file_name_print(nal);
        return 1;
    }
    set_cur_aux_file(aux_file);

    add_extension(&nal, s_log_extension);
    if (init_log_file((char *) nal.name_of_file) == NULL) {
        sam_wrong_file_name_print(nal);
        return 1;
    }

    nal.name_length = aux_name_length;
    add_extension(&nal, s_bbl_extension);
    if ((bbl_file = ttstub_output_open((char *) nal.name_of_file, 0)) == NULL) {
        sam_wrong_file_name_print(nal);
        return 1;
    }

    nal.name_length = aux_name_length;
    add_extension(&nal, s_aux_extension);
    int32_t name_ptr = 0;
    while (name_ptr < nal.name_length) {
        bib_buf(BUF_TY_BASE)[name_ptr + 1] = nal.name_of_file[name_ptr]; // preserve pascal-style string semantics
        name_ptr = name_ptr + 1;
    }

    set_cur_aux(hash_text[str_lookup(bib_buf(BUF_TY_BASE), 1, nal.name_length, 3 /*aux_file_ilk*/, true)]);

    if (hash_found) {
        puts_log("Already encountered auxiliary file");
        print_confusion();
        longjmp(error_jmpbuf, 1);
    }

    free(nal.name_of_file);

    set_cur_aux_ln(0);
    return 0;
}


static void aux_bib_data_command()
{
    if (bib_seen) {
        TRY(aux_err_illegal_another_print(0 /*n_aux_bibdata */ ));
        {
            TRY(aux_err_print());
            return;
        }
    }
    bib_seen = true;
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!scan2_white(125 /*right_brace */ , 44 /*comma */)) {
            aux_err_no_right_brace_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
            aux_err_white_space_in_argument_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        if ((bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) && (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ )) {
            aux_err_stuff_after_right_brace_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        {
            check_bib_files(bib_ptr());

            set_cur_bib(hash_text[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 6 /*bib_file_ilk */ , true)]);
            if (hash_found) {
                puts_log("This database file appears more than once: ");
                TRY(print_bib_name());
                TRY(aux_err_print());
                return;
            }
            NameAndLen nal = start_name(cur_bib());
            PeekableInput* bib_in = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_BIB);
            if (bib_in == NULL) {
                puts_log("I couldn't open database file ");
                TRY(print_bib_name());
                TRY(aux_err_print());
                free(nal.name_of_file);
                return;
            }
            set_cur_bib_file(bib_in);
            free(nal.name_of_file);

            set_bib_ptr(bib_ptr() + 1);
        }
    }
}

static void aux_bib_style_command(BstCtx* ctx)
{
    if (bst_seen) {
        TRY(aux_err_illegal_another_print(1 /*n_aux_bibstyle */ ));
        {
            TRY(aux_err_print());
            return;
        }
    }
    bst_seen = true;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!scan1_white(125 /*right_brace */)) {
        aux_err_no_right_brace_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
        aux_err_white_space_in_argument_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    if (bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) {
        aux_err_stuff_after_right_brace_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    {
        ctx->bst_str = hash_text[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 5 /*bst_file_ilk */ , true)];
        if (hash_found) {
            puts_log("Already encountered style file");
            print_confusion();
            longjmp(error_jmpbuf, 1);
        }
        NameAndLen nal = start_name(ctx->bst_str);
        if ((ctx->bst_file = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_BST)) == NULL) {
            puts_log("I couldn't open style file ");
            print_bst_name(ctx);
            ctx->bst_str = 0;
            TRY(aux_err_print());
            free(nal.name_of_file);
            return;
        }
        free(nal.name_of_file);
        if (verbose) {
            puts_log("The style file: ");
            print_bst_name(ctx);
        } else {
            ttstub_puts (bib_log_file(), "The style file: ");
            TRY(log_pr_bst_name(ctx));
        }
    }
}

static void aux_citation_command(void)
{
    buf_pointer tmp_ptr;

    citation_seen = true;
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        if (!scan2_white(125 /*right_brace */ , 44 /*comma */)) {
            aux_err_no_right_brace_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
            aux_err_white_space_in_argument_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        if ((bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) && (bib_buf_at_offset(BUF_TY_BASE, 2) == 125 /*right_brace */ )) {
            aux_err_stuff_after_right_brace_print();
            {
                TRY(aux_err_print());
                return;
            }
        }
        {
            {
                if ((bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)) == 1) {

                    if (bib_buf_at_offset(BUF_TY_BASE, 1) == 42 /*star */ ) {
                        if (all_entries) {
                            puts_log("Multiple inclusions of entire database\n");
                            TRY(aux_err_print());
                            return;
                        } else {

                            all_entries = true;
                            all_marker = cite_ptr();
                            goto lab23;
                        }
                    }
                }
            }
            tmp_ptr = bib_buf_offset(BUF_TY_BASE, 1);
            while (tmp_ptr < bib_buf_offset(BUF_TY_BASE, 2)) {

                bib_buf(BUF_TY_EX)[tmp_ptr] = bib_buf_at(BUF_TY_BASE, tmp_ptr);
                tmp_ptr = tmp_ptr + 1;
            }
            lower_case(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            lc_cite_loc = str_lookup(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 10 /*lc_cite_ilk */ , true);
            if (hash_found) { /*136: */
                ;

                str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 9 /*cite_ilk */ , false);
                if (!hash_found) {
                    puts_log("Case mismatch error between cite keys ");
                    print_a_token();
                    puts_log(" and ");
                    TRY(print_a_pool_str(cite_list(ilk_info[ilk_info[lc_cite_loc]])));
                    putc_log('\n');
                    TRY(aux_err_print());
                    return;
                }
            } else {            /*137: */

                ;

                cite_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 9 /*cite_ilk */ , true);
                if (hash_found) {
                    hash_cite_confusion();
                    longjmp(error_jmpbuf, 1);
                }
                check_cite_overflow(cite_ptr());
                set_cite_list(cite_ptr(), hash_text[cite_loc]);
                ilk_info[cite_loc] = cite_ptr();
                ilk_info[lc_cite_loc] = cite_loc;
                set_cite_ptr(cite_ptr() + 1);
            }
        }
 lab23:                        /*next_cite */ ;
    }
}

static void aux_input_command(void)
{
    bool aux_extension_ok;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    if (!scan1_white(125 /*right_brace */)) {
        aux_err_no_right_brace_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    if (LEX_CLASS[bib_buf_at_offset(BUF_TY_BASE, 2)] == LEX_CLASS_WHITESPACE ) {
        aux_err_white_space_in_argument_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    if (bib_buf_len(BUF_TY_BASE) > bib_buf_offset(BUF_TY_BASE, 2) + 1) {
        aux_err_stuff_after_right_brace_print();
        {
            TRY(aux_err_print());
            return;
        }
    }
    {
        set_aux_ptr(aux_ptr() + 1);
        if (aux_ptr() == aux_stack_size) {
            print_a_token();
            puts_log(": ");
            print_overflow();
            printf_log("auxiliary file depth %ld\n", (long) aux_stack_size);
            longjmp(error_jmpbuf, 1);
        }
        aux_extension_ok = true;
        if ((bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)) < (bib_str_start(s_aux_extension + 1) - bib_str_start(s_aux_extension)))
            aux_extension_ok = false;
        else if (!bib_str_eq_buf(
                s_aux_extension,
                bib_buf(BUF_TY_BASE),
                bib_buf_offset(BUF_TY_BASE, 2) - (bib_str_start(s_aux_extension + 1) - bib_str_start(s_aux_extension)),
                (bib_str_start(s_aux_extension + 1) - bib_str_start(s_aux_extension))
                ))
            aux_extension_ok = false;
        if (!aux_extension_ok) {
            print_a_token();
            puts_log(" has a wrong extension");
            set_aux_ptr(aux_ptr() - 1);
            TRY(aux_err_print());
            return;
        }
        set_cur_aux(hash_text[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 3 /*aux_file_ilk */ , true)]);
        if (hash_found) {
            puts_log("Already encountered file ");
            TRY(print_aux_name());
            set_aux_ptr(aux_ptr() - 1);
            TRY(aux_err_print());
            return;
        }
        {
            NameAndLen nal = start_name(cur_aux());
            int32_t name_ptr = nal.name_length;
            nal.name_of_file[name_ptr] = 0;
            PeekableInput* aux_file = peekable_open ((char *) nal.name_of_file, TTBC_FILE_FORMAT_TEX);
            if (aux_file == NULL) {
                puts_log("I couldn't open auxiliary file ");
                TRY(print_aux_name());
                set_aux_ptr(aux_ptr() - 1);
                TRY(aux_err_print());
                free(nal.name_of_file);
                return;
            }
            set_cur_aux_file(aux_file);
            free(nal.name_of_file);

            printf_log("A level-%ld auxiliary file: ", (long) aux_ptr());
            TRY(log_pr_aux_name());
            set_cur_aux_ln(0);
        }
    }
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

static void get_aux_command_and_process(BstCtx* ctx)
{
    bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    if (!scan1(123 /*left_brace */))
        return;
    command_num = ilk_info[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 2 /*aux_command_ilk */ , false)];
    if (hash_found)
        switch ((command_num)) {
        case 0:
            aux_bib_data_command(aux_ptr);
            break;
        case 1:
            aux_bib_style_command(ctx);
            break;
        case 2:
            aux_citation_command();
            break;
        case 3:
            aux_input_command();
            break;
        default:
            puts_log("Unknown auxiliary-file command");
            print_confusion();
            longjmp(error_jmpbuf, 1);
            break;
        }
}

static void last_check_for_aux_errors(BstCtx* ctx)
{
    num_cites = cite_ptr();
    ctx->num_bib_files = bib_ptr();
    if (!citation_seen) {
        aux_end1_err_print();
        puts_log("\\citation commands");
        TRY(aux_end2_err_print());
    } else if ((num_cites == 0) && (!all_entries)) {
        aux_end1_err_print();
        puts_log("cite keys");
        TRY(aux_end2_err_print());
    }
    if (!bib_seen) {
        aux_end1_err_print();
        puts_log("\\bibdata command");
        TRY(aux_end2_err_print());
    } else if (ctx->num_bib_files == 0) {
        aux_end1_err_print();
        puts_log("database files");
        TRY(aux_end2_err_print());
    }
    if (!bst_seen) {
        aux_end1_err_print();
        puts_log("\\bibstyle command");
        TRY(aux_end2_err_print());
    } else if (ctx->bst_str == 0) {
        aux_end1_err_print();
        puts_log("style file");
        TRY(aux_end2_err_print());
    }
}

static void bst_entry_command(BstCtx* ctx)
{
    if (entry_seen) {
        puts_log("Illegal, another entry command");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }
    entry_seen = true;
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

            {
                ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
                if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
                else {
                    TRY(bst_id_print(scan_result));
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
            {
                ;

                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , true);
                {
                    if (hash_found) {
                        already_seen_function_print(ctx, fn_loc);
                        return;
                    }
                }
                fn_type[fn_loc] = 4 /*field */ ;
                ilk_info[fn_loc] = num_fields;
                num_fields = num_fields + 1;
            }
            {
                if (!eat_bst_white_space(ctx)) {
                    eat_bst_print();
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    if (num_fields == num_pre_defined_fields) {
        puts_log("Warning--I didn't find any fields");
        TRY(bst_warn_print(ctx));
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

            {
                ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
                if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
                else {
                    TRY(bst_id_print(scan_result));
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
            {
                ;

                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , true);
                {
                    if (hash_found) {
                        already_seen_function_print(ctx, fn_loc);
                        return;
                    }
                }
                fn_type[fn_loc] = 5 /*int_entry_var */ ;
                ilk_info[fn_loc] = num_ent_ints;
                num_ent_ints = num_ent_ints + 1;
            }
            {
                if (!eat_bst_white_space(ctx)) {
                    eat_bst_print();
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("entry");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("entry");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

            {
                ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
                if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
                else {
                    TRY(bst_id_print(scan_result));
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
            {
                ;

                lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , true);
                {
                    if (hash_found) {
                        already_seen_function_print(ctx, fn_loc);
                        return;
                    }
                }
                fn_type[fn_loc] = 6 /*str_entry_var */ ;
                ilk_info[fn_loc] = num_ent_strs;
                num_ent_strs = num_ent_strs + 1;
            }
            {
                if (!eat_bst_white_space(ctx)) {
                    eat_bst_print();
                    puts_log("entry");
                    bst_err_print_and_look_for_blank_line(ctx);
                    return;
                }
            }
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
}

static bool bad_argument_token(BstCtx* ctx)
{
    lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , false);
    if (!hash_found) {
        print_a_token();
        puts_log(" is an unknown function");
        bst_err_print_and_look_for_blank_line(ctx);
        return true;
    } else if ((fn_type[fn_loc] != 0 /*built_in */ ) && (fn_type[fn_loc] != 1 /*wiz_defined */ )) {
        print_a_token();
        puts_log(" has bad function type ");
        print_fn_class(fn_loc);
        bst_err_print_and_look_for_blank_line(ctx);
        return true;
    }
    return false;
}

static void bst_execute_command(ExecCtx* ctx)
{
    if (!read_seen) {
        puts_log("Illegal, execute command before read command");
        bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
        return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
            bst_left_brace_print();
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            TRY(bst_id_print(scan_result));
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ;

        if (bad_argument_token(ctx->bst_ctx))
            return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
            bst_right_brace_print();
            puts_log("execute");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        init_command_execution(ctx);
        ctx->mess_with_entries = false;
        execute_fn(ctx, fn_loc);
        check_command_execution(ctx);
    }
}

static void bst_function_command(BstCtx* ctx)
{
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("function");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
            if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
            else {
                TRY(bst_id_print(scan_result));
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            ;

            lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            wiz_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , true);
            {
                if (hash_found) {
                    already_seen_function_print(ctx, wiz_loc);
                    return;
                }
            }
            fn_type[wiz_loc] = 1 /*wiz_defined */ ;
            if (hash_text[wiz_loc] == s_default)
                b_default = wiz_loc;
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
                bst_right_brace_print();
                puts_log("function");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("function");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
            bst_left_brace_print();
            puts_log("function");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    scan_fn_def(ctx, wiz_loc);
}

static void bst_integers_command(BstCtx* ctx)
{
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("integers");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
            bst_left_brace_print();
            puts_log("integers");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("integers");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {

        {
            ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
            if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
            else {
                TRY(bst_id_print(scan_result));
                puts_log("integers");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            ;

            lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , true);
            {
                if (hash_found) {
                    already_seen_function_print(ctx, fn_loc);
                    return;
                }
            }
            fn_type[fn_loc] = 7 /*int_global_var */ ;
            ilk_info[fn_loc] = 0;
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("integers");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
}

static void bst_iterate_command(ExecCtx* ctx)
{
    if (!read_seen) {
        puts_log("Illegal, iterate command before read command");
        bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
        return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
            bst_left_brace_print();
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            TRY(bst_id_print(scan_result));
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ;

        if (bad_argument_token(ctx->bst_ctx))
            return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
            bst_right_brace_print();
            puts_log("iterate");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        init_command_execution(ctx);
        ctx->mess_with_entries = true;
        sort_cite_ptr = 0;
        while (sort_cite_ptr < num_cites) {

            set_cite_ptr(cite_info(sort_cite_ptr));

            execute_fn(ctx, fn_loc);
            check_command_execution(ctx);
            sort_cite_ptr = sort_cite_ptr + 1;
        }
    }
}

static void bst_macro_command(BstCtx* ctx)
{
    if (read_seen) {
        puts_log("Illegal, macro command after read command");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("macro");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
            if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
            else {
                TRY(bst_id_print(scan_result));
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            ;

            lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            macro_name_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 13 /*macro_ilk */ , true);
            if (hash_found) {
                print_a_token();
                puts_log(" is already defined as a macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            ilk_info[macro_name_loc] = hash_text[macro_name_loc];
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
                bst_right_brace_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
    }
    {
        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("macro");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
    }
    {
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
                bst_left_brace_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 34 /*double_quote */ ) {
            puts_log("A macro definition must be \"-delimited");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }
        {
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            if (!scan1(34 /*double_quote */)) {
                puts_log("There's no `\"' to end macro definition");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }

            macro_def_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 0 /*text_ilk */ , true);
            fn_type[macro_def_loc] = 3 /*str_literal */ ;
            ilk_info[macro_name_loc] = hash_text[macro_def_loc];
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
        {
            if (!eat_bst_white_space(ctx)) {
                eat_bst_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
        }
        {
            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
                bst_right_brace_print();
                puts_log("macro");
                bst_err_print_and_look_for_blank_line(ctx);
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        }
    }
}

static void get_bib_command_or_entry_and_process(void)
{
    // WAS: looking for at_bib_command uses and making them local
    buf_pointer tmp_ptr, tmp_end_ptr;

    bool at_bib_command = false;
    while (!scan1(64 /*at_sign */)) {

        if (!input_ln(cur_bib_file()))
            return;
        set_bib_line_num(bib_line_num() + 1);
        bib_set_buf_offset(BUF_TY_BASE, 2, 0);
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 64 /*at_sign */ ) {
            puts_log("An \"@\" disappeared");
            print_confusion();
            longjmp(error_jmpbuf, 1);
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        {
            if (!eat_bib_white_space()) {
                TRY(eat_bib_print(at_bib_command));
                return;
            }
        }
        ScanRes scan_result = scan_identifier(123 /*left_brace */ , 40 /*left_paren */ , 40 /*left_paren */);
        {
            if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
            else {
                TRY(bib_id_print(scan_result));
                puts_log("an entry type");
                TRY(bib_err_print(at_bib_command));
                return;
            }
        }
        ;

        lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
        command_num = ilk_info[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 12 /*bib_command_ilk */ , false)];
        if (hash_found) {     /*240: */
            at_bib_command = true;
            switch ((command_num)) {
            case 0:
                {
                    return;
                }
                break;
            case 1:
                {
                    check_bib_files(preamble_ptr());
                    if (!eat_bib_white_space()) {
                        TRY(eat_bib_print(at_bib_command));
                        return;
                    }
                    if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
                        right_outer_delim = 125 /*right_brace */ ;
                    else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
                        right_outer_delim = 41 /*right_paren */ ;
                    else {
                        TRY(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
                        return;
                    }
                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                    {
                        if (!eat_bib_white_space()) {
                            TRY(eat_bib_print(at_bib_command));
                            return;
                        }
                    }
                    store_field = true;
                    if (!scan_and_store_the_field_value_and_eat_white(at_bib_command))
                        return;
                    if (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {
                        printf_log("Missing \"%c\" in preamble command", right_outer_delim);
                        TRY(bib_err_print(at_bib_command));
                        return;
                    }
                    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                    return;
                }
                break;
            case 2:
                {
                    {
                        if (!eat_bib_white_space()) {
                            TRY(eat_bib_print(at_bib_command));
                            return;
                        }
                    }
                    {
                        if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
                            right_outer_delim = 125 /*right_brace */ ;
                        else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
                            right_outer_delim = 41 /*right_paren */ ;
                        else {

                            TRY(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
                            return;
                        }
                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                        {
                            if (!eat_bib_white_space()) {
                                TRY(eat_bib_print(at_bib_command));
                                return;
                            }
                        };
                        scan_result = scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */);
                        {
                            if (((scan_result == SCAN_RES_WHITESPACE_ADJACENT)
                                 || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT))) ;
                            else {
                                TRY(bib_id_print(scan_result));
                                puts_log("a string name");
                                TRY(bib_err_print(at_bib_command));
                                return;
                            }
                        }
                        {
                            ;

                            lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                            cur_macro_loc =
                                str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 13 /*macro_ilk */ , true);
                            ilk_info[cur_macro_loc] = hash_text[cur_macro_loc];
                        }
                    }
                    {
                        if (!eat_bib_white_space()) {
                            TRY(eat_bib_print(at_bib_command));
                            return;
                        }
                    }
                    {
                        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 61 /*equals_sign */ ) {
                            TRY(bib_equals_sign_print(at_bib_command));
                            return;
                        }
                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                        {
                            if (!eat_bib_white_space()) {
                                TRY(eat_bib_print(at_bib_command));
                                return;
                            }
                        }
                        store_field = true;
                        if (!scan_and_store_the_field_value_and_eat_white(at_bib_command))
                            return;
                        if (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {
                            printf_log("Missing \"%c\" in string command", right_outer_delim);
                            TRY(bib_err_print(at_bib_command));
                            return;
                        }
                        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
                    }
                    return;
                }
                break;
            default:
                bib_cmd_confusion();
                longjmp(error_jmpbuf, 1);
                break;
            }
        } else {

            entry_type_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , false);
            if ((!hash_found) || (fn_type[entry_type_loc] != 1 /*wiz_defined */ ))
                type_exists = false;
            else
                type_exists = true;
        }
    }
    {
        if (!eat_bib_white_space()) {
            TRY(eat_bib_print(at_bib_command));
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) == 123 /*left_brace */ )
            right_outer_delim = 125 /*right_brace */ ;
        else if (bib_buf_at_offset(BUF_TY_BASE, 2) == 40 /*left_paren */ )
            right_outer_delim = 41 /*right_paren */ ;
        else {

            TRY(bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */, at_bib_command));
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
        {
            if (!eat_bib_white_space()) {
                TRY(eat_bib_print(at_bib_command));
                return;
            }
        }
        if (right_outer_delim == 41 /*right_paren */ ) {
            scan1_white(44 /*comma */);
        } else {
            scan2_white(44 /*comma */ , 125 /*right_brace */);
        }

        {
            ;

            tmp_ptr = bib_buf_offset(BUF_TY_BASE, 1);
            while (tmp_ptr < bib_buf_offset(BUF_TY_BASE, 2)) {

                bib_buf(BUF_TY_EX)[tmp_ptr] = bib_buf_at(BUF_TY_BASE, tmp_ptr);
                tmp_ptr = tmp_ptr + 1;
            }
            lower_case(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
            if (all_entries)
                lc_cite_loc = str_lookup(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 10 /*lc_cite_ilk */ , true);
            else
                lc_cite_loc = str_lookup(bib_buf(BUF_TY_EX), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 10 /*lc_cite_ilk */ , false);
            if (hash_found) {
                entry_cite_ptr = ilk_info[ilk_info[lc_cite_loc]];
                {
                    if ((!all_entries) || (entry_cite_ptr < all_marker) || (entry_cite_ptr >= old_num_cites)) {
                        if (type_list(entry_cite_ptr) == 0 /*empty */ ) {
                            {
                                if ((!all_entries) && (entry_cite_ptr >= old_num_cites)) {
                                    cite_loc =
                                        str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 9 /*cite_ilk */ , true);
                                    if (!hash_found) {
                                        ilk_info[lc_cite_loc] = cite_loc;
                                        ilk_info[cite_loc] = entry_cite_ptr;
                                        set_cite_list(entry_cite_ptr, hash_text[cite_loc]);
                                        hash_found = true;
                                    }
                                }
                            }
                            goto lab26;
                        }
                    } else if (!entry_exists(entry_cite_ptr)) {
                        {
                            bib_set_buf_offset(BUF_TY_EX, 1, 0);
                            tmp_ptr = bib_str_start(cite_info(entry_cite_ptr));
                            tmp_end_ptr = bib_str_start(cite_info(entry_cite_ptr) + 1);
                            while (tmp_ptr < tmp_end_ptr) {

                                bib_buf(BUF_TY_EX)[bib_buf_offset(BUF_TY_EX, 1)] = bib_str_pool(tmp_ptr);
                                bib_set_buf_offset(BUF_TY_EX, 1, bib_buf_offset(BUF_TY_EX, 1) + 1);
                                tmp_ptr = tmp_ptr + 1;
                            }
                            lower_case(bib_buf(BUF_TY_EX), 0,
                                       (bib_str_start(cite_info(entry_cite_ptr) + 1) -
                                        bib_str_start(cite_info(entry_cite_ptr))));
                            lc_xcite_loc =
                                str_lookup(bib_buf(BUF_TY_EX), 0,
                                           (bib_str_start(cite_info(entry_cite_ptr) + 1) -
                                            bib_str_start(cite_info(entry_cite_ptr))), 10 /*lc_cite_ilk */ , false);
                            if (!hash_found) {
                                cite_key_disappeared_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                        }
                        if (lc_xcite_loc == lc_cite_loc)
                            goto lab26;
                    }
                    if (type_list(entry_cite_ptr) == 0 /*empty */ ) {
                        puts_log("The cite list is messed up");
                        print_confusion();
                        longjmp(error_jmpbuf, 1);
                    }

                    puts_log("Repeated entry");
                    TRY(bib_err_print(at_bib_command));
                    return;
 lab26:                        /*first_time_entry */ ;
                }
            }
            store_entry = true;
            if (all_entries) {        /*273: */
                if (hash_found) {
                    if (entry_cite_ptr < all_marker)
                        goto lab22;
                    else {

                        set_entry_exists(entry_cite_ptr, true);
                        cite_loc = ilk_info[lc_cite_loc];
                    }
                } else {

                    cite_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 9 /*cite_ilk */ , true);
                    if (hash_found) {
                        hash_cite_confusion();
                        longjmp(error_jmpbuf, 1);
                    }
                }
                entry_cite_ptr = cite_ptr();
                set_cite_ptr(add_database_cite(cite_ptr()));
 lab22:                        /*cite_already_set */ ;
            } else if (!hash_found)
                store_entry = false;
            if (store_entry) {        /*274: */
                if (type_exists)
                    set_type_list(entry_cite_ptr, entry_type_loc);
                else {
                    set_type_list(entry_cite_ptr, undefined);
                    puts_log("Warning--entry type for \"");
                    print_a_token();
                    puts_log("\" isn't style-file defined\n");
                    TRY(bib_warn_print());
                }
            }
        }
    }
    {
        if (!eat_bib_white_space()) {
            TRY(eat_bib_print(at_bib_command));
            return;
        }
    }
    {
        while (bib_buf_at_offset(BUF_TY_BASE, 2) != right_outer_delim) {

            if (bib_buf_at_offset(BUF_TY_BASE, 2) != 44 /*comma */ ) {
                TRY(bib_one_of_two_print(44 /*comma */ , right_outer_delim, at_bib_command));
                return;
            }
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            {
                if (!eat_bib_white_space()) {
                    TRY(eat_bib_print(at_bib_command));
                    return;
                }
            }
            if (bib_buf_at_offset(BUF_TY_BASE, 2) == right_outer_delim)
                goto loop_exit;
            {
                ScanRes scan_result = scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */);
                {
                    if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
                    else {
                        TRY(bib_id_print(scan_result));
                        puts_log("a field name");
                        TRY(bib_err_print(at_bib_command));
                        return;
                    }
                }
                ;

                store_field = false;
                if (store_entry) {
                    lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
                    field_name_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 11 /*bst_fn_ilk */ , false);
                    if (hash_found) {

                        if (fn_type[field_name_loc] == 4 /*field */ )
                            store_field = true;
                    }
                }
                {
                    if (!eat_bib_white_space()) {
                        TRY(eat_bib_print(at_bib_command));
                        return;
                    }
                }
                if (bib_buf_at_offset(BUF_TY_BASE, 2) != 61 /*equals_sign */ ) {
                    TRY(bib_equals_sign_print(at_bib_command));
                    return;
                }
                bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
            }
            {
                if (!eat_bib_white_space()) {
                    TRY(eat_bib_print(at_bib_command));
                    return;
                }
            }
            if (!scan_and_store_the_field_value_and_eat_white(at_bib_command))
                return;
        }
 loop_exit:
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
}

static void bst_read_command(BstCtx* ctx)
{
    buf_pointer tmp_ptr;

    if (read_seen) {
        puts_log("Illegal, another read command");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }
    read_seen = true;
    if (!entry_seen) {
        puts_log("Illegal, read command before entry command");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }
    bib_set_buf_offset(BUF_TY_SV, 1, bib_buf_offset(BUF_TY_BASE, 2));
    bib_set_buf_offset(BUF_TY_SV, 2, bib_buf_len(BUF_TY_BASE));
    tmp_ptr = bib_buf_offset(BUF_TY_SV, 1);
    while (tmp_ptr < bib_buf_offset(BUF_TY_SV, 2)) {

        bib_buf(BUF_TY_SV)[tmp_ptr] = bib_buf_at(BUF_TY_BASE, tmp_ptr);
        tmp_ptr = tmp_ptr + 1;
    }
    {
        {
            {
                check_field_overflow(num_fields * num_cites);
                field_ptr = 0;
                while (field_ptr < max_fields) {

                    field_info[field_ptr] = 0 /*missing */ ;
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
                old_num_cites = num_cites;
                if (all_entries) {
                    set_cite_ptr(all_marker);
                    while (cite_ptr() < old_num_cites) {
                        set_cite_info(cite_ptr(), cite_list(cite_ptr()));
                        set_entry_exists(cite_ptr(), false);
                        set_cite_ptr(cite_ptr() + 1);
                    }
                    set_cite_ptr(all_marker);
                } else {

                    set_cite_ptr(num_cites);
                    all_marker = 0 /*any_value */ ;
                }
            }
        }
        read_performed = true;
        set_bib_ptr(0);
        while (bib_ptr() < ctx->num_bib_files) {
            if (verbose) {
                printf_log("Database file #%ld: ", (long) bib_ptr() + 1);
                TRY(print_bib_name());
            } else {
                char buf[512];
                snprintf(buf, sizeof(buf) - 1, "Database file #%ld: ", (long) bib_ptr() + 1);
                ttstub_output_write (bib_log_file(), buf, strlen(buf));
                TRY(log_pr_bib_name());
            }
            set_bib_line_num(0);
            bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_len(BUF_TY_BASE));
            while (!eof(cur_bib_file()))
                get_bib_command_or_entry_and_process();
            peekable_close(cur_bib_file());
            set_cur_bib_file(NULL);
            set_bib_ptr(bib_ptr() + 1);
        }
        reading_completed = true;
        ;

        {
            num_cites = cite_ptr();
            ctx->num_preamble_strings = preamble_ptr();
            {
                if ((num_cites - 1) * num_fields + crossref_num >= max_fields) {
                    puts_log("field_info index is out of range");
                    print_confusion();
                    longjmp(error_jmpbuf, 1);
                }
                set_cite_ptr(0);
                while (cite_ptr() < num_cites) {

                    field_ptr = cite_ptr() * num_fields + crossref_num;
                    if (field_info[field_ptr] != 0 /*missing */ ) {

                        if (find_cite_locs_for_this_cite_key(field_info[field_ptr])) {
                            cite_loc = ilk_info[lc_cite_loc];
                            field_info[field_ptr] = hash_text[cite_loc];
                            cite_parent_ptr = ilk_info[cite_loc];
                            field_ptr = cite_ptr() * num_fields + num_pre_defined_fields;
                            field_end_ptr = field_ptr - num_pre_defined_fields + num_fields;
                            field_parent_ptr = cite_parent_ptr * num_fields + num_pre_defined_fields;
                            while (field_ptr < field_end_ptr) {

                                if (field_info[field_ptr] == 0 /*missing */ )
                                    field_info[field_ptr] = field_info[field_parent_ptr];
                                field_ptr = field_ptr + 1;
                                field_parent_ptr = field_parent_ptr + 1;
                            }
                        }
                    }
                    set_cite_ptr(cite_ptr() + 1);
                }
            }
            {
                if ((num_cites - 1) * num_fields + crossref_num >= max_fields) {
                    puts_log("field_info index is out of range");
                    print_confusion();
                    longjmp(error_jmpbuf, 1);
                }
                set_cite_ptr(0);
                while (cite_ptr() < num_cites) {

                    field_ptr = cite_ptr() * num_fields + crossref_num;
                    if (field_info[field_ptr] != 0 /*missing */ ) {

                        if (!find_cite_locs_for_this_cite_key(field_info[field_ptr])) {
                            if (cite_hash_found) {
                                hash_cite_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                            nonexistent_cross_reference_error();
                            field_info[field_ptr] = 0 /*missing */ ;
                        } else {

                            if (cite_loc != ilk_info[lc_cite_loc]) {
                                hash_cite_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                            cite_parent_ptr = ilk_info[cite_loc];
                            if (type_list(cite_parent_ptr) == 0 /*empty */ ) {
                                nonexistent_cross_reference_error();
                                field_info[field_ptr] = 0 /*missing */ ;
                            } else {

                                field_parent_ptr = cite_parent_ptr * num_fields + crossref_num;
                                if (field_info[field_parent_ptr] != 0 /*missing */ ) {        /*283: */
                                    puts_log("Warning--you've nested cross references");
                                    TRY(bad_cross_reference_print(cite_list(cite_parent_ptr)));
                                    puts_log("\", which also refers to something\n");
                                    mark_warning();
                                }
                                if (((!all_entries) && (cite_parent_ptr >= old_num_cites)
                                     && (cite_info(cite_parent_ptr) < bibtex_config->min_crossrefs)))
                                    field_info[field_ptr] = 0 /*missing */ ;
                            }
                        }
                    }
                    set_cite_ptr(cite_ptr() + 1);
                }
            }
            {
                set_cite_ptr(0);
                while (cite_ptr() < num_cites) {

                    if (type_list(cite_ptr()) == 0 /*empty */ )
                        TRY(print_missing_entry(cite_list(cite_ptr())));
                    else if ((all_entries) || (cite_ptr() < old_num_cites) || (cite_info(cite_ptr()) >= bibtex_config->min_crossrefs)) {
                        if (cite_ptr() > cite_xptr) {   /*286: */
                            if ((cite_xptr + 1) * num_fields > max_fields) {
                                puts_log("field_info index is out of range");
                                print_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                            set_cite_list(cite_xptr, cite_list(cite_ptr()));
                            set_type_list(cite_xptr, type_list(cite_ptr()));
                            if (!find_cite_locs_for_this_cite_key(cite_list(cite_ptr()))) {
                                cite_key_disappeared_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                            if ((!cite_hash_found) || (cite_loc != ilk_info[lc_cite_loc])) {
                                hash_cite_confusion();
                                longjmp(error_jmpbuf, 1);
                            }
                            ilk_info[cite_loc] = cite_xptr;
                            field_ptr = cite_xptr * num_fields;
                            field_end_ptr = field_ptr + num_fields;
                            tmp_ptr = cite_ptr() * num_fields;
                            while (field_ptr < field_end_ptr) {

                                field_info[field_ptr] = field_info[tmp_ptr];
                                field_ptr = field_ptr + 1;
                                tmp_ptr = tmp_ptr + 1;
                            }
                        }
                        cite_xptr = cite_xptr + 1;
                    }
                    set_cite_ptr(cite_ptr() + 1);
                }
                num_cites = cite_xptr;
                if (all_entries) {    /*287: */
                    set_cite_ptr(all_marker);
                    while (cite_ptr() < old_num_cites) {

                        if (!entry_exists(cite_ptr()))
                            TRY(print_missing_entry(cite_info(cite_ptr())));
                        set_cite_ptr(cite_ptr() + 1);
                    }
                }
            }
            {
                entry_ints = XTALLOC((num_ent_ints + 1) * (num_cites + 1), int32_t);
                int_ent_ptr = 0;
                while (int_ent_ptr < num_ent_ints * num_cites) {

                    entry_ints[int_ent_ptr] = 0;
                    int_ent_ptr = int_ent_ptr + 1;
                }
            }
            {
                entry_strs = XTALLOC((num_ent_strs + 1) * (num_cites + 1) * (ent_str_size + 1), ASCII_code);
                str_ent_ptr = 0;
                while (str_ent_ptr < num_ent_strs * num_cites) {

                    entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (0)] = 127 /*end_of_string */ ;
                    str_ent_ptr = str_ent_ptr + 1;
                }
            }
            {
                set_cite_ptr(0);
                while (cite_ptr() < num_cites) {
                    set_cite_info(cite_ptr(), cite_ptr());
                    set_cite_ptr(cite_ptr() + 1);
                }
            }
        }
    }
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_SV, 1));
    bib_set_buf_len(BUF_TY_BASE, bib_buf_offset(BUF_TY_SV, 2));
    tmp_ptr = bib_buf_offset(BUF_TY_BASE, 2);
    while (tmp_ptr < bib_buf_len(BUF_TY_BASE)) {

        bib_buf(BUF_TY_BASE)[tmp_ptr] = bib_buf_at(BUF_TY_SV, tmp_ptr);
        tmp_ptr = tmp_ptr + 1;
    }
}

static void bst_reverse_command(ExecCtx* ctx)
{
    if (!read_seen) {
        puts_log("Illegal, reverse command before read command");
        bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
        return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
            bst_left_brace_print();
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if ((scan_result == SCAN_RES_WHITESPACE_ADJACENT) || (scan_result == SCAN_RES_SPECIFIED_CHAR_ADJACENT)) ;
        else {
            TRY(bst_id_print(scan_result));
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        ;

        if (bad_argument_token(ctx->bst_ctx))
            return;
    }
    {
        if (!eat_bst_white_space(ctx->bst_ctx)) {
            eat_bst_print();
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
    }
    {
        if (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
            bst_right_brace_print();
            puts_log("reverse");
            bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
            return;
        }
        bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);
    }
    {
        init_command_execution(ctx);
        ctx->mess_with_entries = true;
        if (num_cites > 0) {
            sort_cite_ptr = num_cites;
            do {
                sort_cite_ptr = sort_cite_ptr - 1;
                set_cite_ptr(cite_info(sort_cite_ptr));
                ;

                execute_fn(ctx, fn_loc);
                check_command_execution(ctx);
            } while (!((sort_cite_ptr == 0)));
        }
    }
}


static void
bst_sort_command(BstCtx* ctx)
{
    if (!read_seen) {
        puts_log("Illegal, sort command before read command");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }

    if (num_cites > 1)
        quick_sort(0, num_cites - 1);
}


static void
bst_strings_command(BstCtx* ctx)
{
    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("strings");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }

    if (bib_buf_at_offset(BUF_TY_BASE, 2) != 123 /*left_brace */ ) {
        bst_left_brace_print();
        puts_log("strings");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }

    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_offset(BUF_TY_BASE, 2) + 1);

    if (!eat_bst_white_space(ctx)) {
        eat_bst_print();
        puts_log("strings");
        bst_err_print_and_look_for_blank_line(ctx);
        return;
    }

    while (bib_buf_at_offset(BUF_TY_BASE, 2) != 125 /*right_brace */ ) {
        ScanRes scan_result = scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */);
        if (scan_result != SCAN_RES_WHITESPACE_ADJACENT && scan_result != SCAN_RES_SPECIFIED_CHAR_ADJACENT) {
            TRY(bst_id_print(scan_result));
            puts_log("strings");
            bst_err_print_and_look_for_blank_line(ctx);
            return;
        }

        lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1));
        fn_loc = str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1), 11 /*bst_fn_ilk */ , true);
        if (hash_found) {
            already_seen_function_print(ctx, fn_loc);
            return;
        }

        fn_type[fn_loc] = 8 /*str_global_var */ ;
        ilk_info[fn_loc] = num_glb_strs;

        if (num_glb_strs == max_glob_strs) {
            BIB_XRETALLOC_NOSET("glb_bib_str_ptr", glb_bib_str_ptr, str_number, max_glob_strs,
                                max_glob_strs + MAX_GLOB_STRS);
            BIB_XRETALLOC_STRING("global_strs", global_strs, glob_str_size, max_glob_strs,
                                 max_glob_strs + MAX_GLOB_STRS);
            BIB_XRETALLOC("glb_str_end", glb_str_end, int32_t, max_glob_strs, max_glob_strs + MAX_GLOB_STRS);
            str_glb_ptr = num_glb_strs;

            while (str_glb_ptr < max_glob_strs) {
                glb_bib_str_ptr[str_glb_ptr] = 0;
                glb_str_end[str_glb_ptr] = 0;
                str_glb_ptr = str_glb_ptr + 1;
            }
        }

        num_glb_strs++;

        if (!eat_bst_white_space(ctx)) {
            eat_bst_print();
            puts_log("strings");
            bst_err_print_and_look_for_blank_line(ctx);
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
        bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
        return;
    }

    lower_case(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)));
    command_num = ilk_info[str_lookup(bib_buf(BUF_TY_BASE), bib_buf_offset(BUF_TY_BASE, 1), (bib_buf_offset(BUF_TY_BASE, 2) - bib_buf_offset(BUF_TY_BASE, 1)), 4 /*bst_command_ilk */ , false)];
    if (!hash_found) {
        print_a_token();
        puts_log(" is an illegal style-file command");
        bst_err_print_and_look_for_blank_line(ctx->bst_ctx);
        return;
    }

    switch (command_num) {
    case 0:
        bst_entry_command(ctx->bst_ctx);
        break;
    case 1:
        bst_execute_command(ctx);
        break;
    case 2:
        bst_function_command(ctx->bst_ctx);
        break;
    case 3:
        bst_integers_command(ctx->bst_ctx);
        break;
    case 4:
        bst_iterate_command(ctx);
        break;
    case 5:
        bst_macro_command(ctx->bst_ctx);
        break;
    case 6:
        bst_read_command(ctx->bst_ctx);
        break;
    case 7:
        bst_reverse_command(ctx);
        break;
    case 8:
        bst_sort_command(ctx->bst_ctx);
        break;
    case 9:
        bst_strings_command(ctx->bst_ctx);
        break;
    default:
        puts_log("Unknown style-file command");
        print_confusion();
        longjmp(error_jmpbuf, 1);
        break;
    }
}


static void
setup_params(void)
{
    ent_str_size = ENT_STR_SIZE;
    glob_str_size = GLOB_STR_SIZE;

    hash_size = bib_max_strings();
    if (hash_size < 5000 /*HASH_SIZE */ )
        hash_size = 5000 /*HASH_SIZE */ ;
    hash_max = hash_size + hash_base - 1;
    end_of_def = hash_max + 1;
    undefined = hash_max + 1;
}


static void
compute_hash_prime(void)
{
    int32_t hash_want, k, j, o, n, square;
    bool j_prime;

    hash_want = (hash_size / 20) * 17;
    j = 1;
    k = 1;
    hash_prime = 2;
    hash_next[k] = hash_prime;
    o = 2;
    square = 9;

    while (hash_prime < hash_want) {
        do {
            j += 2;

            if (j == square) {
                hash_text[o] = j;
                j += 2;
                o += 1;
                square = hash_next[o] * hash_next[o];
            }

            n = 2;
            j_prime = true;

            while (n < o && j_prime) {
                while (hash_text[n] < j)
                    hash_text[n] += 2 * hash_next[n];
                if (hash_text[n] == j)
                    j_prime = false;
                n = n + 1;
            }
        } while (!j_prime);

        k++;
        hash_prime = j;
        hash_next[k] = hash_prime;
    }
}


static int
initialize(const char *aux_file_name)
{
    hash_loc k;

    bad = 0;
    reset_all();

    if (min_print_line < 3)
        bad = 1;
    if (max_print_line <= min_print_line)
        bad = 10 * bad + 2;
    if (max_print_line >= bib_buf_size())
        bad = 10 * bad + 3;
    if (hash_prime < 128)
        bad = 10 * bad + 4;
    if (hash_prime > hash_size)
        bad = 10 * bad + 5;
    if (hash_base != 1)
        bad = 10 * bad + 6;
    if (bib_max_strings() > hash_size)
        bad = 10 * bad + 7;
    if (max_cites() > bib_max_strings())
        bad = 10 * bad + 8;
    if (10 /*short_list */  < 2 * 4 /*end_offset */  + 2)
        bad = 100 * bad + 22;

    if (bad)
        return 1;

    for (k = hash_base; k <= hash_max; k++) {
        hash_next[k] = 0 /*empty */ ;
        hash_text[k] = 0;
    }

    cite_xptr = 0;
    hash_used = hash_max + 1;
    bib_set_pool_ptr(0);
    bib_set_str_ptr(1);
    bib_set_str_start(bib_str_ptr(), 0);
    bib_seen = false;
    bst_seen = false;
    citation_seen = false;
    all_entries = false;
    wiz_def_ptr = 0;
    num_ent_ints = 0;
    num_ent_strs = 0;
    num_fields = 0;
    str_glb_ptr = 0;

    while (str_glb_ptr < max_glob_strs) {
        glb_bib_str_ptr[str_glb_ptr] = 0;
        glb_str_end[str_glb_ptr] = 0;
        str_glb_ptr = str_glb_ptr + 1;
    }

    num_glb_strs = 0;
    entry_seen = false;
    read_seen = false;
    read_performed = false;
    reading_completed = false;
    impl_fn_num = 0;
    bib_set_buf_len(BUF_TY_OUT, 0);

    pre_def_certain_strings();
    return get_the_top_level_aux_file_name(aux_file_name);
}


History
bibtex_main(const char *aux_file_name)
{
    max_glob_strs = MAX_GLOB_STRS;
    max_fields = MAX_FIELDS;
    wiz_fn_space = WIZ_FN_SPACE;

    if (standard_output() == NULL)
        return HISTORY_FATAL_ERROR;

    setup_params();

    entry_ints = NULL;
    entry_strs = NULL;

    wiz_functions = XTALLOC(wiz_fn_space + 1, hash_ptr2);
    field_info = XTALLOC(max_fields + 1, str_number);
    name_sep_char = XTALLOC(bib_buf_size() + 1, ASCII_code);
    glb_bib_str_ptr = XTALLOC(max_glob_strs, str_number);
    global_strs = XTALLOC(max_glob_strs * (glob_str_size + 1), ASCII_code);
    glb_str_end = XTALLOC(max_glob_strs, int32_t);
    hash_next = XTALLOC(hash_max + 1, hash_pointer);
    hash_text = XTALLOC(hash_max + 1, str_number);
    hash_ilk = XTALLOC(hash_max + 1, str_ilk);
    ilk_info = XTALLOC(hash_max + 1, int32_t);
    fn_type = XTALLOC(hash_max + 1, fn_class);

    compute_hash_prime();

    if (initialize(aux_file_name)) {
        /* TODO: log initialization or get_the_..() error */
        return HISTORY_FATAL_ERROR;
    }

    if (setjmp(error_jmpbuf) == 1)
        goto close_up_shop;

    if (verbose)
        puts_log("This is BibTeX, Version 0.99d\n");
    else
        ttstub_puts (bib_log_file(), "This is BibTeX, Version 0.99d\n");

    {
        char buf[512];
        snprintf (buf, sizeof(buf) - 1, "Capacity: max_strings=%ld, hash_size=%ld, hash_prime=%ld\n",
                  (long) bib_max_strings(), (long) hash_size, (long) hash_prime);
        ttstub_output_write (bib_log_file(), buf, strlen(buf));
    }

    if (verbose) {
        puts_log("The top-level auxiliary file: ");
        TRY(print_aux_name());
    } else {
        ttstub_puts (bib_log_file(), "The top-level auxiliary file: ");
        TRY(log_pr_aux_name());
    }

    BstCtx bst_ctx = { 0 };

    while (true) {
        set_cur_aux_ln(cur_aux_ln() + 1);

        if (!input_ln(cur_aux_file())) {
            if (pop_the_aux_stack())
                break;
        } else {
            get_aux_command_and_process(&bst_ctx);
        }
    }

    last_check_for_aux_errors(&bst_ctx);

    if (bst_ctx.bst_str == 0)
        goto no_bst_file;

    bst_ctx.bst_line_num = 0;
    bbl_line_num = 1;
    bib_set_buf_offset(BUF_TY_BASE, 2, bib_buf_len(BUF_TY_BASE));

    if (setjmp(recover_jmpbuf) == 0) {
        ExecCtx ctx = { 0 };
        ctx.bst_ctx = &bst_ctx;
        ctx.lit_stk_size = LIT_STK_SIZE;
        ctx.lit_stack = XTALLOC(ctx.lit_stk_size + 1, ExecVal);
        
        while(true) {
            if (!eat_bst_white_space(ctx.bst_ctx))
                break;
            get_bst_command_and_process(&ctx);
        }
    }

    peekable_close(bst_ctx.bst_file);
    bst_ctx.bst_file = NULL;

 no_bst_file:
    ttstub_output_close (bbl_file);

close_up_shop:
    /*456:*/

    if (read_performed && !reading_completed) {
        printf_log("Aborted at line %ld of file ", (long) bib_line_num());
        TRY(print_bib_name());
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

    ttstub_output_close(bib_log_file());
    return get_history();
}


History
tt_engine_bibtex_main(ttbc_state_t *api, const BibtexConfig *config, const char *aux_file_name)
{
    History rv;

    if (setjmp(*ttbc_global_engine_enter(api))) {
        ttbc_global_engine_exit();
        return HISTORY_ABORTED;
    }

    bibtex_config = config;
    rv = bibtex_main(aux_file_name);
    ttbc_global_engine_exit();
    return rv;
}