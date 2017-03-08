/*common.defines.  Public domain. This file is concatenated by ./convert to the beginning of the Pascal
 code that tangle outputs.  The idea is to put all these symbols, which
 can be defined as macros or functions, or as part of standard C, into
 web2c's symbol table, so that we can access them from a change file. Some are standard Pascal functions, others are simply used in our
                                                                  implementation. web2c.yacc can parse these @define statements.*//*The fields in the memory_word structure, defined in
                                                                                                                                                                                                                                                                                                                                                             `mfware/gftodmem.h' and `common/texmf.h'. *//*These fields are the ones defined by the getopt library. *//*This is used by \TeX--XeT. *//*@define @field rh; *//*For BibTeX. *//*can't keep |break|, since it's a reserved word *//*for gftodvi, TeX, and Metafont *//*These are all set by getopt.  optiontable is really _getopt_long_options. *//*10: *//*11: *//*$C-,A+,D- */
#ifdef TEXMF_DEBUG
/*$C+,D+*/
#endif                          /* TEXMF_DEBUG */
/*:11*/
#define BIBTEX
#include "cpascal.h"
#include <setjmp.h>
jmp_buf jmp9998, jmp32;
int lab31 = 0;
#define /*14:*/hash_base ( 1 /*empty 1*/)
#define quote_next_fn ( hash_base - 1 )
#define BUF_SIZE ( 20000 )
#define min_print_line ( 3 )
#define max_print_line ( 79 )
#define aux_stack_size ( 20 )
#define MAX_BIB_FILES ( 20 )
#define POOL_SIZE ( 65000L )
#define MAX_STRINGS ( 4000 )
#define MAX_CITES ( 750 )
#define WIZ_FN_SPACE ( 3000 )
#define SINGLE_FN_SPACE ( 50 )
#define ENT_STR_SIZE ( 100 )
#define GLOB_STR_SIZE ( 1000 )
#define MAX_GLOB_STRS ( 10 )
#define MAX_FIELDS ( 5000 )
#define LIT_STK_SIZE ( 50 )
#define num_blt_in_fns ( 37 )
typedef /*22: */ unsigned char ASCII_code;
typedef unsigned char /*last_lex */ lex_type;
typedef unsigned char id_type;
typedef text /* of  ASCII_code */ alpha_file;
typedef integer buf_pointer;
typedef ASCII_code *buf_type;
typedef integer pool_pointer;
typedef integer str_number;
typedef integer hash_loc;
typedef integer hash_pointer;
typedef unsigned char /*last_ilk */ str_ilk;
typedef unsigned char /*longest_pds */ pds_loc;
typedef unsigned char /*longest_pds */ pds_len;
typedef const_cstring pds_type;
typedef integer aux_number;
typedef integer bib_number;
typedef integer cite_number;
typedef unsigned char /*last_fn_class */ fn_class;
typedef integer wiz_fn_loc;
typedef integer int_ent_loc;
typedef integer str_ent_loc;
typedef integer str_glob_loc;
typedef integer field_loc;
typedef integer hash_ptr2;
typedef integer lit_stk_loc;
typedef unsigned char /*last_lit_type */ stk_type;
typedef integer blt_in_range;
text standard_input, standard_output;
integer pool_size;
integer max_bib_files;
integer max_cites;
integer wiz_fn_space;
integer ent_str_size;
integer glob_str_size;
integer max_glob_strs;
integer max_fields;
integer lit_stk_size;
integer max_strings;
integer hash_size;
integer hash_prime;
integer hash_max;
integer end_of_def;
integer undefined;
integer bad;
unsigned char /*fatal_message */ history;
integer err_count;
ASCII_code xord[256];
ASCII_code xchr[256];
lex_type lex_class[256];
id_type id_class[256];
integer char_width[256];
integer string_width;
ASCII_code *name_of_file;
integer name_length;
integer name_ptr;
integer buf_size;
buf_type buffer;
buf_pointer last;
buf_type sv_buffer;
buf_pointer sv_ptr1;
buf_pointer sv_ptr2;
integer tmp_ptr, tmp_end_ptr;
ASCII_code *str_pool;
pool_pointer *str_start;
pool_pointer pool_ptr;
str_number str_ptr;
str_number str_num;
pool_pointer p_ptr1, p_ptr2;
hash_pointer *hash_next;
str_number *hash_text;
str_ilk *hash_ilk;
integer *ilk_info;
integer hash_used;
boolean hash_found;
hash_loc dummy_loc;
str_number s_aux_extension;
str_number s_log_extension;
str_number s_bbl_extension;
str_number s_bst_extension;
str_number s_bib_extension;
str_number s_bst_area;
str_number s_bib_area;
hash_loc pre_def_loc;
integer command_num;
buf_pointer buf_ptr1;
buf_pointer buf_ptr2;
unsigned char /*white_adjacent */ scan_result;
integer token_value;
integer aux_name_length;
alpha_file aux_file[aux_stack_size + 1];
str_number aux_list[aux_stack_size + 1];
aux_number aux_ptr;
integer aux_ln_stack[aux_stack_size + 1];
str_number top_lev_str;
alpha_file log_file;
alpha_file bbl_file;
str_number *bib_list;
bib_number bib_ptr;
bib_number num_bib_files;
boolean bib_seen;
alpha_file *bib_file;
boolean bst_seen;
str_number bst_str;
alpha_file bst_file;
str_number *cite_list;
cite_number cite_ptr;
cite_number entry_cite_ptr;
cite_number num_cites;
cite_number old_num_cites;
boolean citation_seen;
hash_loc cite_loc;
hash_loc lc_cite_loc;
hash_loc lc_xcite_loc;
boolean cite_found;
boolean all_entries;
cite_number all_marker;
integer bbl_line_num;
integer bst_line_num;
hash_loc fn_loc;
hash_loc wiz_loc;
hash_loc literal_loc;
hash_loc macro_name_loc;
hash_loc macro_def_loc;
fn_class *fn_type;
wiz_fn_loc wiz_def_ptr;
wiz_fn_loc wiz_fn_ptr;
hash_ptr2 *wiz_functions;
int_ent_loc int_ent_ptr;
integer *entry_ints;
int_ent_loc num_ent_ints;
str_ent_loc str_ent_ptr;
ASCII_code *entry_strs;
str_ent_loc num_ent_strs;
integer str_glb_ptr;
str_number *glb_str_ptr;
ASCII_code *global_strs;
integer *glb_str_end;
integer num_glb_strs;
field_loc field_ptr;
field_loc field_parent_ptr, field_end_ptr;
cite_number cite_parent_ptr, cite_xptr;
str_number *field_info;
field_loc num_fields;
field_loc num_pre_defined_fields;
field_loc crossref_num;
boolean no_fields;
boolean entry_seen;
boolean read_seen;
boolean read_performed;
boolean reading_completed;
boolean read_completed;
integer impl_fn_num;
integer bib_line_num;
hash_loc entry_type_loc;
hash_ptr2 *type_list;
boolean type_exists;
boolean *entry_exists;
boolean store_entry;
hash_loc field_name_loc;
hash_loc field_val_loc;
boolean store_field;
boolean store_token;
ASCII_code right_outer_delim;
ASCII_code right_str_delim;
boolean at_bib_command;
hash_loc cur_macro_loc;
str_number *cite_info;
boolean cite_hash_found;
bib_number preamble_ptr;
bib_number num_preamble_strings;
integer bib_brace_level;
integer *lit_stack;
stk_type *lit_stk_type;
lit_stk_loc lit_stk_ptr;
str_number cmd_str_ptr;
integer ent_chr_ptr;
integer glob_chr_ptr;
buf_type ex_buf;
buf_pointer ex_buf_ptr;
buf_pointer ex_buf_length;
buf_type out_buf;
buf_pointer out_buf_ptr;
buf_pointer out_buf_length;
boolean mess_with_entries;
cite_number sort_cite_ptr;
str_ent_loc sort_key_num;
integer brace_level;
hash_loc b_equals;
hash_loc b_greater_than;
hash_loc b_less_than;
hash_loc b_plus;
hash_loc b_minus;
hash_loc b_concatenate;
hash_loc b_gets;
hash_loc b_add_period;
hash_loc b_call_type;
hash_loc b_change_case;
hash_loc b_chr_to_int;
hash_loc b_cite;
hash_loc b_duplicate;
hash_loc b_empty;
hash_loc b_format_name;
hash_loc b_if;
hash_loc b_int_to_chr;
hash_loc b_int_to_str;
hash_loc b_missing;
hash_loc b_newline;
hash_loc b_num_names;
hash_loc b_pop;
hash_loc b_preamble;
hash_loc b_purify;
hash_loc b_quote;
hash_loc b_skip;
hash_loc b_stack;
hash_loc b_substring;
hash_loc b_swap;
hash_loc b_text_length;
hash_loc b_text_prefix;
hash_loc b_top_stack;
hash_loc b_type;
hash_loc b_warning;
hash_loc b_while;
hash_loc b_width;
hash_loc b_write;
hash_loc b_default;

#ifndef NO_BIBTEX_STAT
hash_loc blt_in_loc[num_blt_in_fns + 1];
integer execution_count[num_blt_in_fns + 1];
integer total_ex_count;
blt_in_range blt_in_ptr;

#endif                          /* not NO_BIBTEX_STAT */
str_number s_null;
str_number s_default;
str_number s_t;
str_number s_l;
str_number s_u;
str_number *s_preamble;
integer pop_lit1, pop_lit2, pop_lit3;
stk_type pop_typ1, pop_typ2, pop_typ3;
pool_pointer sp_ptr;
pool_pointer sp_xptr1, sp_xptr2;
pool_pointer sp_end;
pool_pointer sp_length, sp2_length;
integer sp_brace_level;
buf_pointer ex_buf_xptr, ex_buf_yptr;
hash_loc control_seq_loc;
boolean preceding_white;
boolean and_found;
integer num_names;
buf_pointer name_bf_ptr;
buf_pointer name_bf_xptr, name_bf_yptr;
integer nm_brace_level;
buf_pointer *name_tok;
ASCII_code *name_sep_char;
buf_pointer num_tokens;
boolean token_starting;
boolean alpha_found;
boolean double_letter, end_of_group, to_be_written;
buf_pointer first_start;
buf_pointer first_end;
buf_pointer last_end;
buf_pointer von_start;
buf_pointer von_end;
buf_pointer jr_end;
buf_pointer cur_token, last_token;
boolean use_default;
buf_pointer num_commas;
buf_pointer comma1, comma2;
buf_pointer num_text_chars;
unsigned char /*bad_conversion */ conversion_type;
boolean prev_colon;
c_int_type verbose;
integer min_crossrefs;
               /*:473*//*12: *//*3: */
#include "bibtex.h"
void print_a_newline(void)
{
    putc('\n', log_file);
    putc('\n', standard_output);
}

void mark_warning(void)
{
    if ((history == 1 /*warning_message */ ))
        err_count = err_count + 1;
    else if ((history == 0 /*spotless */ )) {
        history = 1 /*warning_message */ ;
        err_count = 1;
    }
}

void mark_error(void)
{
    if ((history < 2 /*error_message */ )) {
        history = 2 /*error_message */ ;
        err_count = 1;
    } else
        err_count = err_count + 1;
}

void mark_fatal(void)
{
    history = 3 /*fatal_message */ ;
}

void print_overflow(void)
{
    {
        Fputs(log_file, "Sorry---you've exceeded BibTeX's ");
        Fputs(standard_output, "Sorry---you've exceeded BibTeX's ");
    }
    mark_fatal();
}

void print_confusion(void)
{
    {
        fprintf(log_file, "%s\n", "---this can't happen");
        fprintf(standard_output, "%s\n", "---this can't happen");
    }
    {
        fprintf(log_file, "%s\n", "*Please notify the BibTeX maintainer*");
        fprintf(standard_output, "%s\n", "*Please notify the BibTeX maintainer*");
    }
    mark_fatal();
}

void buffer_overflow(void)
{
    BIB_XRETALLOC_NOSET("buffer", buffer, ASCII_code, buf_size, buf_size + BUF_SIZE);
    BIB_XRETALLOC_NOSET("sv_buffer", sv_buffer, ASCII_code, buf_size, buf_size + BUF_SIZE);
    BIB_XRETALLOC_NOSET("ex_buf", ex_buf, ASCII_code, buf_size, buf_size + BUF_SIZE);
    BIB_XRETALLOC_NOSET("out_buf", out_buf, ASCII_code, buf_size, buf_size + BUF_SIZE);
    BIB_XRETALLOC_NOSET("name_tok", name_tok, buf_pointer, buf_size, buf_size + BUF_SIZE);
    BIB_XRETALLOC("name_sep_char", name_sep_char, ASCII_code, buf_size, buf_size + BUF_SIZE);
}

boolean zinput_ln(alpha_file f)
{
    register boolean Result;
    last = 0;
    if ((eof(f)))
        Result = false;
    else {

        while ((!eoln(f))) {

            if ((last >= buf_size))
                buffer_overflow();
            buffer[last] = xord[getc(f)];
            last = last + 1;
        }
        vgetc(f);
        while ((last > 0))
            if ((lex_class[buffer[last - 1]] == 1 /*white_space */ ))
                last = last - 1;
            else
                goto lab15;
 lab15:                        /*loop_exit */ Result = true;
    }
    return Result;
}

void zout_pool_str(alpha_file f, str_number s)
{
    pool_pointer i;
    if (((s < 0) || (s >= str_ptr + 3) || (s >= max_strings))) {
        {
            fprintf(log_file, "%s%ld", "Illegal string number:", (long)s);
            fprintf(standard_output, "%s%ld", "Illegal string number:", (long)s);
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
    {
        register integer for_end;
        i = str_start[s];
        for_end = str_start[s + 1] - 1;
        if (i <= for_end)
            do
                putc(xchr[str_pool[i]], f);
            while (i++ < for_end);
    }
}

void zprint_a_pool_str(str_number s)
{
    out_pool_str(standard_output, s);
    out_pool_str(log_file, s);
}

void pool_overflow(void)
{
    BIB_XRETALLOC("str_pool", str_pool, ASCII_code, pool_size, pool_size + POOL_SIZE);
}

void zout_token(alpha_file f)
{
    buf_pointer i;
    i = buf_ptr1;
    while ((i < buf_ptr2)) {

        putc(xchr[buffer[i]], f);
        i = i + 1;
    }
}

void print_a_token(void)
{
    out_token(standard_output);
    out_token(log_file);
}

void print_bad_input_line(void)
{
    buf_pointer bf_ptr;
    {
        Fputs(log_file, " : ");
        Fputs(standard_output, " : ");
    }
    bf_ptr = 0;
    while ((bf_ptr < buf_ptr2)) {

        if ((lex_class[buffer[bf_ptr]] == 1 /*white_space */ )) {
            putc(xchr[32 /*space */ ], log_file);
            putc(xchr[32 /*space */ ], standard_output);
        } else {

            putc(xchr[buffer[bf_ptr]], log_file);
            putc(xchr[buffer[bf_ptr]], standard_output);
        }
        bf_ptr = bf_ptr + 1;
    }
    print_a_newline();
    {
        Fputs(log_file, " : ");
        Fputs(standard_output, " : ");
    }
    bf_ptr = 0;
    while ((bf_ptr < buf_ptr2)) {

        {
            putc(xchr[32 /*space */ ], log_file);
            putc(xchr[32 /*space */ ], standard_output);
        }
        bf_ptr = bf_ptr + 1;
    }
    bf_ptr = buf_ptr2;
    while ((bf_ptr < last)) {

        if ((lex_class[buffer[bf_ptr]] == 1 /*white_space */ )) {
            putc(xchr[32 /*space */ ], log_file);
            putc(xchr[32 /*space */ ], standard_output);
        } else {

            putc(xchr[buffer[bf_ptr]], log_file);
            putc(xchr[buffer[bf_ptr]], standard_output);
        }
        bf_ptr = bf_ptr + 1;
    }
    print_a_newline();
    bf_ptr = 0;
    while (((bf_ptr < buf_ptr2) && (lex_class[buffer[bf_ptr]] == 1 /*white_space */ )))
        bf_ptr = bf_ptr + 1;
    if ((bf_ptr == buf_ptr2)) {
        fprintf(log_file, "%s\n", "(Error may have been on previous line)");
        fprintf(standard_output, "%s\n", "(Error may have been on previous line)");
    }
    mark_error();
}

void print_skipping_whatever_remains(void)
{
    {
        Fputs(log_file, "I'm skipping whatever remains of this ");
        Fputs(standard_output, "I'm skipping whatever remains of this ");
    }
}

void sam_too_long_file_name_print(void)
{
    Fputs(standard_output, "File name `");
    name_ptr = 1;
    while ((name_ptr <= aux_name_length)) {

        putc(name_of_file[name_ptr], standard_output);
        name_ptr = name_ptr + 1;
    }
    fprintf(standard_output, "%s\n", "' is too long");
}

void sam_wrong_file_name_print(void)
{
    Fputs(standard_output, "I couldn't open file name `");
    name_ptr = 1;
    while ((name_ptr <= name_length)) {

        putc(name_of_file[name_ptr], standard_output);
        name_ptr = name_ptr + 1;
    }
    {
        putc('\'', standard_output);
        putc('\n', standard_output);
    }
}

void print_aux_name(void)
{
    print_a_pool_str(aux_list[aux_ptr]);
    print_a_newline();
}

void log_pr_aux_name(void)
{
    {
        out_pool_str(log_file, aux_list[aux_ptr]);
    }
    {
        putc('\n', log_file);
    }
}

void aux_err_print(void)
{
    {
        fprintf(log_file, "%s%ld%s", "---line ", (long)aux_ln_stack[aux_ptr], " of file ");
        fprintf(standard_output, "%s%ld%s", "---line ", (long)aux_ln_stack[aux_ptr], " of file ");
    }
    print_aux_name();
    print_bad_input_line();
    print_skipping_whatever_remains();
    {
        fprintf(log_file, "%s\n", "command");
        fprintf(standard_output, "%s\n", "command");
    }
}

void zaux_err_illegal_another_print(integer cmd_num)
{
    {
        Fputs(log_file, "Illegal, another \\bib");
        Fputs(standard_output, "Illegal, another \\bib");
    }
    switch ((cmd_num)) {
    case 0:
        {
            Fputs(log_file, "data");
            Fputs(standard_output, "data");
        }
        break;
    case 1:
        {
            Fputs(log_file, "style");
            Fputs(standard_output, "style");
        }
        break;
    default:
        {
            {
                Fputs(log_file, "Illegal auxiliary-file command");
                Fputs(standard_output, "Illegal auxiliary-file command");
            }
            print_confusion();
            longjmp(jmp9998, 1);
        }
        break;
    }
    {
        Fputs(log_file, " command");
        Fputs(standard_output, " command");
    }
}

void aux_err_no_right_brace_print(void)
{
    {
        fprintf(log_file, "%s%c%c", "No \"", xchr[125 /*right_brace */ ], '"');
        fprintf(standard_output, "%s%c%c", "No \"", xchr[125 /*right_brace */ ], '"');
    }
}

void aux_err_stuff_after_right_brace_print(void)
{
    {
        fprintf(log_file, "%s%c%c", "Stuff after \"", xchr[125 /*right_brace */ ], '"');
        fprintf(standard_output, "%s%c%c", "Stuff after \"", xchr[125 /*right_brace */ ], '"');
    }
}

void aux_err_white_space_in_argument_print(void)
{
    {
        Fputs(log_file, "White space in argument");
        Fputs(standard_output, "White space in argument");
    }
}

void print_bib_name(void)
{
    print_a_pool_str(bib_list[bib_ptr]);
    print_a_pool_str(s_bib_extension);
    print_a_newline();
}

void log_pr_bib_name(void)
{
    {
        out_pool_str(log_file, bib_list[bib_ptr]);
    }
    {
        out_pool_str(log_file, s_bib_extension);
    }
    {
        putc('\n', log_file);
    }
}

void print_bst_name(void)
{
    print_a_pool_str(bst_str);
    print_a_pool_str(s_bst_extension);
    print_a_newline();
}

void log_pr_bst_name(void)
{
    {
        out_pool_str(log_file, bst_str);
    }
    {
        out_pool_str(log_file, s_bst_extension);
    }
    {
        putc('\n', log_file);
    }
}

void hash_cite_confusion(void)
{
    {
        {
            Fputs(log_file, "Cite hash error");
            Fputs(standard_output, "Cite hash error");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void zcheck_cite_overflow(cite_number last_cite)
{
    if ((last_cite == max_cites)) {
        BIB_XRETALLOC_NOSET("cite_list", cite_list, str_number, max_cites, max_cites + MAX_CITES);
        BIB_XRETALLOC_NOSET("type_list", type_list, hash_ptr2, max_cites, max_cites + MAX_CITES);
        BIB_XRETALLOC_NOSET("entry_exists", entry_exists, boolean, max_cites, max_cites + MAX_CITES);
        BIB_XRETALLOC("cite_info", cite_info, str_number, max_cites, max_cites + MAX_CITES);
        while ((last_cite < max_cites)) {

            type_list[last_cite] = 0 /*empty */ ;
            cite_info[last_cite] = 0 /*any_value */ ;
            last_cite = last_cite + 1;
        }
    }
}

void aux_end1_err_print(void)
{
    {
        Fputs(log_file, "I found no ");
        Fputs(standard_output, "I found no ");
    }
}

void aux_end2_err_print(void)
{
    {
        Fputs(log_file, "---while reading file ");
        Fputs(standard_output, "---while reading file ");
    }
    print_aux_name();
    mark_error();
}

void bst_ln_num_print(void)
{
    {
        fprintf(log_file, "%s%ld%s", "--line ", (long)bst_line_num, " of file ");
        fprintf(standard_output, "%s%ld%s", "--line ", (long)bst_line_num, " of file ");
    }
    print_bst_name();
}

void bst_err_print_and_look_for_blank_line(void)
{
    {
        putc('-', log_file);
        putc('-', standard_output);
    }
    bst_ln_num_print();
    print_bad_input_line();
    while ((last != 0))
        if ((!input_ln(bst_file)))
            longjmp(jmp32, 1);
        else
            bst_line_num = bst_line_num + 1;
    buf_ptr2 = last;
}

void bst_warn_print(void)
{
    bst_ln_num_print();
    mark_warning();
}

void eat_bst_print(void)
{
    {
        Fputs(log_file, "Illegal end of style file in command: ");
        Fputs(standard_output, "Illegal end of style file in command: ");
    }
}

void unknwn_function_class_confusion(void)
{
    {
        {
            Fputs(log_file, "Unknown function class");
            Fputs(standard_output, "Unknown function class");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void zprint_fn_class(hash_loc fn_loc)
{
    switch ((fn_type[fn_loc])) {
    case 0:
        {
            Fputs(log_file, "built-in");
            Fputs(standard_output, "built-in");
        }
        break;
    case 1:
        {
            Fputs(log_file, "wizard-defined");
            Fputs(standard_output, "wizard-defined");
        }
        break;
    case 2:
        {
            Fputs(log_file, "integer-literal");
            Fputs(standard_output, "integer-literal");
        }
        break;
    case 3:
        {
            Fputs(log_file, "string-literal");
            Fputs(standard_output, "string-literal");
        }
        break;
    case 4:
        {
            Fputs(log_file, "field");
            Fputs(standard_output, "field");
        }
        break;
    case 5:
        {
            Fputs(log_file, "integer-entry-variable");
            Fputs(standard_output, "integer-entry-variable");
        }
        break;
    case 6:
        {
            Fputs(log_file, "string-entry-variable");
            Fputs(standard_output, "string-entry-variable");
        }
        break;
    case 7:
        {
            Fputs(log_file, "integer-global-variable");
            Fputs(standard_output, "integer-global-variable");
        }
        break;
    case 8:
        {
            Fputs(log_file, "string-global-variable");
            Fputs(standard_output, "string-global-variable");
        }
        break;
    default:
        unknwn_function_class_confusion();
        break;
    }
}

        /*:159*//*160: */
#ifdef TRACE
void ztrace_pr_fn_class(hash_loc fn_loc)
{
    switch ((fn_type[fn_loc])) {
    case 0:
        {
            Fputs(log_file, "built-in");
        }
        break;
    case 1:
        {
            Fputs(log_file, "wizard-defined");
        }
        break;
    case 2:
        {
            Fputs(log_file, "integer-literal");
        }
        break;
    case 3:
        {
            Fputs(log_file, "string-literal");
        }
        break;
    case 4:
        {
            Fputs(log_file, "field");
        }
        break;
    case 5:
        {
            Fputs(log_file, "integer-entry-variable");
        }
        break;
    case 6:
        {
            Fputs(log_file, "string-entry-variable");
        }
        break;
    case 7:
        {
            Fputs(log_file, "integer-global-variable");
        }
        break;
    case 8:
        {
            Fputs(log_file, "string-global-variable");
        }
        break;
    default:
        unknwn_function_class_confusion();
        break;
    }
}

#endif                          /* TRACE */
void id_scanning_confusion(void)
{
    {
        {
            Fputs(log_file, "Identifier scanning error");
            Fputs(standard_output, "Identifier scanning error");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void bst_id_print(void)
{
    if ((scan_result == 0 /*id_null */ )) {
        fprintf(log_file, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" begins identifier, command: ");
        fprintf(standard_output, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" begins identifier, command: ");
    } else if ((scan_result == 2 /*other_char_adjacent */ )) {
        fprintf(log_file, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" immediately follows identifier, command: ");
        fprintf(standard_output, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" immediately follows identifier, command: ");
    } else
        id_scanning_confusion();
}

void bst_left_brace_print(void)
{
    {
        fprintf(log_file, "%c%c%s", '"', xchr[123 /*left_brace */ ], "\" is missing in command: ");
        fprintf(standard_output, "%c%c%s", '"', xchr[123 /*left_brace */ ], "\" is missing in command: ");
    }
}

void bst_right_brace_print(void)
{
    {
        fprintf(log_file, "%c%c%s", '"', xchr[125 /*right_brace */ ], "\" is missing in command: ");
        fprintf(standard_output, "%c%c%s", '"', xchr[125 /*right_brace */ ], "\" is missing in command: ");
    }
}

void zalready_seen_function_print(hash_loc seen_fn_loc)
{
    print_a_pool_str(hash_text[seen_fn_loc]);
    {
        Fputs(log_file, " is already a type \"");
        Fputs(standard_output, " is already a type \"");
    }
    print_fn_class(seen_fn_loc);
    {
        fprintf(log_file, "%s\n", "\" function name");
        fprintf(standard_output, "%s\n", "\" function name");
    }
    {
        bst_err_print_and_look_for_blank_line();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void bib_ln_num_print(void)
{
    {
        fprintf(log_file, "%s%ld%s", "--line ", (long)bib_line_num, " of file ");
        fprintf(standard_output, "%s%ld%s", "--line ", (long)bib_line_num, " of file ");
    }
    print_bib_name();
}

void bib_err_print(void)
{
    {
        putc('-', log_file);
        putc('-', standard_output);
    }
    bib_ln_num_print();
    print_bad_input_line();
    print_skipping_whatever_remains();
    if ((at_bib_command)) {
        fprintf(log_file, "%s\n", "command");
        fprintf(standard_output, "%s\n", "command");
    } else {

        fprintf(log_file, "%s\n", "entry");
        fprintf(standard_output, "%s\n", "entry");
    }
}

void bib_warn_print(void)
{
    bib_ln_num_print();
    mark_warning();
}

void zcheck_field_overflow(integer total_fields)
{
    field_loc f_ptr;
    field_loc start_fields;
    if ((total_fields > max_fields)) {
        start_fields = max_fields;
        BIB_XRETALLOC("field_info", field_info, str_number, max_fields, total_fields + MAX_FIELDS);
        {
            register integer for_end;
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

void eat_bib_print(void)
{
    {
        {
            Fputs(log_file, "Illegal end of database file");
            Fputs(standard_output, "Illegal end of database file");
        }
        bib_err_print();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void zbib_one_of_two_print(ASCII_code char1, ASCII_code char2)
{
    {
        {
            fprintf(log_file, "%s%c%s%c%c", "I was expecting a `", xchr[char1], "' or a `", xchr[char2], '\'');
            fprintf(standard_output, "%s%c%s%c%c", "I was expecting a `", xchr[char1], "' or a `", xchr[char2], '\'');
        }
        bib_err_print();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void bib_equals_sign_print(void)
{
    {
        {
            fprintf(log_file, "%s%c%c", "I was expecting an \"", xchr[61 /*equals_sign */ ], '"');
            fprintf(standard_output, "%s%c%c", "I was expecting an \"", xchr[61 /*equals_sign */ ], '"');
        }
        bib_err_print();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void bib_unbalanced_braces_print(void)
{
    {
        {
            Fputs(log_file, "Unbalanced braces");
            Fputs(standard_output, "Unbalanced braces");
        }
        bib_err_print();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void bib_field_too_long_print(void)
{
    {
        {
            fprintf(log_file, "%s%ld%s", "Your field is more than ", (long)buf_size, " characters");
            fprintf(standard_output, "%s%ld%s", "Your field is more than ", (long)buf_size, " characters");
        }
        bib_err_print();
        goto lab10;
    }
 lab10:                        /*exit */ ;
}

void macro_warn_print(void)
{
    {
        Fputs(log_file, "Warning--string name \"");
        Fputs(standard_output, "Warning--string name \"");
    }
    print_a_token();
    {
        Fputs(log_file, "\" is ");
        Fputs(standard_output, "\" is ");
    }
}

void bib_id_print(void)
{
    if ((scan_result == 0 /*id_null */ )) {
        Fputs(log_file, "You're missing ");
        Fputs(standard_output, "You're missing ");
    } else if ((scan_result == 2 /*other_char_adjacent */ )) {
        fprintf(log_file, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" immediately follows ");
        fprintf(standard_output, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" immediately follows ");
    } else
        id_scanning_confusion();
}

void bib_cmd_confusion(void)
{
    {
        {
            Fputs(log_file, "Unknown database-file command");
            Fputs(standard_output, "Unknown database-file command");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void cite_key_disappeared_confusion(void)
{
    {
        {
            Fputs(log_file, "A cite key disappeared");
            Fputs(standard_output, "A cite key disappeared");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void zbad_cross_reference_print(str_number s)
{
    {
        Fputs(log_file, "--entry \"");
        Fputs(standard_output, "--entry \"");
    }
    print_a_pool_str(cite_list[cite_ptr]);
    {
        {
            putc('"', log_file);
            putc('\n', log_file);
        }
        {
            putc('"', standard_output);
            putc('\n', standard_output);
        }
    }
    {
        Fputs(log_file, "refers to entry \"");
        Fputs(standard_output, "refers to entry \"");
    }
    print_a_pool_str(s);
}

void nonexistent_cross_reference_error(void)
{
    {
        Fputs(log_file, "A bad cross reference-");
        Fputs(standard_output, "A bad cross reference-");
    }
    bad_cross_reference_print(field_info[field_ptr]);
    {
        fprintf(log_file, "%s\n", "\", which doesn't exist");
        fprintf(standard_output, "%s\n", "\", which doesn't exist");
    }
    mark_error();
}

void zprint_missing_entry(str_number s)
{
    {
        Fputs(log_file, "Warning--I didn't find a database entry for \"");
        Fputs(standard_output, "Warning--I didn't find a database entry for \"");
    }
    print_a_pool_str(s);
    {
        {
            putc('"', log_file);
            putc('\n', log_file);
        }
        {
            putc('"', standard_output);
            putc('\n', standard_output);
        }
    }
    mark_warning();
}

void bst_ex_warn_print(void)
{
    if ((mess_with_entries)) {
        {
            Fputs(log_file, " for entry ");
            Fputs(standard_output, " for entry ");
        }
        print_a_pool_str(cite_list[cite_ptr]);
    }
    print_a_newline();
    {
        Fputs(log_file, "while executing-");
        Fputs(standard_output, "while executing-");
    }
    bst_ln_num_print();
    mark_error();
}

void bst_mild_ex_warn_print(void)
{
    if ((mess_with_entries)) {
        {
            Fputs(log_file, " for entry ");
            Fputs(standard_output, " for entry ");
        }
        print_a_pool_str(cite_list[cite_ptr]);
    }
    print_a_newline();
    {
        {
            Fputs(log_file, "while executing");
            Fputs(standard_output, "while executing");
        }
        bst_warn_print();
    }
}

void bst_cant_mess_with_entries_print(void)
{
    {
        {
            Fputs(log_file, "You can't mess with entries here");
            Fputs(standard_output, "You can't mess with entries here");
        }
        bst_ex_warn_print();
    }
}

void illegl_literal_confusion(void)
{
    {
        {
            Fputs(log_file, "Illegal literal type");
            Fputs(standard_output, "Illegal literal type");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void unknwn_literal_confusion(void)
{
    {
        {
            Fputs(log_file, "Unknown literal type");
            Fputs(standard_output, "Unknown literal type");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void zprint_stk_lit(integer stk_lt, stk_type stk_tp)
{
    switch ((stk_tp)) {
    case 0:
        {
            fprintf(log_file, "%ld%s", (long)stk_lt, " is an integer literal");
            fprintf(standard_output, "%ld%s", (long)stk_lt, " is an integer literal");
        }
        break;
    case 1:
        {
            {
                putc('"', log_file);
                putc('"', standard_output);
            }
            print_a_pool_str(stk_lt);
            {
                Fputs(log_file, "\" is a string literal");
                Fputs(standard_output, "\" is a string literal");
            }
        }
        break;
    case 2:
        {
            {
                putc('`', log_file);
                putc('`', standard_output);
            }
            print_a_pool_str(hash_text[stk_lt]);
            {
                Fputs(log_file, "' is a function literal");
                Fputs(standard_output, "' is a function literal");
            }
        }
        break;
    case 3:
        {
            {
                putc('`', log_file);
                putc('`', standard_output);
            }
            print_a_pool_str(stk_lt);
            {
                Fputs(log_file, "' is a missing field");
                Fputs(standard_output, "' is a missing field");
            }
        }
        break;
    case 4:
        illegl_literal_confusion();
        break;
    default:
        unknwn_literal_confusion();
        break;
    }
}

void zprint_lit(integer stk_lt, stk_type stk_tp)
{
    switch ((stk_tp)) {
    case 0:
        {
            fprintf(log_file, "%ld\n", (long)stk_lt);
            fprintf(standard_output, "%ld\n", (long)stk_lt);
        }
        break;
    case 1:
        {
            print_a_pool_str(stk_lt);
            print_a_newline();
        }
        break;
    case 2:
        {
            print_a_pool_str(hash_text[stk_lt]);
            print_a_newline();
        }
        break;
    case 3:
        {
            print_a_pool_str(stk_lt);
            print_a_newline();
        }
        break;
    case 4:
        illegl_literal_confusion();
        break;
    default:
        unknwn_literal_confusion();
        break;
    }
}

void output_bbl_line(void)
{
    if ((out_buf_length != 0)) {
        while ((out_buf_length > 0))
            if ((lex_class[out_buf[out_buf_length - 1]] == 1 /*white_space */ ))
                out_buf_length = out_buf_length - 1;
            else
                goto lab15;
 lab15:                        /*loop_exit */ if ((out_buf_length == 0))
            goto lab10;
        out_buf_ptr = 0;
        while ((out_buf_ptr < out_buf_length)) {

            putc(xchr[out_buf[out_buf_ptr]], bbl_file);
            out_buf_ptr = out_buf_ptr + 1;
        }
    }
    putc('\n', bbl_file);
    bbl_line_num = bbl_line_num + 1;
    out_buf_length = 0;
 lab10:                        /*exit */ ;
}

void bst_1print_string_size_exceeded(void)
{
    {
        Fputs(log_file, "Warning--you've exceeded ");
        Fputs(standard_output, "Warning--you've exceeded ");
    }
}

void bst_2print_string_size_exceeded(void)
{
    {
        Fputs(log_file, "-string-size,");
        Fputs(standard_output, "-string-size,");
    }
    bst_mild_ex_warn_print();
    {
        fprintf(log_file, "%s\n", "*Please notify the bibstyle designer*");
        fprintf(standard_output, "%s\n", "*Please notify the bibstyle designer*");
    }
}

void zbraces_unbalanced_complaint(str_number pop_lit_var)
{
    {
        Fputs(log_file, "Warning--\"");
        Fputs(standard_output, "Warning--\"");
    }
    print_a_pool_str(pop_lit_var);
    {
        {
            Fputs(log_file, "\" isn't a brace-balanced string");
            Fputs(standard_output, "\" isn't a brace-balanced string");
        }
        bst_mild_ex_warn_print();
    }
}

void case_conversion_confusion(void)
{
    {
        {
            Fputs(log_file, "Unknown type of case conversion");
            Fputs(standard_output, "Unknown type of case conversion");
        }
        print_confusion();
        longjmp(jmp9998, 1);
    }
}

void trace_and_stat_printing(void)
{

    ;

#ifdef TRACE
    {
        if ((num_bib_files == 1)) {
            fprintf(log_file, "%s\n", "The 1 database file is");
        } else {

            fprintf(log_file, "%s%ld%s\n", "The ", (long)num_bib_files, " database files are");
        }
        if ((num_bib_files == 0)) {
            fprintf(log_file, "%s\n", "   undefined");
        } else {

            bib_ptr = 0;
            while ((bib_ptr < num_bib_files)) {

                {
                    Fputs(log_file, "   ");
                }
                {
                    out_pool_str(log_file, bib_list[bib_ptr]);
                }
                {
                    out_pool_str(log_file, s_bib_extension);
                }
                {
                    putc('\n', log_file);
                }
                bib_ptr = bib_ptr + 1;
            }
        }
        {
            Fputs(log_file, "The style file is ");
        }
        if ((bst_str == 0)) {
            fprintf(log_file, "%s\n", "undefined");
        } else {

            {
                out_pool_str(log_file, bst_str);
            }
            {
                out_pool_str(log_file, s_bst_extension);
            }
            {
                putc('\n', log_file);
            }
        }
    }
    {
        if ((all_entries)) {
            fprintf(log_file, "%s%ld%s", "all_marker=", (long)all_marker, ", ");
        }
        if ((read_performed)) {
            fprintf(log_file, "%s%ld\n", "old_num_cites=", (long)old_num_cites);
        } else {

            putc('\n', log_file);
        }
        {
            fprintf(log_file, "%s%ld", "The ", (long)num_cites);
        }
        if ((num_cites == 1)) {
            fprintf(log_file, "%s\n", " entry:");
        } else {

            fprintf(log_file, "%s\n", " entries:");
        }
        if ((num_cites == 0)) {
            fprintf(log_file, "%s\n", "   undefined");
        } else {

            sort_cite_ptr = 0;
            while ((sort_cite_ptr < num_cites)) {

                if ((!read_completed))
                    cite_ptr = sort_cite_ptr;
                else
                    cite_ptr = cite_info[sort_cite_ptr];
                {
                    out_pool_str(log_file, cite_list[cite_ptr]);
                }
                if ((read_performed)) { /*460: */
                    {
                        Fputs(log_file, ", entry-type ");
                    }
                    if ((type_list[cite_ptr] == undefined)) {
                        Fputs(log_file, "unknown");
                    } else if ((type_list[cite_ptr] == 0 /*empty */ )) {
                        Fputs(log_file, "--- no type found");
                    } else {

                        out_pool_str(log_file, hash_text[type_list[cite_ptr]]);
                    }
                    {
                        fprintf(log_file, "%s\n", ", has entry strings");
                    }
                    {
                        if ((num_ent_strs == 0)) {
                            fprintf(log_file, "%s\n", "    undefined");
                        } else if ((!read_completed)) {
                            fprintf(log_file, "%s\n", "    uninitialized");
                        } else {

                            str_ent_ptr = cite_ptr * num_ent_strs;
                            while ((str_ent_ptr < (cite_ptr + 1) * num_ent_strs)) {

                                ent_chr_ptr = 0;
                                {
                                    Fputs(log_file, "    \"");
                                }
                                while ((entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)] !=
                                        127 /*end_of_string */ )) {

                                    {
                                        putc(xchr[entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)]],
                                             log_file);
                                    }
                                    ent_chr_ptr = ent_chr_ptr + 1;
                                }
                                {
                                    {
                                        putc('"', log_file);
                                        putc('\n', log_file);
                                    }
                                }
                                str_ent_ptr = str_ent_ptr + 1;
                            }
                        }
                    }
                    {
                        Fputs(log_file, "  has entry integers");
                    }
                    {
                        if ((num_ent_ints == 0)) {
                            Fputs(log_file, " undefined");
                        } else if ((!read_completed)) {
                            Fputs(log_file, " uninitialized");
                        } else {

                            int_ent_ptr = cite_ptr * num_ent_ints;
                            while ((int_ent_ptr < (cite_ptr + 1) * num_ent_ints)) {

                                {
                                    fprintf(log_file, "%c%ld", ' ', (long)entry_ints[int_ent_ptr]);
                                }
                                int_ent_ptr = int_ent_ptr + 1;
                            }
                        }
                        {
                            putc('\n', log_file);
                        }
                    }
                    {
                        fprintf(log_file, "%s\n", "  and has fields");
                    }
                    {
                        if ((!read_performed)) {
                            fprintf(log_file, "%s\n", "    uninitialized");
                        } else {

                            field_ptr = cite_ptr * num_fields;
                            field_end_ptr = field_ptr + num_fields;
                            if ((field_end_ptr > max_fields)) {
                                {
                                    Fputs(log_file, "field_info index is out of range");
                                    Fputs(standard_output, "field_info index is out of range");
                                }
                                print_confusion();
                                longjmp(jmp9998, 1);
                            }
                            no_fields = true;
                            while ((field_ptr < field_end_ptr)) {

                                if ((field_info[field_ptr] != 0 /*missing */ )) {
                                    {
                                        Fputs(log_file, "    \"");
                                    }
                                    {
                                        out_pool_str(log_file, field_info[field_ptr]);
                                    }
                                    {
                                        {
                                            putc('"', log_file);
                                            putc('\n', log_file);
                                        }
                                    }
                                    no_fields = false;
                                }
                                field_ptr = field_ptr + 1;
                            }
                            if ((no_fields)) {
                                fprintf(log_file, "%s\n", "    missing");
                            }
                        }
                    }
                } else {

                    putc('\n', log_file);
                }
                sort_cite_ptr = sort_cite_ptr + 1;
            }
        }
    }
    {
        {
            fprintf(log_file, "%s\n", "The wiz-defined functions are");
        }
        if ((wiz_def_ptr == 0)) {
            fprintf(log_file, "%s\n", "   nonexistent");
        } else {

            wiz_fn_ptr = 0;
            while ((wiz_fn_ptr < wiz_def_ptr)) {

                if ((wiz_functions[wiz_fn_ptr] == end_of_def)) {
                    fprintf(log_file, "%ld%s\n", (long)wiz_fn_ptr, "--end-of-def--");
                } else if ((wiz_functions[wiz_fn_ptr] == quote_next_fn)) {
                    fprintf(log_file, "%ld%s", (long)wiz_fn_ptr, "  quote_next_function    ");
                } else {

                    {
                        fprintf(log_file, "%ld%s", (long)wiz_fn_ptr, "  `");
                    }
                    {
                        out_pool_str(log_file, hash_text[wiz_functions[wiz_fn_ptr]]);
                    }
                    {
                        {
                            putc('\'', log_file);
                            putc('\n', log_file);
                        }
                    }
                }
                wiz_fn_ptr = wiz_fn_ptr + 1;
            }
        }
    }
    {
        {
            fprintf(log_file, "%s\n", "The string pool is");
        }
        str_num = 1;
        while ((str_num < str_ptr)) {

            {
                fprintf(log_file, "%ld%ld%s", (long)str_num, (long)str_start[str_num], " \"");
            }
            {
                out_pool_str(log_file, str_num);
            }
            {
                {
                    putc('"', log_file);
                    putc('\n', log_file);
                }
            }
            str_num = str_num + 1;
        }
    }
/*:465*/
#endif                          /* TRACE */
    ;

#ifndef NO_BIBTEX_STAT
    {
        {
            fprintf(log_file, "%s%ld", "You've used ", (long)num_cites);
        }
        if ((num_cites == 1)) {
            fprintf(log_file, "%s\n", " entry,");
        } else {

            fprintf(log_file, "%s\n", " entries,");
        }
        {
            fprintf(log_file, "%s%ld%s\n", "            ", (long)wiz_def_ptr, " wiz_defined-function locations,");
        }
        {
            fprintf(log_file, "%s%ld%s%ld%s\n", "            ", (long)str_ptr, " strings with ",
                    (long)str_start[str_ptr], " characters,");
        }
        blt_in_ptr = 0;
        total_ex_count = 0;
        while ((blt_in_ptr < num_blt_in_fns)) {

            total_ex_count = total_ex_count + execution_count[blt_in_ptr];
            blt_in_ptr = blt_in_ptr + 1;
        }
        {
            fprintf(log_file, "%s%ld%s\n", "and the built_in function-call counts, ", (long)total_ex_count,
                    " in all, are:");
        }
        blt_in_ptr = 0;
        while ((blt_in_ptr < num_blt_in_fns)) {

            {
                out_pool_str(log_file, hash_text[blt_in_loc[blt_in_ptr]]);
            }
            {
                fprintf(log_file, "%s%ld\n", " -- ", (long)execution_count[blt_in_ptr]);
            }
            blt_in_ptr = blt_in_ptr + 1;
        }
    }
/*:466*/
#endif                          /* not NO_BIBTEX_STAT */
}

void zstart_name(str_number file_name)
{
    pool_pointer p_ptr;
    free(name_of_file);
    name_of_file = xmalloc_array(ASCII_code, (str_start[file_name + 1] - str_start[file_name]) + 1);
    name_ptr = 1;
    p_ptr = str_start[file_name];
    while ((p_ptr < str_start[file_name + 1])) {

        name_of_file[name_ptr] = chr(str_pool[p_ptr]);
        name_ptr = name_ptr + 1;
        p_ptr = p_ptr + 1;
    }
    name_length = (str_start[file_name + 1] - str_start[file_name]);
    name_of_file[name_length + 1] = 0;
}

void zadd_extension(str_number ext)
{
    pool_pointer p_ptr;
    name_ptr = name_length + 1;
    p_ptr = str_start[ext];
    while ((p_ptr < str_start[ext + 1])) {

        name_of_file[name_ptr] = chr(str_pool[p_ptr]);
        name_ptr = name_ptr + 1;
        p_ptr = p_ptr + 1;
    }
    name_length = name_length + (str_start[ext + 1] - str_start[ext]);
    name_of_file[name_length + 1] = 0;
}

str_number make_string(void)
{
    register str_number Result;
    if ((str_ptr == max_strings)) {
        print_overflow();
        {
            fprintf(log_file, "%s%ld\n", "number of strings ", (long)max_strings);
            fprintf(standard_output, "%s%ld\n", "number of strings ", (long)max_strings);
        }
        longjmp(jmp9998, 1);
    }
    str_ptr = str_ptr + 1;
    str_start[str_ptr] = pool_ptr;
    Result = str_ptr - 1;
    return Result;
}

boolean zstr_eq_buf(str_number s, buf_type buf, buf_pointer bf_ptr, buf_pointer len)
{
    register boolean Result;
    buf_pointer i;
    pool_pointer j;
    if (((str_start[s + 1] - str_start[s]) != len)) {
        Result = false;
        goto lab10;
    }
    i = bf_ptr;
    j = str_start[s];
    while ((j < str_start[s + 1])) {

        if ((str_pool[j] != buf[i])) {
            Result = false;
            goto lab10;
        }
        i = i + 1;
        j = j + 1;
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

boolean zstr_eq_str(str_number s1, str_number s2)
{
    register boolean Result;
    if (((str_start[s1 + 1] - str_start[s1]) != (str_start[s2 + 1] - str_start[s2]))) {
        Result = false;
        goto lab10;
    }
    p_ptr1 = str_start[s1];
    p_ptr2 = str_start[s2];
    while ((p_ptr1 < str_start[s1 + 1])) {

        if ((str_pool[p_ptr1] != str_pool[p_ptr2])) {
            Result = false;
            goto lab10;
        }
        p_ptr1 = p_ptr1 + 1;
        p_ptr2 = p_ptr2 + 1;
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

void zlower_case(buf_type buf, buf_pointer bf_ptr, buf_pointer len)
{
    buf_pointer i;
    if ((len > 0)) {
        register integer for_end;
        i = bf_ptr;
        for_end = bf_ptr + len - 1;
        if (i <= for_end)
            do
                if (((buf[i] >= 65 /*"A" */ ) && (buf[i] <= 90 /*"Z" */ )))
                    buf[i] = buf[i] + 32;
            while (i++ < for_end) ;
    }
}

void zupper_case(buf_type buf, buf_pointer bf_ptr, buf_pointer len)
{
    buf_pointer i;
    if ((len > 0)) {
        register integer for_end;
        i = bf_ptr;
        for_end = bf_ptr + len - 1;
        if (i <= for_end)
            do
                if (((buf[i] >= 97 /*"a" */ ) && (buf[i] <= 122 /*"z" */ )))
                    buf[i] = buf[i] - 32;
            while (i++ < for_end) ;
    }
}

hash_loc zstr_lookup(buf_type buf, buf_pointer j, buf_pointer l, str_ilk ilk, boolean insert_it)
{
    register hash_loc Result;
    integer h;
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
            if ((hash_text[p] > 0)) {

                if ((str_eq_buf(hash_text[p], buf, j, l))) {

                    if ((hash_ilk[p] == ilk)) {
                        hash_found = true;
                        goto lab40;
                    } else {

                        str_num = hash_text[p];
                    }
                }
            }
        }
        if ((hash_next[p] == 0 /*empty */ )) {
            if ((!insert_it))
                goto lab45;
            {
                if ((hash_text[p] > 0)) {
                    do {
                        if (((hash_used == hash_base))) {
                            print_overflow();
                            {
                                fprintf(log_file, "%s%ld\n", "hash size ", (long)hash_size);
                                fprintf(standard_output, "%s%ld\n", "hash size ", (long)hash_size);
                            }
                            longjmp(jmp9998, 1);
                        }
                        hash_used = hash_used - 1;
                    } while (!((hash_text[hash_used] == 0)));
                    hash_next[p] = hash_used;
                    p = hash_used;
                }
                if ((str_num > 0))
                    hash_text[p] = str_num;
                else {

                    {
                        while ((pool_ptr + l > pool_size))
                            pool_overflow();
                    }
                    k = j;
                    while ((k < j + l)) {

                        {
                            str_pool[pool_ptr] = buf[k];
                            pool_ptr = pool_ptr + 1;
                        }
                        k = k + 1;
                    }
                    hash_text[p] = make_string();
                }
                hash_ilk[p] = ilk;
            }
            goto lab40;
        }
        p = hash_next[p];
    }
 lab45:                        /*str_not_found */ ;
 lab40:                        /*str_found */ Result = p;
    return Result;
}

void zpre_define(pds_type pds, pds_len len, str_ilk ilk)
{
    pds_len i;
    {
        register integer for_end;
        i = 1;
        for_end = len;
        if (i <= for_end)
            do
                buffer[i] = xord[ucharcast(pds[i - 1])];
            while (i++ < for_end);
    }
    pre_def_loc = str_lookup(buffer, 1, len, ilk, true);
}

void zzint_to_ASCII(integer the_int, buf_type int_buf, buf_pointer int_begin, buf_pointer * int_end)
{
    buf_pointer int_ptr, int_xptr;
    ASCII_code int_tmp_val;
    int_ptr = int_begin;
    if ((the_int < 0)) {
        {
            if ((int_ptr == buf_size))
                buffer_overflow();
            int_buf[int_ptr] = 45 /*minus_sign */ ;
            int_ptr = int_ptr + 1;
        }
        the_int = -(integer) the_int;
    }
    int_xptr = int_ptr;
    do {
        {
            if ((int_ptr == buf_size))
                buffer_overflow();
            int_buf[int_ptr] = 48 /*"0" */  + (the_int % 10);
            int_ptr = int_ptr + 1;
        }
        the_int = the_int / 10;
    } while (!((the_int == 0)));
    *int_end = int_ptr;
    int_ptr = int_ptr - 1;
    while ((int_xptr < int_ptr)) {

        int_tmp_val = int_buf[int_xptr];
        int_buf[int_xptr] = int_buf[int_ptr];
        int_buf[int_ptr] = int_tmp_val;
        int_ptr = int_ptr - 1;
        int_xptr = int_xptr + 1;
    }
}

void zzadd_database_cite(cite_number * new_cite)
{
    check_cite_overflow(*new_cite);
    check_field_overflow(num_fields * (*new_cite + 1));
    cite_list[*new_cite] = hash_text[cite_loc];
    ilk_info[cite_loc] = *new_cite;
    ilk_info[lc_cite_loc] = cite_loc;
    *new_cite = *new_cite + 1;
}

boolean zfind_cite_locs_for_this_cite_key(str_number cite_str)
{
    register boolean Result;
    ex_buf_ptr = 0;
    tmp_ptr = str_start[cite_str];
    tmp_end_ptr = str_start[cite_str + 1];
    while ((tmp_ptr < tmp_end_ptr)) {

        ex_buf[ex_buf_ptr] = str_pool[tmp_ptr];
        ex_buf_ptr = ex_buf_ptr + 1;
        tmp_ptr = tmp_ptr + 1;
    }
    cite_loc = str_lookup(ex_buf, 0, (str_start[cite_str + 1] - str_start[cite_str]), 9 /*cite_ilk */ , false);
    cite_hash_found = hash_found;
    lower_case(ex_buf, 0, (str_start[cite_str + 1] - str_start[cite_str]));
    lc_cite_loc = str_lookup(ex_buf, 0, (str_start[cite_str + 1] - str_start[cite_str]), 10 /*lc_cite_ilk */ , false);
    if ((hash_found))
        Result = true;
    else
        Result = false;
    return Result;
}

void zswap(cite_number swap1, cite_number swap2)
{
    cite_number innocent_bystander;
    innocent_bystander = cite_info[swap2];
    cite_info[swap2] = cite_info[swap1];
    cite_info[swap1] = innocent_bystander;
}

boolean zless_than(cite_number arg1, cite_number arg2)
{
    register boolean Result;
    integer char_ptr;
    str_ent_loc ptr1, ptr2;
    ASCII_code char1, char2;
    ptr1 = arg1 * num_ent_strs + sort_key_num;
    ptr2 = arg2 * num_ent_strs + sort_key_num;
    char_ptr = 0;
    while (true) {

        char1 = entry_strs[(ptr1) * (ent_str_size + 1) + (char_ptr)];
        char2 = entry_strs[(ptr2) * (ent_str_size + 1) + (char_ptr)];
        if ((char1 == 127 /*end_of_string */ )) {

            if ((char2 == 127 /*end_of_string */ )) {

                if ((arg1 < arg2)) {
                    Result = true;
                    goto lab10;
                } else if ((arg1 > arg2)) {
                    Result = false;
                    goto lab10;
                } else {

                    {
                        Fputs(log_file, "Duplicate sort key");
                        Fputs(standard_output, "Duplicate sort key");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
            } else {

                Result = true;
                goto lab10;
            }
        } else if ((char2 == 127 /*end_of_string */ )) {
            Result = false;
            goto lab10;
        } else if ((char1 < char2)) {
            Result = true;
            goto lab10;
        } else if ((char1 > char2)) {
            Result = false;
            goto lab10;
        }
        char_ptr = char_ptr + 1;
    }
 lab10:                        /*exit */ ;
    return Result;
}

void zquick_sort(cite_number left_end, cite_number right_end)
{
    cite_number left, right;
    cite_number insert_ptr;
    cite_number middle;
    cite_number partition;
    ;

#ifdef TRACE
    {
        fprintf(log_file, "%s%ld%s%ld\n", "Sorting ", (long)left_end, " through ", (long)right_end);
    }

#endif                          /* TRACE */
    if ((right_end - left_end < 10 /*short_list */ )) { /*305: */
        {
            register integer for_end;
            insert_ptr = left_end + 1;
            for_end = right_end;
            if (insert_ptr <= for_end)
                do {
                    {
                        register integer for_end;
                        right = insert_ptr;
                        for_end = left_end + 1;
                        if (right >= for_end)
                            do {
                                if ((less_than(cite_info[right - 1], cite_info[right])))
                                    goto lab24;
                                swap(right - 1, right);
                            }
                            while (right-- > for_end);
                    }
 lab24:                        /*next_insert */ ;
                }
                while (insert_ptr++ < for_end);
        }
    } else {

        {
            left = left_end + 4;
            middle = (left_end + right_end) / 2;
            right = right_end - 4;
            if ((less_than(cite_info[left], cite_info[middle]))) {

                if ((less_than(cite_info[middle], cite_info[right])))
                    swap(left_end, middle);
                else if ((less_than(cite_info[left], cite_info[right])))
                    swap(left_end, right);
                else
                    swap(left_end, left);
            } else if ((less_than(cite_info[right], cite_info[middle])))
                swap(left_end, middle);
            else if ((less_than(cite_info[right], cite_info[left])))
                swap(left_end, right);
            else
                swap(left_end, left);
        }
        {
            partition = cite_info[left_end];
            left = left_end + 1;
            right = right_end;
            do {
                while ((less_than(cite_info[left], partition)))
                    left = left + 1;
                while ((less_than(partition, cite_info[right])))
                    right = right - 1;
                if ((left < right)) {
                    swap(left, right);
                    left = left + 1;
                    right = right - 1;
                }
            } while (!((left == right + 1)));
            swap(left_end, right);
            quick_sort(left_end, right - 1);
            quick_sort(left, right_end);
        }
    }
}

void zzbuild_in(pds_type pds, pds_len len, hash_loc * fn_hash_loc, blt_in_range blt_in_num)
{
    pre_define(pds, len, 11 /*bst_fn_ilk */ );
    *fn_hash_loc = pre_def_loc;
    fn_type[*fn_hash_loc] = 0 /*built_in */ ;
    ilk_info[*fn_hash_loc] = blt_in_num;
    ;

#ifndef NO_BIBTEX_STAT
    blt_in_loc[blt_in_num] = *fn_hash_loc;
    execution_count[blt_in_num] = 0;

#endif                          /* not NO_BIBTEX_STAT */
}

void pre_def_certain_strings(void)
{
    pre_define(".aux        ", 4, 7 /*file_ext_ilk */ );
    s_aux_extension = hash_text[pre_def_loc];
    pre_define(".bbl        ", 4, 7 /*file_ext_ilk */ );
    s_bbl_extension = hash_text[pre_def_loc];
    pre_define(".blg        ", 4, 7 /*file_ext_ilk */ );
    s_log_extension = hash_text[pre_def_loc];
    pre_define(".bst        ", 4, 7 /*file_ext_ilk */ );
    s_bst_extension = hash_text[pre_def_loc];
    pre_define(".bib        ", 4, 7 /*file_ext_ilk */ );
    s_bib_extension = hash_text[pre_def_loc];
    pre_define("texinputs:  ", 10, 8 /*file_area_ilk */ );
    s_bst_area = hash_text[pre_def_loc];
    pre_define("texbib:     ", 7, 8 /*file_area_ilk */ );
    s_bib_area = hash_text[pre_def_loc];
    pre_define("\\citation   ", 9, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_aux_citation */ ;
    pre_define("\\bibdata    ", 8, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_aux_bibdata */ ;
    pre_define("\\bibstyle   ", 9, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_aux_bibstyle */ ;
    pre_define("\\@input     ", 7, 2 /*aux_command_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_aux_input */ ;
    pre_define("entry       ", 5, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_bst_entry */ ;
    pre_define("execute     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_bst_execute */ ;
    pre_define("function    ", 8, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_bst_function */ ;
    pre_define("integers    ", 8, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_bst_integers */ ;
    pre_define("iterate     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 4 /*n_bst_iterate */ ;
    pre_define("macro       ", 5, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 5 /*n_bst_macro */ ;
    pre_define("read        ", 4, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 6 /*n_bst_read */ ;
    pre_define("reverse     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 7 /*n_bst_reverse */ ;
    pre_define("sort        ", 4, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 8 /*n_bst_sort */ ;
    pre_define("strings     ", 7, 4 /*bst_command_ilk */ );
    ilk_info[pre_def_loc] = 9 /*n_bst_strings */ ;
    pre_define("comment     ", 7, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_bib_comment */ ;
    pre_define("preamble    ", 8, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_bib_preamble */ ;
    pre_define("string      ", 6, 12 /*bib_command_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_bib_string */ ;
    build_in("=           ", 1, b_equals, 0 /*n_equals */ );
    build_in(">           ", 1, b_greater_than, 1 /*n_greater_than */ );
    build_in("<           ", 1, b_less_than, 2 /*n_less_than */ );
    build_in("+           ", 1, b_plus, 3 /*n_plus */ );
    build_in("-           ", 1, b_minus, 4 /*n_minus */ );
    build_in("*           ", 1, b_concatenate, 5 /*n_concatenate */ );
    build_in(":=          ", 2, b_gets, 6 /*n_gets */ );
    build_in("add.period$ ", 11, b_add_period, 7 /*n_add_period */ );
    build_in("call.type$  ", 10, b_call_type, 8 /*n_call_type */ );
    build_in("change.case$", 12, b_change_case, 9 /*n_change_case */ );
    build_in("chr.to.int$ ", 11, b_chr_to_int, 10 /*n_chr_to_int */ );
    build_in("cite$       ", 5, b_cite, 11 /*n_cite */ );
    build_in("duplicate$  ", 10, b_duplicate, 12 /*n_duplicate */ );
    build_in("empty$      ", 6, b_empty, 13 /*n_empty */ );
    build_in("format.name$", 12, b_format_name, 14 /*n_format_name */ );
    build_in("if$         ", 3, b_if, 15 /*n_if */ );
    build_in("int.to.chr$ ", 11, b_int_to_chr, 16 /*n_int_to_chr */ );
    build_in("int.to.str$ ", 11, b_int_to_str, 17 /*n_int_to_str */ );
    build_in("missing$    ", 8, b_missing, 18 /*n_missing */ );
    build_in("newline$    ", 8, b_newline, 19 /*n_newline */ );
    build_in("num.names$  ", 10, b_num_names, 20 /*n_num_names */ );
    build_in("pop$        ", 4, b_pop, 21 /*n_pop */ );
    build_in("preamble$   ", 9, b_preamble, 22 /*n_preamble */ );
    build_in("purify$     ", 7, b_purify, 23 /*n_purify */ );
    build_in("quote$      ", 6, b_quote, 24 /*n_quote */ );
    build_in("skip$       ", 5, b_skip, 25 /*n_skip */ );
    build_in("stack$      ", 6, b_stack, 26 /*n_stack */ );
    build_in("substring$  ", 10, b_substring, 27 /*n_substring */ );
    build_in("swap$       ", 5, b_swap, 28 /*n_swap */ );
    build_in("text.length$", 12, b_text_length, 29 /*n_text_length */ );
    build_in("text.prefix$", 12, b_text_prefix, 30 /*n_text_prefix */ );
    build_in("top$        ", 4, b_top_stack, 31 /*n_top_stack */ );
    build_in("type$       ", 5, b_type, 32 /*n_type */ );
    build_in("warning$    ", 8, b_warning, 33 /*n_warning */ );
    build_in("while$      ", 6, b_while, 34 /*n_while */ );
    build_in("width$      ", 6, b_width, 35 /*n_width */ );
    build_in("write$      ", 6, b_write, 36 /*n_write */ );
    pre_define("            ", 0, 0 /*text_ilk */ );
    s_null = hash_text[pre_def_loc];
    fn_type[pre_def_loc] = 3 /*str_literal */ ;
    pre_define("default.type", 12, 0 /*text_ilk */ );
    s_default = hash_text[pre_def_loc];
    fn_type[pre_def_loc] = 3 /*str_literal */ ;
    b_default = b_skip;
    preamble_ptr = 0;
    pre_define("i           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 0 /*n_i */ ;
    pre_define("j           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 1 /*n_j */ ;
    pre_define("oe          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 2 /*n_oe */ ;
    pre_define("OE          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 3 /*n_oe_upper */ ;
    pre_define("ae          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 4 /*n_ae */ ;
    pre_define("AE          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 5 /*n_ae_upper */ ;
    pre_define("aa          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 6 /*n_aa */ ;
    pre_define("AA          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 7 /*n_aa_upper */ ;
    pre_define("o           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 8 /*n_o */ ;
    pre_define("O           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 9 /*n_o_upper */ ;
    pre_define("l           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 10 /*n_l */ ;
    pre_define("L           ", 1, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 11 /*n_l_upper */ ;
    pre_define("ss          ", 2, 14 /*control_seq_ilk */ );
    ilk_info[pre_def_loc] = 12 /*n_ss */ ;
    pre_define("crossref    ", 8, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 4 /*field */ ;
    ilk_info[pre_def_loc] = num_fields;
    crossref_num = num_fields;
    num_fields = num_fields + 1;
    num_pre_defined_fields = num_fields;
    pre_define("sort.key$   ", 9, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 6 /*str_entry_var */ ;
    ilk_info[pre_def_loc] = num_ent_strs;
    sort_key_num = num_ent_strs;
    num_ent_strs = num_ent_strs + 1;
    pre_define("entry.max$  ", 10, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 7 /*int_global_var */ ;
    ilk_info[pre_def_loc] = ent_str_size;
    pre_define("global.max$ ", 11, 11 /*bst_fn_ilk */ );
    fn_type[pre_def_loc] = 7 /*int_global_var */ ;
    ilk_info[pre_def_loc] = glob_str_size;
}

boolean zscan1(ASCII_code char1)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((buffer[buf_ptr2] != char1) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean zscan1_white(ASCII_code char1)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((lex_class[buffer[buf_ptr2]] != 1 /*white_space */ ) && (buffer[buf_ptr2] != char1) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean zscan2(ASCII_code char1, ASCII_code char2)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((buffer[buf_ptr2] != char1) && (buffer[buf_ptr2] != char2) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean zscan2_white(ASCII_code char1, ASCII_code char2)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((buffer[buf_ptr2] != char1) && (buffer[buf_ptr2] != char2)
            && (lex_class[buffer[buf_ptr2]] != 1 /*white_space */ ) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean zscan3(ASCII_code char1, ASCII_code char2, ASCII_code char3)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((buffer[buf_ptr2] != char1) && (buffer[buf_ptr2] != char2) && (buffer[buf_ptr2] != char3)
            && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean scan_alpha(void)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    while (((lex_class[buffer[buf_ptr2]] == 2 /*alpha */ ) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if (((buf_ptr2 - buf_ptr1) == 0))
        Result = false;
    else
        Result = true;
    return Result;
}

void zscan_identifier(ASCII_code char1, ASCII_code char2, ASCII_code char3)
{
    buf_ptr1 = buf_ptr2;
    if ((lex_class[buffer[buf_ptr2]] != 3 /*numeric */ ))
        while (((id_class[buffer[buf_ptr2]] == 1 /*legal_id_char */ ) && (buf_ptr2 < last)))
            buf_ptr2 = buf_ptr2 + 1;
    if (((buf_ptr2 - buf_ptr1) == 0))
        scan_result = 0 /*id_null */ ;
    else if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ ) || (buf_ptr2 == last)))
        scan_result = 3 /*white_adjacent */ ;
    else if (((buffer[buf_ptr2] == char1) || (buffer[buf_ptr2] == char2) || (buffer[buf_ptr2] == char3)))
        scan_result = 1 /*specified_char_adjacent */ ;
    else
        scan_result = 2 /*other_char_adjacent */ ;
}

boolean scan_nonneg_integer(void)
{
    register boolean Result;
    buf_ptr1 = buf_ptr2;
    token_value = 0;
    while (((lex_class[buffer[buf_ptr2]] == 3 /*numeric */ ) && (buf_ptr2 < last))) {

        token_value = token_value * 10 + (buffer[buf_ptr2] - 48);
        buf_ptr2 = buf_ptr2 + 1;
    }
    if (((buf_ptr2 - buf_ptr1) == 0))
        Result = false;
    else
        Result = true;
    return Result;
}

boolean scan_integer(void)
{
    register boolean Result;
    unsigned char sign_length;
    buf_ptr1 = buf_ptr2;
    if ((buffer[buf_ptr2] == 45 /*minus_sign */ )) {
        sign_length = 1;
        buf_ptr2 = buf_ptr2 + 1;
    } else
        sign_length = 0;
    token_value = 0;
    while (((lex_class[buffer[buf_ptr2]] == 3 /*numeric */ ) && (buf_ptr2 < last))) {

        token_value = token_value * 10 + (buffer[buf_ptr2] - 48);
        buf_ptr2 = buf_ptr2 + 1;
    }
    if (((sign_length == 1)))
        token_value = -(integer) token_value;
    if (((buf_ptr2 - buf_ptr1) == sign_length))
        Result = false;
    else
        Result = true;
    return Result;
}

boolean scan_white_space(void)
{
    register boolean Result;
    while (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ ) && (buf_ptr2 < last)))
        buf_ptr2 = buf_ptr2 + 1;
    if ((buf_ptr2 < last))
        Result = true;
    else
        Result = false;
    return Result;
}

boolean eat_bst_white_space(void)
{
    register boolean Result;
    while (true) {

        if ((scan_white_space())) {

            if ((buffer[buf_ptr2] != 37 /*comment */ )) {
                Result = true;
                goto lab10;
            }
        }
        if ((!input_ln(bst_file))) {
            Result = false;
            goto lab10;
        }
        bst_line_num = bst_line_num + 1;
        buf_ptr2 = 0;
    }
 lab10:                        /*exit */ ;
    return Result;
}

void skip_token_print(void)
{
    {
        putc('-', log_file);
        putc('-', standard_output);
    }
    bst_ln_num_print();
    mark_error();
    if ((scan2_white(125 /*right_brace */ , 37 /*comment */ ))) ;
}

void print_recursion_illegal(void)
{

    ;

#ifdef TRACE
    {
        putc('\n', log_file);
    }

#endif                          /* TRACE */
    {
        fprintf(log_file, "%s\n", "Curse you, wizard, before you recurse me:");
        fprintf(standard_output, "%s\n", "Curse you, wizard, before you recurse me:");
    }
    {
        Fputs(log_file, "function ");
        Fputs(standard_output, "function ");
    }
    print_a_token();
    {
        fprintf(log_file, "%s\n", " is illegal in its own definition");
        fprintf(standard_output, "%s\n", " is illegal in its own definition");
    }
    skip_token_print();
}

void skp_token_unknown_function_print(void)
{
    print_a_token();
    {
        Fputs(log_file, " is an unknown function");
        Fputs(standard_output, " is an unknown function");
    }
    skip_token_print();
}

void skip_illegal_stuff_after_token_print(void)
{
    {
        fprintf(log_file, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" can't follow a literal");
        fprintf(standard_output, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" can't follow a literal");
    }
    skip_token_print();
}

void zscan_fn_def(hash_loc fn_hash_loc)
{
    typedef integer fn_def_loc;
    hash_ptr2 *singl_function;
    integer single_fn_space;
    fn_def_loc single_ptr;
    fn_def_loc copy_ptr;
    buf_pointer end_of_num;
    hash_loc impl_fn_loc;
    single_fn_space = SINGLE_FN_SPACE;
    singl_function = XTALLOC(single_fn_space + 1, hash_ptr2);
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "function");
                    Fputs(standard_output, "function");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    single_ptr = 0;
    while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

        switch ((buffer[buf_ptr2])) {
        case 35:
            {
                buf_ptr2 = buf_ptr2 + 1;
                if ((!scan_integer())) {
                    {
                        Fputs(log_file, "Illegal integer in integer literal");
                        Fputs(standard_output, "Illegal integer in integer literal");
                    }
                    skip_token_print();
                    goto lab25;
                };

#ifdef TRACE
                {
                    putc('#', log_file);
                }
                {
                    out_token(log_file);
                }
                {
                    fprintf(log_file, "%s%ld\n", " is an integer literal with value ", (long)token_value);
                }

#endif                          /* TRACE */
                literal_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 1 /*integer_ilk */ , true);
                if ((!hash_found)) {
                    fn_type[literal_loc] = 2 /*int_literal */ ;
                    ilk_info[literal_loc] = token_value;
                }
                if (((lex_class[buffer[buf_ptr2]] != 1 /*white_space */ ) && (buf_ptr2 < last)
                     && (buffer[buf_ptr2] != 125 /*right_brace */ ) && (buffer[buf_ptr2] != 37 /*comment */ ))) {
                    skip_illegal_stuff_after_token_print();
                    goto lab25;
                }
                {
                    singl_function[single_ptr] = literal_loc;
                    if ((single_ptr == single_fn_space)) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
            }
            break;
        case 34:
            {
                buf_ptr2 = buf_ptr2 + 1;
                if ((!scan1(34 /*double_quote */ ))) {
                    {
                        fprintf(log_file, "%s%c%s", "No `", xchr[34 /*double_quote */ ], "' to end string literal");
                        fprintf(standard_output, "%s%c%s", "No `", xchr[34 /*double_quote */ ],
                                "' to end string literal");
                    }
                    skip_token_print();
                    goto lab25;
                };

#ifdef TRACE
                {
                    putc('"', log_file);
                }
                {
                    out_token(log_file);
                }
                {
                    putc('"', log_file);
                }
                {
                    fprintf(log_file, "%s\n", " is a string literal");
                }

#endif                          /* TRACE */
                literal_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 0 /*text_ilk */ , true);
                fn_type[literal_loc] = 3 /*str_literal */ ;
                buf_ptr2 = buf_ptr2 + 1;
                if (((lex_class[buffer[buf_ptr2]] != 1 /*white_space */ ) && (buf_ptr2 < last)
                     && (buffer[buf_ptr2] != 125 /*right_brace */ ) && (buffer[buf_ptr2] != 37 /*comment */ ))) {
                    skip_illegal_stuff_after_token_print();
                    goto lab25;
                }
                {
                    singl_function[single_ptr] = literal_loc;
                    if ((single_ptr == single_fn_space)) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
            }
            break;
        case 39:
            {
                buf_ptr2 = buf_ptr2 + 1;
                if ((scan2_white(125 /*right_brace */ , 37 /*comment */ ))) ;
                ;

#ifdef TRACE
                {
                    putc('\'', log_file);
                }
                {
                    out_token(log_file);
                }
                {
                    Fputs(log_file, " is a quoted function ");
                }

#endif                          /* TRACE */
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , false);
                if ((!hash_found)) {
                    skp_token_unknown_function_print();
                    goto lab25;
                } else {        /*194: */

                    if ((fn_loc == wiz_loc)) {
                        print_recursion_illegal();
                        goto lab25;
                    } else {

                        ;

#ifdef TRACE
                        {
                            Fputs(log_file, "of type ");
                        }
                        trace_pr_fn_class(fn_loc);
                        {
                            putc('\n', log_file);
                        }

#endif                          /* TRACE */
                        {
                            singl_function[single_ptr] = quote_next_fn;
                            if ((single_ptr == single_fn_space)) {
                                BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                              single_fn_space + SINGLE_FN_SPACE);
                            }
                            single_ptr = single_ptr + 1;
                        }
                        {
                            singl_function[single_ptr] = fn_loc;
                            if ((single_ptr == single_fn_space)) {
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
                ex_buf[0] = 39 /*single_quote */ ;
                int_to_ASCII(impl_fn_num, ex_buf, 1, end_of_num);
                impl_fn_loc = str_lookup(ex_buf, 0, end_of_num, 11 /*bst_fn_ilk */ , true);
                if ((hash_found)) {
                    {
                        Fputs(log_file, "Already encountered implicit function");
                        Fputs(standard_output, "Already encountered implicit function");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                };

#ifdef TRACE
                {
                    out_pool_str(log_file, hash_text[impl_fn_loc]);
                }
                {
                    fprintf(log_file, "%s\n", " is an implicit function");
                }

#endif                          /* TRACE */
                impl_fn_num = impl_fn_num + 1;
                fn_type[impl_fn_loc] = 1 /*wiz_defined */ ;
                {
                    singl_function[single_ptr] = quote_next_fn;
                    if ((single_ptr == single_fn_space)) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
                {
                    singl_function[single_ptr] = impl_fn_loc;
                    if ((single_ptr == single_fn_space)) {
                        BIB_XRETALLOC("singl_function", singl_function, hash_ptr2, single_fn_space,
                                      single_fn_space + SINGLE_FN_SPACE);
                    }
                    single_ptr = single_ptr + 1;
                }
                buf_ptr2 = buf_ptr2 + 1;
                scan_fn_def(impl_fn_loc);
            }
            break;
        default:
            {
                if ((scan2_white(125 /*right_brace */ , 37 /*comment */ ))) ;
                ;

#ifdef TRACE
                {
                    out_token(log_file);
                }
                {
                    Fputs(log_file, " is a function ");
                }

#endif                          /* TRACE */
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , false);
                if ((!hash_found)) {
                    skp_token_unknown_function_print();
                    goto lab25;
                } else if ((fn_loc == wiz_loc)) {
                    print_recursion_illegal();
                    goto lab25;
                } else {

                    ;

#ifdef TRACE
                    {
                        Fputs(log_file, "of type ");
                    }
                    trace_pr_fn_class(fn_loc);
                    {
                        putc('\n', log_file);
                    }

#endif                          /* TRACE */
                    {
                        singl_function[single_ptr] = fn_loc;
                        if ((single_ptr == single_fn_space)) {
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

            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
    }
    {
        {
            singl_function[single_ptr] = end_of_def;
            if ((single_ptr == single_fn_space)) {
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
    buf_ptr2 = buf_ptr2 + 1;
 lab10:                        /*exit */ libc_free(singl_function);
}

boolean eat_bib_white_space(void)
{
    register boolean Result;
    while ((!scan_white_space())) {

        if ((!input_ln(bib_file[bib_ptr]))) {
            Result = false;
            goto lab10;
        }
        bib_line_num = bib_line_num + 1;
        buf_ptr2 = 0;
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

boolean compress_bib_white(void)
{
    register boolean Result;
    Result = false;
    {
        if ((ex_buf_ptr == buf_size)) {
            bib_field_too_long_print();
            goto lab10;
        } else {

            ex_buf[ex_buf_ptr] = 32 /*space */ ;
            ex_buf_ptr = ex_buf_ptr + 1;
        }
    }
    while ((!scan_white_space())) {

        if ((!input_ln(bib_file[bib_ptr]))) {
            eat_bib_print();
            goto lab10;
        }
        bib_line_num = bib_line_num + 1;
        buf_ptr2 = 0;
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

boolean scan_balanced_braces(void)
{
    register boolean Result;
    Result = false;
    buf_ptr2 = buf_ptr2 + 1;
    {
        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ ) || (buf_ptr2 == last))) {

            if ((!compress_bib_white()))
                goto lab10;
        }
    }
    if ((ex_buf_ptr > 1)) {

        if ((ex_buf[ex_buf_ptr - 1] == 32 /*space */ )) {

            if ((ex_buf[ex_buf_ptr - 2] == 32 /*space */ ))
                ex_buf_ptr = ex_buf_ptr - 1;
        }
    }
    bib_brace_level = 0;
    if ((store_field)) {        /*257: */
        while ((buffer[buf_ptr2] != right_str_delim))
            switch ((buffer[buf_ptr2])) {
            case 123:
                {
                    bib_brace_level = bib_brace_level + 1;
                    {
                        if ((ex_buf_ptr == buf_size)) {
                            bib_field_too_long_print();
                            goto lab10;
                        } else {

                            ex_buf[ex_buf_ptr] = 123 /*left_brace */ ;
                            ex_buf_ptr = ex_buf_ptr + 1;
                        }
                    }
                    buf_ptr2 = buf_ptr2 + 1;
                    {
                        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ ) || (buf_ptr2 == last))) {

                            if ((!compress_bib_white()))
                                goto lab10;
                        }
                    }
                    {
                        while (true)
                            switch ((buffer[buf_ptr2])) {
                            case 125:
                                {
                                    bib_brace_level = bib_brace_level - 1;
                                    {
                                        if ((ex_buf_ptr == buf_size)) {
                                            bib_field_too_long_print();
                                            goto lab10;
                                        } else {

                                            ex_buf[ex_buf_ptr] = 125 /*right_brace */ ;
                                            ex_buf_ptr = ex_buf_ptr + 1;
                                        }
                                    }
                                    buf_ptr2 = buf_ptr2 + 1;
                                    {
                                        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )
                                             || (buf_ptr2 == last))) {

                                            if ((!compress_bib_white()))
                                                goto lab10;
                                        }
                                    }
                                    if ((bib_brace_level == 0))
                                        goto lab15;
                                }
                                break;
                            case 123:
                                {
                                    bib_brace_level = bib_brace_level + 1;
                                    {
                                        if ((ex_buf_ptr == buf_size)) {
                                            bib_field_too_long_print();
                                            goto lab10;
                                        } else {

                                            ex_buf[ex_buf_ptr] = 123 /*left_brace */ ;
                                            ex_buf_ptr = ex_buf_ptr + 1;
                                        }
                                    }
                                    buf_ptr2 = buf_ptr2 + 1;
                                    {
                                        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )
                                             || (buf_ptr2 == last))) {

                                            if ((!compress_bib_white()))
                                                goto lab10;
                                        }
                                    }
                                }
                                break;
                            default:
                                {
                                    {
                                        if ((ex_buf_ptr == buf_size)) {
                                            bib_field_too_long_print();
                                            goto lab10;
                                        } else {

                                            ex_buf[ex_buf_ptr] = buffer[buf_ptr2];
                                            ex_buf_ptr = ex_buf_ptr + 1;
                                        }
                                    }
                                    buf_ptr2 = buf_ptr2 + 1;
                                    {
                                        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )
                                             || (buf_ptr2 == last))) {

                                            if ((!compress_bib_white()))
                                                goto lab10;
                                        }
                                    }
                                }
                                break;
                            }
 lab15:                        /*loop_exit */ ;
                    }
                }
                break;
            case 125:
                {
                    bib_unbalanced_braces_print();
                    goto lab10;
                }
                break;
            default:
                {
                    {
                        if ((ex_buf_ptr == buf_size)) {
                            bib_field_too_long_print();
                            goto lab10;
                        } else {

                            ex_buf[ex_buf_ptr] = buffer[buf_ptr2];
                            ex_buf_ptr = ex_buf_ptr + 1;
                        }
                    }
                    buf_ptr2 = buf_ptr2 + 1;
                    {
                        if (((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ ) || (buf_ptr2 == last))) {

                            if ((!compress_bib_white()))
                                goto lab10;
                        }
                    }
                }
                break;
            }
    } else {                    /*255: */

        while ((buffer[buf_ptr2] != right_str_delim))
            if ((buffer[buf_ptr2] == 123 /*left_brace */ )) {
                bib_brace_level = bib_brace_level + 1;
                buf_ptr2 = buf_ptr2 + 1;
                {
                    if ((!eat_bib_white_space())) {
                        eat_bib_print();
                        goto lab10;
                    }
                }
                while ((bib_brace_level > 0)) { /*256: */

                    if ((buffer[buf_ptr2] == 125 /*right_brace */ )) {
                        bib_brace_level = bib_brace_level - 1;
                        buf_ptr2 = buf_ptr2 + 1;
                        {
                            if ((!eat_bib_white_space())) {
                                eat_bib_print();
                                goto lab10;
                            }
                        }
                    } else if ((buffer[buf_ptr2] == 123 /*left_brace */ )) {
                        bib_brace_level = bib_brace_level + 1;
                        buf_ptr2 = buf_ptr2 + 1;
                        {
                            if ((!eat_bib_white_space())) {
                                eat_bib_print();
                                goto lab10;
                            }
                        }
                    } else {

                        buf_ptr2 = buf_ptr2 + 1;
                        if ((!scan2(125 /*right_brace */ , 123 /*left_brace */ ))) {
                            if ((!eat_bib_white_space())) {
                                eat_bib_print();
                                goto lab10;
                            }
                        }
                    }
                }
            } else if ((buffer[buf_ptr2] == 125 /*right_brace */ )) {
                bib_unbalanced_braces_print();
                goto lab10;
            } else {

                buf_ptr2 = buf_ptr2 + 1;
                if ((!scan3(right_str_delim, 123 /*left_brace */ , 125 /*right_brace */ ))) {
                    if ((!eat_bib_white_space())) {
                        eat_bib_print();
                        goto lab10;
                    }
                }
            }
    }
    buf_ptr2 = buf_ptr2 + 1;
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

boolean scan_a_field_token_and_eat_white(void)
{
    register boolean Result;
    Result = false;
    switch ((buffer[buf_ptr2])) {
    case 123:
        {
            right_str_delim = 125 /*right_brace */ ;
            if ((!scan_balanced_braces()))
                goto lab10;
        }
        break;
    case 34:
        {
            right_str_delim = 34 /*double_quote */ ;
            if ((!scan_balanced_braces()))
                goto lab10;
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
            if ((!scan_nonneg_integer())) {
                {
                    Fputs(log_file, "A digit disappeared");
                    Fputs(standard_output, "A digit disappeared");
                }
                print_confusion();
                longjmp(jmp9998, 1);
            }
            if ((store_field)) {
                tmp_ptr = buf_ptr1;
                while ((tmp_ptr < buf_ptr2)) {

                    {
                        if ((ex_buf_ptr == buf_size)) {
                            bib_field_too_long_print();
                            goto lab10;
                        } else {

                            ex_buf[ex_buf_ptr] = buffer[tmp_ptr];
                            ex_buf_ptr = ex_buf_ptr + 1;
                        }
                    }
                    tmp_ptr = tmp_ptr + 1;
                }
            }
        }
        break;
    default:
        {
            scan_identifier(44 /*comma */ , right_outer_delim, 35 /*concat_char */ );
            {
                if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                else {

                    bib_id_print();
                    {
                        {
                            Fputs(log_file, "a field part");
                            Fputs(standard_output, "a field part");
                        }
                        bib_err_print();
                        goto lab10;
                    }
                }
            }
            if ((store_field)) {
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                macro_name_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 13 /*macro_ilk */ , false);
                store_token = true;
                if ((at_bib_command)) {

                    if ((command_num == 2 /*n_bib_string */ )) {

                        if ((macro_name_loc == cur_macro_loc)) {
                            store_token = false;
                            {
                                macro_warn_print();
                                {
                                    {
                                        fprintf(log_file, "%s\n", "used in its own definition");
                                        fprintf(standard_output, "%s\n", "used in its own definition");
                                    }
                                    bib_warn_print();
                                }
                            }
                        }
                    }
                }
                if ((!hash_found)) {
                    store_token = false;
                    {
                        macro_warn_print();
                        {
                            {
                                fprintf(log_file, "%s\n", "undefined");
                                fprintf(standard_output, "%s\n", "undefined");
                            }
                            bib_warn_print();
                        }
                    }
                }
                if ((store_token)) {    /*261: */
                    tmp_ptr = str_start[ilk_info[macro_name_loc]];
                    tmp_end_ptr = str_start[ilk_info[macro_name_loc] + 1];
                    if ((ex_buf_ptr == 0)) {

                        if (((lex_class[str_pool[tmp_ptr]] == 1 /*white_space */ ) && (tmp_ptr < tmp_end_ptr))) {
                            {
                                if ((ex_buf_ptr == buf_size)) {
                                    bib_field_too_long_print();
                                    goto lab10;
                                } else {

                                    ex_buf[ex_buf_ptr] = 32 /*space */ ;
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                }
                            }
                            tmp_ptr = tmp_ptr + 1;
                            while (((lex_class[str_pool[tmp_ptr]] == 1 /*white_space */ ) && (tmp_ptr < tmp_end_ptr)))
                                tmp_ptr = tmp_ptr + 1;
                        }
                    }
                    while ((tmp_ptr < tmp_end_ptr)) {

                        if ((lex_class[str_pool[tmp_ptr]] != 1 /*white_space */ )) {
                            if ((ex_buf_ptr == buf_size)) {
                                bib_field_too_long_print();
                                goto lab10;
                            } else {

                                ex_buf[ex_buf_ptr] = str_pool[tmp_ptr];
                                ex_buf_ptr = ex_buf_ptr + 1;
                            }
                        } else if ((ex_buf[ex_buf_ptr - 1] != 32 /*space */ )) {
                            if ((ex_buf_ptr == buf_size)) {
                                bib_field_too_long_print();
                                goto lab10;
                            } else {

                                ex_buf[ex_buf_ptr] = 32 /*space */ ;
                                ex_buf_ptr = ex_buf_ptr + 1;
                            }
                        }
                        tmp_ptr = tmp_ptr + 1;
                    }
                }
            }
        }
        break;
    }
    {
        if ((!eat_bib_white_space())) {
            eat_bib_print();
            goto lab10;
        }
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

boolean scan_and_store_the_field_value_and_eat_white(void)
{
    register boolean Result;
    Result = false;
    ex_buf_ptr = 0;
    if ((!scan_a_field_token_and_eat_white()))
        goto lab10;
    while ((buffer[buf_ptr2] == 35 /*concat_char */ )) {

        buf_ptr2 = buf_ptr2 + 1;
        {
            if ((!eat_bib_white_space())) {
                eat_bib_print();
                goto lab10;
            }
        }
        if ((!scan_a_field_token_and_eat_white()))
            goto lab10;
    }
    if ((store_field)) {        /*262: */
        if ((!at_bib_command)) {

            if ((ex_buf_ptr > 0)) {

                if ((ex_buf[ex_buf_ptr - 1] == 32 /*space */ ))
                    ex_buf_ptr = ex_buf_ptr - 1;
            }
        }
        if (((!at_bib_command) && (ex_buf[0] == 32 /*space */ ) && (ex_buf_ptr > 0)))
            ex_buf_xptr = 1;
        else
            ex_buf_xptr = 0;
        field_val_loc = str_lookup(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr, 0 /*text_ilk */ , true);
        fn_type[field_val_loc] = 3 /*str_literal */ ;
        ;

#ifdef TRACE
        {
            putc('"', log_file);
        }
        {
            out_pool_str(log_file, hash_text[field_val_loc]);
        }
        {
            fprintf(log_file, "%s\n", "\" is a field value");
        }

#endif                          /* TRACE */
        if ((at_bib_command)) { /*263: */
            switch ((command_num)) {
            case 1:
                {
                    s_preamble[preamble_ptr] = hash_text[field_val_loc];
                    preamble_ptr = preamble_ptr + 1;
                }
                break;
            case 2:
                ilk_info[cur_macro_loc] = hash_text[field_val_loc];
                break;
            default:
                bib_cmd_confusion();
                break;
            }
        } else {                /*264: */

            field_ptr = entry_cite_ptr * num_fields + ilk_info[field_name_loc];
            if ((field_ptr >= max_fields)) {
                {
                    Fputs(log_file, "field_info index is out of range");
                    Fputs(standard_output, "field_info index is out of range");
                }
                print_confusion();
                longjmp(jmp9998, 1);
            }
            if ((field_info[field_ptr] != 0 /*missing */ )) {
                {
                    Fputs(log_file, "Warning--I'm ignoring ");
                    Fputs(standard_output, "Warning--I'm ignoring ");
                }
                print_a_pool_str(cite_list[entry_cite_ptr]);
                {
                    Fputs(log_file, "'s extra \"");
                    Fputs(standard_output, "'s extra \"");
                }
                print_a_pool_str(hash_text[field_name_loc]);
                {
                    {
                        fprintf(log_file, "%s\n", "\" field");
                        fprintf(standard_output, "%s\n", "\" field");
                    }
                    bib_warn_print();
                }
            } else {

                field_info[field_ptr] = hash_text[field_val_loc];
                if (((ilk_info[field_name_loc] == crossref_num) && (!all_entries))) {   /*265: */
                    tmp_ptr = ex_buf_xptr;
                    while ((tmp_ptr < ex_buf_ptr)) {

                        out_buf[tmp_ptr] = ex_buf[tmp_ptr];
                        tmp_ptr = tmp_ptr + 1;
                    }
                    lower_case(out_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                    lc_cite_loc =
                        str_lookup(out_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr, 10 /*lc_cite_ilk */ , true);
                    if ((hash_found)) {
                        cite_loc = ilk_info[lc_cite_loc];
                        if ((ilk_info[cite_loc] >= old_num_cites))
                            cite_info[ilk_info[cite_loc]] = cite_info[ilk_info[cite_loc]] + 1;
                    } else {

                        cite_loc = str_lookup(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr, 9 /*cite_ilk */ , true);
                        if ((hash_found))
                            hash_cite_confusion();
                        add_database_cite(cite_ptr);
                        cite_info[ilk_info[cite_loc]] = 1;
                    }
                }
            }
        }
    }
    Result = true;
 lab10:                        /*exit */ ;
    return Result;
}

void zdecr_brace_level(str_number pop_lit_var)
{
    if ((brace_level == 0))
        braces_unbalanced_complaint(pop_lit_var);
    else
        brace_level = brace_level - 1;
}

void zcheck_brace_level(str_number pop_lit_var)
{
    if ((brace_level > 0))
        braces_unbalanced_complaint(pop_lit_var);
}

void zname_scan_for_and(str_number pop_lit_var)
{
    brace_level = 0;
    preceding_white = false;
    and_found = false;
    while (((!and_found) && (ex_buf_ptr < ex_buf_length)))
        switch ((ex_buf[ex_buf_ptr])) {
        case 97:
        case 65:
            {
                ex_buf_ptr = ex_buf_ptr + 1;
                if ((preceding_white)) {        /*387: */
                    if ((ex_buf_ptr <= (ex_buf_length - 3))) {

                        if (((ex_buf[ex_buf_ptr] == 110 /*"n" */ ) || (ex_buf[ex_buf_ptr] == 78 /*"N" */ ))) {

                            if (((ex_buf[ex_buf_ptr + 1] == 100 /*"d" */ ) || (ex_buf[ex_buf_ptr + 1] == 68 /*"D" */ ))) {

                                if ((lex_class[ex_buf[ex_buf_ptr + 2]] == 1 /*white_space */ )) {
                                    ex_buf_ptr = ex_buf_ptr + 2;
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
                ex_buf_ptr = ex_buf_ptr + 1;
                while (((brace_level > 0) && (ex_buf_ptr < ex_buf_length))) {

                    if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ ))
                        brace_level = brace_level - 1;
                    else if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ ))
                        brace_level = brace_level + 1;
                    ex_buf_ptr = ex_buf_ptr + 1;
                }
                preceding_white = false;
            }
            break;
        case 125:
            {
                decr_brace_level(pop_lit_var);
                ex_buf_ptr = ex_buf_ptr + 1;
                preceding_white = false;
            }
            break;
        default:
            if ((lex_class[ex_buf[ex_buf_ptr]] == 1 /*white_space */ )) {
                ex_buf_ptr = ex_buf_ptr + 1;
                preceding_white = true;
            } else {

                ex_buf_ptr = ex_buf_ptr + 1;
                preceding_white = false;
            }
            break;
        }
    check_brace_level(pop_lit_var);
}

boolean von_token_found(void)
{
    register boolean Result;
    nm_brace_level = 0;
    Result = false;
    while ((name_bf_ptr < name_bf_xptr))
        if (((sv_buffer[name_bf_ptr] >= 65 /*"A" */ ) && (sv_buffer[name_bf_ptr] <= 90 /*"Z" */ )))
            goto lab10;
        else if (((sv_buffer[name_bf_ptr] >= 97 /*"a" */ ) && (sv_buffer[name_bf_ptr] <= 122 /*"z" */ ))) {
            Result = true;
            goto lab10;
        } else if ((sv_buffer[name_bf_ptr] == 123 /*left_brace */ )) {
            nm_brace_level = nm_brace_level + 1;
            name_bf_ptr = name_bf_ptr + 1;
            if (((name_bf_ptr + 2 < name_bf_xptr) && (sv_buffer[name_bf_ptr] == 92 /*backslash */ ))) { /*399: */
                name_bf_ptr = name_bf_ptr + 1;
                name_bf_yptr = name_bf_ptr;
                while (((name_bf_ptr < name_bf_xptr) && (lex_class[sv_buffer[name_bf_ptr]] == 2 /*alpha */ )))
                    name_bf_ptr = name_bf_ptr + 1;
                control_seq_loc =
                    str_lookup(sv_buffer, name_bf_yptr, name_bf_ptr - name_bf_yptr, 14 /*control_seq_ilk */ , false);
                if ((hash_found)) {     /*400: */
                    switch ((ilk_info[control_seq_loc])) {
                    case 3:
                    case 5:
                    case 7:
                    case 9:
                    case 11:
                        goto lab10;
                        break;
                    case 0:
                    case 1:
                    case 2:
                    case 4:
                    case 6:
                    case 8:
                    case 10:
                    case 12:
                        {
                            Result = true;
                            goto lab10;
                        }
                        break;
                    default:
                        {
                            {
                                Fputs(log_file, "Control-sequence hash error");
                                Fputs(standard_output, "Control-sequence hash error");
                            }
                            print_confusion();
                            longjmp(jmp9998, 1);
                        }
                        break;
                    }
                }
                while (((name_bf_ptr < name_bf_xptr) && (nm_brace_level > 0))) {

                    if (((sv_buffer[name_bf_ptr] >= 65 /*"A" */ ) && (sv_buffer[name_bf_ptr] <= 90 /*"Z" */ )))
                        goto lab10;
                    else if (((sv_buffer[name_bf_ptr] >= 97 /*"a" */ ) && (sv_buffer[name_bf_ptr] <= 122 /*"z" */ ))) {
                        Result = true;
                        goto lab10;
                    } else if ((sv_buffer[name_bf_ptr] == 125 /*right_brace */ ))
                        nm_brace_level = nm_brace_level - 1;
                    else if ((sv_buffer[name_bf_ptr] == 123 /*left_brace */ ))
                        nm_brace_level = nm_brace_level + 1;
                    name_bf_ptr = name_bf_ptr + 1;
                }
                goto lab10;
            } else              /*401: */
                while (((nm_brace_level > 0) && (name_bf_ptr < name_bf_xptr))) {

                    if ((sv_buffer[name_bf_ptr] == 125 /*right_brace */ ))
                        nm_brace_level = nm_brace_level - 1;
                    else if ((sv_buffer[name_bf_ptr] == 123 /*left_brace */ ))
                        nm_brace_level = nm_brace_level + 1;
                    name_bf_ptr = name_bf_ptr + 1;
                }
        } else
            name_bf_ptr = name_bf_ptr + 1;
 lab10:                        /*exit */ ;
    return Result;
}

void von_name_ends_and_last_name_starts_stuff(void)
{
    von_end = last_end - 1;
    while ((von_end > von_start)) {

        name_bf_ptr = name_tok[von_end - 1];
        name_bf_xptr = name_tok[von_end];
        if ((von_token_found()))
            goto lab10;
        von_end = von_end - 1;
    }
 lab10:                        /*exit */ ;
}

void skip_stuff_at_sp_brace_level_greater_than_one(void)
{
    while (((sp_brace_level > 1) && (sp_ptr < sp_end))) {

        if ((str_pool[sp_ptr] == 125 /*right_brace */ ))
            sp_brace_level = sp_brace_level - 1;
        else if ((str_pool[sp_ptr] == 123 /*left_brace */ ))
            sp_brace_level = sp_brace_level + 1;
        sp_ptr = sp_ptr + 1;
    }
}

void brace_lvl_one_letters_complaint(void)
{
    {
        Fputs(log_file, "The format string \"");
        Fputs(standard_output, "The format string \"");
    }
    print_a_pool_str(pop_lit1);
    {
        {
            Fputs(log_file, "\" has an illegal brace-level-1 letter");
            Fputs(standard_output, "\" has an illegal brace-level-1 letter");
        }
        bst_ex_warn_print();
    }
}

boolean zenough_text_chars(buf_pointer enough_chars)
{
    register boolean Result;
    num_text_chars = 0;
    ex_buf_yptr = ex_buf_xptr;
    while (((ex_buf_yptr < ex_buf_ptr) && (num_text_chars < enough_chars))) {

        ex_buf_yptr = ex_buf_yptr + 1;
        if ((ex_buf[ex_buf_yptr - 1] == 123 /*left_brace */ )) {
            brace_level = brace_level + 1;
            if (((brace_level == 1) && (ex_buf_yptr < ex_buf_ptr))) {

                if ((ex_buf[ex_buf_yptr] == 92 /*backslash */ )) {
                    ex_buf_yptr = ex_buf_yptr + 1;
                    while (((ex_buf_yptr < ex_buf_ptr) && (brace_level > 0))) {

                        if ((ex_buf[ex_buf_yptr] == 125 /*right_brace */ ))
                            brace_level = brace_level - 1;
                        else if ((ex_buf[ex_buf_yptr] == 123 /*left_brace */ ))
                            brace_level = brace_level + 1;
                        ex_buf_yptr = ex_buf_yptr + 1;
                    }
                }
            }
        } else if ((ex_buf[ex_buf_yptr - 1] == 125 /*right_brace */ ))
            brace_level = brace_level - 1;
        num_text_chars = num_text_chars + 1;
    }
    if ((num_text_chars < enough_chars))
        Result = false;
    else
        Result = true;
    return Result;
}

void figure_out_the_formatted_name(void)
{
    {
        ex_buf_ptr = 0;
        sp_brace_level = 0;
        sp_ptr = str_start[pop_lit1];
        sp_end = str_start[pop_lit1 + 1];
        while ((sp_ptr < sp_end))
            if ((str_pool[sp_ptr] == 123 /*left_brace */ )) {
                sp_brace_level = sp_brace_level + 1;
                sp_ptr = sp_ptr + 1;
                {
                    sp_xptr1 = sp_ptr;
                    alpha_found = false;
                    double_letter = false;
                    end_of_group = false;
                    to_be_written = true;
                    while (((!end_of_group) && (sp_ptr < sp_end)))
                        if ((lex_class[str_pool[sp_ptr]] == 2 /*alpha */ )) {
                            sp_ptr = sp_ptr + 1;
                            {
                                if ((alpha_found)) {
                                    brace_lvl_one_letters_complaint();
                                    to_be_written = false;
                                } else {

                                    switch ((str_pool[sp_ptr - 1])) {
                                    case 102:
                                    case 70:
                                        {
                                            cur_token = first_start;
                                            last_token = first_end;
                                            if ((cur_token == last_token))
                                                to_be_written = false;
                                            if (((str_pool[sp_ptr] == 102 /*"f" */ )
                                                 || (str_pool[sp_ptr] == 70 /*"F" */ )))
                                                double_letter = true;
                                        }
                                        break;
                                    case 118:
                                    case 86:
                                        {
                                            cur_token = von_start;
                                            last_token = von_end;
                                            if ((cur_token == last_token))
                                                to_be_written = false;
                                            if (((str_pool[sp_ptr] == 118 /*"v" */ )
                                                 || (str_pool[sp_ptr] == 86 /*"V" */ )))
                                                double_letter = true;
                                        }
                                        break;
                                    case 108:
                                    case 76:
                                        {
                                            cur_token = von_end;
                                            last_token = last_end;
                                            if ((cur_token == last_token))
                                                to_be_written = false;
                                            if (((str_pool[sp_ptr] == 108 /*"l" */ )
                                                 || (str_pool[sp_ptr] == 76 /*"L" */ )))
                                                double_letter = true;
                                        }
                                        break;
                                    case 106:
                                    case 74:
                                        {
                                            cur_token = last_end;
                                            last_token = jr_end;
                                            if ((cur_token == last_token))
                                                to_be_written = false;
                                            if (((str_pool[sp_ptr] == 106 /*"j" */ )
                                                 || (str_pool[sp_ptr] == 74 /*"J" */ )))
                                                double_letter = true;
                                        }
                                        break;
                                    default:
                                        {
                                            brace_lvl_one_letters_complaint();
                                            to_be_written = false;
                                        }
                                        break;
                                    }
                                    if ((double_letter))
                                        sp_ptr = sp_ptr + 1;
                                }
                                alpha_found = true;
                            }
                        } else if ((str_pool[sp_ptr] == 125 /*right_brace */ )) {
                            sp_brace_level = sp_brace_level - 1;
                            sp_ptr = sp_ptr + 1;
                            end_of_group = true;
                        } else if ((str_pool[sp_ptr] == 123 /*left_brace */ )) {
                            sp_brace_level = sp_brace_level + 1;
                            sp_ptr = sp_ptr + 1;
                            skip_stuff_at_sp_brace_level_greater_than_one();
                        } else
                            sp_ptr = sp_ptr + 1;
                    if (((end_of_group) && (to_be_written))) {  /*412: */
                        ex_buf_xptr = ex_buf_ptr;
                        sp_ptr = sp_xptr1;
                        sp_brace_level = 1;
                        while ((sp_brace_level > 0))
                            if (((lex_class[str_pool[sp_ptr]] == 2 /*alpha */ ) && (sp_brace_level == 1))) {
                                sp_ptr = sp_ptr + 1;
                                {
                                    if ((double_letter))
                                        sp_ptr = sp_ptr + 1;
                                    use_default = true;
                                    sp_xptr2 = sp_ptr;
                                    if ((str_pool[sp_ptr] == 123 /*left_brace */ )) {
                                        use_default = false;
                                        sp_brace_level = sp_brace_level + 1;
                                        sp_ptr = sp_ptr + 1;
                                        sp_xptr1 = sp_ptr;
                                        skip_stuff_at_sp_brace_level_greater_than_one();
                                        sp_xptr2 = sp_ptr - 1;
                                    }
                                    while ((cur_token < last_token)) {

                                        if ((double_letter)) {  /*415: */
                                            name_bf_ptr = name_tok[cur_token];
                                            name_bf_xptr = name_tok[cur_token + 1];
                                            if ((ex_buf_length + (name_bf_xptr - name_bf_ptr) > buf_size))
                                                buffer_overflow();
                                            while ((name_bf_ptr < name_bf_xptr)) {

                                                {
                                                    ex_buf[ex_buf_ptr] = sv_buffer[name_bf_ptr];
                                                    ex_buf_ptr = ex_buf_ptr + 1;
                                                }
                                                name_bf_ptr = name_bf_ptr + 1;
                                            }
                                        } else {        /*416: */

                                            name_bf_ptr = name_tok[cur_token];
                                            name_bf_xptr = name_tok[cur_token + 1];
                                            while ((name_bf_ptr < name_bf_xptr)) {

                                                if ((lex_class[sv_buffer[name_bf_ptr]] == 2 /*alpha */ )) {
                                                    {
                                                        if ((ex_buf_ptr == buf_size))
                                                            buffer_overflow();
                                                        {
                                                            ex_buf[ex_buf_ptr] = sv_buffer[name_bf_ptr];
                                                            ex_buf_ptr = ex_buf_ptr + 1;
                                                        }
                                                    }
                                                    goto lab15;
                                                } else
                                                    if (((sv_buffer[name_bf_ptr] == 123 /*left_brace */ )
                                                         && (name_bf_ptr + 1 < name_bf_xptr))) {

                                                    if ((sv_buffer[name_bf_ptr + 1] == 92 /*backslash */ )) {   /*417: */
                                                        if ((ex_buf_ptr + 2 > buf_size))
                                                            buffer_overflow();
                                                        {
                                                            ex_buf[ex_buf_ptr] = 123 /*left_brace */ ;
                                                            ex_buf_ptr = ex_buf_ptr + 1;
                                                        }
                                                        {
                                                            ex_buf[ex_buf_ptr] = 92 /*backslash */ ;
                                                            ex_buf_ptr = ex_buf_ptr + 1;
                                                        }
                                                        name_bf_ptr = name_bf_ptr + 2;
                                                        nm_brace_level = 1;
                                                        while (((name_bf_ptr < name_bf_xptr) && (nm_brace_level > 0))) {

                                                            if ((sv_buffer[name_bf_ptr] == 125 /*right_brace */ ))
                                                                nm_brace_level = nm_brace_level - 1;
                                                            else if ((sv_buffer[name_bf_ptr] == 123 /*left_brace */ ))
                                                                nm_brace_level = nm_brace_level + 1;
                                                            {
                                                                if ((ex_buf_ptr == buf_size))
                                                                    buffer_overflow();
                                                                {
                                                                    ex_buf[ex_buf_ptr] = sv_buffer[name_bf_ptr];
                                                                    ex_buf_ptr = ex_buf_ptr + 1;
                                                                }
                                                            }
                                                            name_bf_ptr = name_bf_ptr + 1;
                                                        }
                                                        goto lab15;
                                                    }
                                                }
                                                name_bf_ptr = name_bf_ptr + 1;
                                            }
 lab15:                        /*loop_exit */ ;
                                        }
                                        cur_token = cur_token + 1;
                                        if ((cur_token < last_token)) { /*418: */
                                            if ((use_default)) {
                                                if ((!double_letter)) {
                                                    if ((ex_buf_ptr == buf_size))
                                                        buffer_overflow();
                                                    {
                                                        ex_buf[ex_buf_ptr] = 46 /*period */ ;
                                                        ex_buf_ptr = ex_buf_ptr + 1;
                                                    }
                                                }
                                                if ((lex_class[name_sep_char[cur_token]] == 4 /*sep_char */ )) {
                                                    if ((ex_buf_ptr == buf_size))
                                                        buffer_overflow();
                                                    {
                                                        ex_buf[ex_buf_ptr] = name_sep_char[cur_token];
                                                        ex_buf_ptr = ex_buf_ptr + 1;
                                                    }
                                                } else
                                                    if (((cur_token == last_token - 1)
                                                         || (!enough_text_chars(3 /*long_token */ )))) {
                                                    if ((ex_buf_ptr == buf_size))
                                                        buffer_overflow();
                                                    {
                                                        ex_buf[ex_buf_ptr] = 126 /*tie */ ;
                                                        ex_buf_ptr = ex_buf_ptr + 1;
                                                    }
                                                } else {

                                                    if ((ex_buf_ptr == buf_size))
                                                        buffer_overflow();
                                                    {
                                                        ex_buf[ex_buf_ptr] = 32 /*space */ ;
                                                        ex_buf_ptr = ex_buf_ptr + 1;
                                                    }
                                                }
                                            } else {

                                                if ((ex_buf_length + (sp_xptr2 - sp_xptr1) > buf_size))
                                                    buffer_overflow();
                                                sp_ptr = sp_xptr1;
                                                while ((sp_ptr < sp_xptr2)) {

                                                    {
                                                        ex_buf[ex_buf_ptr] = str_pool[sp_ptr];
                                                        ex_buf_ptr = ex_buf_ptr + 1;
                                                    }
                                                    sp_ptr = sp_ptr + 1;
                                                }
                                            }
                                        }
                                    }
                                    if ((!use_default))
                                        sp_ptr = sp_xptr2 + 1;
                                }
                            } else if ((str_pool[sp_ptr] == 125 /*right_brace */ )) {
                                sp_brace_level = sp_brace_level - 1;
                                sp_ptr = sp_ptr + 1;
                                if ((sp_brace_level > 0)) {
                                    if ((ex_buf_ptr == buf_size))
                                        buffer_overflow();
                                    {
                                        ex_buf[ex_buf_ptr] = 125 /*right_brace */ ;
                                        ex_buf_ptr = ex_buf_ptr + 1;
                                    }
                                }
                            } else if ((str_pool[sp_ptr] == 123 /*left_brace */ )) {
                                sp_brace_level = sp_brace_level + 1;
                                sp_ptr = sp_ptr + 1;
                                {
                                    if ((ex_buf_ptr == buf_size))
                                        buffer_overflow();
                                    {
                                        ex_buf[ex_buf_ptr] = 123 /*left_brace */ ;
                                        ex_buf_ptr = ex_buf_ptr + 1;
                                    }
                                }
                            } else {

                                {
                                    if ((ex_buf_ptr == buf_size))
                                        buffer_overflow();
                                    {
                                        ex_buf[ex_buf_ptr] = str_pool[sp_ptr];
                                        ex_buf_ptr = ex_buf_ptr + 1;
                                    }
                                }
                                sp_ptr = sp_ptr + 1;
                            }
                        if ((ex_buf_ptr > 0)) {

                            if ((ex_buf[ex_buf_ptr - 1] == 126 /*tie */ )) {    /*420: */
                                ex_buf_ptr = ex_buf_ptr - 1;
                                if ((ex_buf[ex_buf_ptr - 1] == 126 /*tie */ )) ;
                                else if ((!enough_text_chars(3 /*long_name */ )))
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                else {

                                    ex_buf[ex_buf_ptr] = 32 /*space */ ;
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                }
                            }
                        }
                    }
                }
            } else if ((str_pool[sp_ptr] == 125 /*right_brace */ )) {
                braces_unbalanced_complaint(pop_lit1);
                sp_ptr = sp_ptr + 1;
            } else {

                {
                    if ((ex_buf_ptr == buf_size))
                        buffer_overflow();
                    {
                        ex_buf[ex_buf_ptr] = str_pool[sp_ptr];
                        ex_buf_ptr = ex_buf_ptr + 1;
                    }
                }
                sp_ptr = sp_ptr + 1;
            }
        if ((sp_brace_level > 0))
            braces_unbalanced_complaint(pop_lit1);
        ex_buf_length = ex_buf_ptr;
    }
}

void zpush_lit_stk(integer push_lt, stk_type push_type)
{

#ifdef TRACE
    lit_stk_loc dum_ptr;

#endif                          /* TRACE */
    lit_stack[lit_stk_ptr] = push_lt;
    lit_stk_type[lit_stk_ptr] = push_type;
    ;

#ifdef TRACE
    {
        register integer for_end;
        dum_ptr = 0;
        for_end = lit_stk_ptr;
        if (dum_ptr <= for_end)
            do {
                Fputs(log_file, "  ");
            }
            while (dum_ptr++ < for_end);
    }
    {
        Fputs(log_file, "Pushing ");
    }
    switch ((lit_stk_type[lit_stk_ptr])) {
    case 0:
        {
            fprintf(log_file, "%ld\n", (long)lit_stack[lit_stk_ptr]);
        }
        break;
    case 1:
        {
            {
                putc('"', log_file);
            }
            {
                out_pool_str(log_file, lit_stack[lit_stk_ptr]);
            }
            {
                {
                    putc('"', log_file);
                    putc('\n', log_file);
                }
            }
        }
        break;
    case 2:
        {
            {
                putc('`', log_file);
            }
            {
                out_pool_str(log_file, hash_text[lit_stack[lit_stk_ptr]]);
            }
            {
                {
                    putc('\'', log_file);
                    putc('\n', log_file);
                }
            }
        }
        break;
    case 3:
        {
            {
                Fputs(log_file, "missing field `");
            }
            {
                out_pool_str(log_file, lit_stack[lit_stk_ptr]);
            }
            {
                {
                    putc('\'', log_file);
                    putc('\n', log_file);
                }
            }
        }
        break;
    case 4:
        {
            fprintf(log_file, "%s\n", "a bad literal--popped from an empty stack");
        }
        break;
    default:
        unknwn_literal_confusion();
        break;
    }

#endif                          /* TRACE */
    if ((lit_stk_ptr == lit_stk_size)) {
        BIB_XRETALLOC_NOSET("lit_stack", lit_stack, integer, lit_stk_size, lit_stk_size + LIT_STK_SIZE);
        BIB_XRETALLOC("lit_stk_type", lit_stk_type, stk_type, lit_stk_size, lit_stk_size + LIT_STK_SIZE);
    }
    lit_stk_ptr = lit_stk_ptr + 1;
}

void zzpop_lit_stk(integer * pop_lit, stk_type * pop_type)
{
    if ((lit_stk_ptr == 0)) {
        {
            {
                Fputs(log_file, "You can't pop an empty literal stack");
                Fputs(standard_output, "You can't pop an empty literal stack");
            }
            bst_ex_warn_print();
        }
        *pop_type = 4 /*stk_empty */ ;
    } else {

        lit_stk_ptr = lit_stk_ptr - 1;
        *pop_lit = lit_stack[lit_stk_ptr];
        *pop_type = lit_stk_type[lit_stk_ptr];
        if ((*pop_type == 1 /*stk_str */ )) {

            if ((*pop_lit >= cmd_str_ptr)) {
                if ((*pop_lit != str_ptr - 1)) {
                    {
                        Fputs(log_file, "Nontop top of string stack");
                        Fputs(standard_output, "Nontop top of string stack");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
                {
                    str_ptr = str_ptr - 1;
                    pool_ptr = str_start[str_ptr];
                }
            }
        }
    }
}

void zprint_wrong_stk_lit(integer stk_lt, stk_type stk_tp1, stk_type stk_tp2)
{
    if ((stk_tp1 != 4 /*stk_empty */ )) {
        print_stk_lit(stk_lt, stk_tp1);
        switch ((stk_tp2)) {
        case 0:
            {
                Fputs(log_file, ", not an integer,");
                Fputs(standard_output, ", not an integer,");
            }
            break;
        case 1:
            {
                Fputs(log_file, ", not a string,");
                Fputs(standard_output, ", not a string,");
            }
            break;
        case 2:
            {
                Fputs(log_file, ", not a function,");
                Fputs(standard_output, ", not a function,");
            }
            break;
        case 3:
        case 4:
            illegl_literal_confusion();
            break;
        default:
            unknwn_literal_confusion();
            break;
        }
        bst_ex_warn_print();
    }
}

void pop_top_and_print(void)
{
    integer stk_lt;
    stk_type stk_tp;
    pop_lit_stk(stk_lt, stk_tp);
    if ((stk_tp == 4 /*stk_empty */ )) {
        fprintf(log_file, "%s\n", "Empty literal");
        fprintf(standard_output, "%s\n", "Empty literal");
    } else
        print_lit(stk_lt, stk_tp);
}

void pop_whole_stack(void)
{
    while ((lit_stk_ptr > 0))
        pop_top_and_print();
}

void init_command_execution(void)
{
    lit_stk_ptr = 0;
    cmd_str_ptr = str_ptr;
}

void check_command_execution(void)
{
    if ((lit_stk_ptr != 0)) {
        {
            fprintf(log_file, "%s%ld%s\n", "ptr=", (long)lit_stk_ptr, ", stack=");
            fprintf(standard_output, "%s%ld%s\n", "ptr=", (long)lit_stk_ptr, ", stack=");
        }
        pop_whole_stack();
        {
            {
                Fputs(log_file, "---the literal stack isn't empty");
                Fputs(standard_output, "---the literal stack isn't empty");
            }
            bst_ex_warn_print();
        }
    }
    if ((cmd_str_ptr != str_ptr)) {
        ;

#ifdef TRACE
        {
            fprintf(log_file, "%s%ld%s%ld\n", "Pointer is ", (long)str_ptr, " but should be ", (long)cmd_str_ptr);
            fprintf(standard_output, "%s%ld%s%ld\n", "Pointer is ", (long)str_ptr, " but should be ",
                    (long)cmd_str_ptr);
        }

#endif                          /* TRACE */
        {
            {
                Fputs(log_file, "Nonempty empty string stack");
                Fputs(standard_output, "Nonempty empty string stack");
            }
            print_confusion();
            longjmp(jmp9998, 1);
        }
    }
}

void add_pool_buf_and_push(void)
{
    {
        while ((pool_ptr + ex_buf_length > pool_size))
            pool_overflow();
    }
    ex_buf_ptr = 0;
    while ((ex_buf_ptr < ex_buf_length)) {

        {
            str_pool[pool_ptr] = ex_buf[ex_buf_ptr];
            pool_ptr = pool_ptr + 1;
        }
        ex_buf_ptr = ex_buf_ptr + 1;
    }
    push_lit_stk(make_string(), 1 /*stk_str */ );
}

void zadd_buf_pool(str_number p_str)
{
    p_ptr1 = str_start[p_str];
    p_ptr2 = str_start[p_str + 1];
    if ((ex_buf_length + (p_ptr2 - p_ptr1) > buf_size))
        buffer_overflow();
    ex_buf_ptr = ex_buf_length;
    while ((p_ptr1 < p_ptr2)) {

        {
            ex_buf[ex_buf_ptr] = str_pool[p_ptr1];
            ex_buf_ptr = ex_buf_ptr + 1;
        }
        p_ptr1 = p_ptr1 + 1;
    }
    ex_buf_length = ex_buf_ptr;
}

void zadd_out_pool(str_number p_str)
{
    buf_pointer break_ptr;
    buf_pointer end_ptr;
    boolean break_pt_found;
    boolean unbreakable_tail;
    p_ptr1 = str_start[p_str];
    p_ptr2 = str_start[p_str + 1];
    while ((out_buf_length + (p_ptr2 - p_ptr1) > buf_size))
        buffer_overflow();
    out_buf_ptr = out_buf_length;
    while ((p_ptr1 < p_ptr2)) {

        out_buf[out_buf_ptr] = str_pool[p_ptr1];
        p_ptr1 = p_ptr1 + 1;
        out_buf_ptr = out_buf_ptr + 1;
    }
    out_buf_length = out_buf_ptr;
    unbreakable_tail = false;
    while (((out_buf_length > max_print_line) && (!unbreakable_tail))) {        /*324: */

        end_ptr = out_buf_length;
        out_buf_ptr = max_print_line;
        break_pt_found = false;
        while (((lex_class[out_buf[out_buf_ptr]] != 1 /*white_space */ ) && (out_buf_ptr >= min_print_line)))
            out_buf_ptr = out_buf_ptr - 1;
        if ((out_buf_ptr == min_print_line - 1)) {      /*325: */
            out_buf_ptr = max_print_line + 1;
            while ((out_buf_ptr < end_ptr))
                if ((lex_class[out_buf[out_buf_ptr]] != 1 /*white_space */ ))
                    out_buf_ptr = out_buf_ptr + 1;
                else
                    goto lab16;
 lab16:                        /*loop1_exit */ if ((out_buf_ptr == end_ptr))
                unbreakable_tail = true;
            else {

                break_pt_found = true;
                while ((out_buf_ptr + 1 < end_ptr))
                    if ((lex_class[out_buf[out_buf_ptr + 1]] == 1 /*white_space */ ))
                        out_buf_ptr = out_buf_ptr + 1;
                    else
                        goto lab17;
 lab17:                        /*loop2_exit */ ;
            }
        } else
            break_pt_found = true;
        if ((break_pt_found)) {
            out_buf_length = out_buf_ptr;
            break_ptr = out_buf_length + 1;
            output_bbl_line();
            out_buf[0] = 32 /*space */ ;
            out_buf[1] = 32 /*space */ ;
            out_buf_ptr = 2;
            tmp_ptr = break_ptr;
            while ((tmp_ptr < end_ptr)) {

                out_buf[out_buf_ptr] = out_buf[tmp_ptr];
                out_buf_ptr = out_buf_ptr + 1;
                tmp_ptr = tmp_ptr + 1;
            }
            out_buf_length = end_ptr - break_ptr + 2;
        }
    }
}

void x_equals(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != pop_typ2)) {
        if (((pop_typ1 != 4 /*stk_empty */ ) && (pop_typ2 != 4 /*stk_empty */ ))) {
            print_stk_lit(pop_lit1, pop_typ1);
            {
                Fputs(log_file, ", ");
                Fputs(standard_output, ", ");
            }
            print_stk_lit(pop_lit2, pop_typ2);
            print_a_newline();
            {
                {
                    Fputs(log_file, "---they aren't the same literal types");
                    Fputs(standard_output, "---they aren't the same literal types");
                }
                bst_ex_warn_print();
            }
        }
        push_lit_stk(0, 0 /*stk_int */ );
    } else if (((pop_typ1 != 0 /*stk_int */ ) && (pop_typ1 != 1 /*stk_str */ ))) {
        if ((pop_typ1 != 4 /*stk_empty */ )) {
            print_stk_lit(pop_lit1, pop_typ1);
            {
                {
                    Fputs(log_file, ", not an integer or a string,");
                    Fputs(standard_output, ", not an integer or a string,");
                }
                bst_ex_warn_print();
            }
        }
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ1 == 0 /*stk_int */ )) {

        if ((pop_lit2 == pop_lit1))
            push_lit_stk(1, 0 /*stk_int */ );
        else
            push_lit_stk(0, 0 /*stk_int */ );
    } else if ((str_eq_str(pop_lit2, pop_lit1)))
        push_lit_stk(1, 0 /*stk_int */ );
    else
        push_lit_stk(0, 0 /*stk_int */ );
}

void x_greater_than(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_lit2 > pop_lit1))
        push_lit_stk(1, 0 /*stk_int */ );
    else
        push_lit_stk(0, 0 /*stk_int */ );
}

void x_less_than(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_lit2 < pop_lit1))
        push_lit_stk(1, 0 /*stk_int */ );
    else
        push_lit_stk(0, 0 /*stk_int */ );
}

void x_plus(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else
        push_lit_stk(pop_lit2 + pop_lit1, 0 /*stk_int */ );
}

void x_minus(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else
        push_lit_stk(pop_lit2 - pop_lit1, 0 /*stk_int */ );
}

void x_concatenate(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ2 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {                    /*352: */

        if ((pop_lit2 >= cmd_str_ptr)) {

            if ((pop_lit1 >= cmd_str_ptr)) {
                str_start[pop_lit1] = str_start[pop_lit1 + 1];
                {
                    str_ptr = str_ptr + 1;
                    pool_ptr = str_start[str_ptr];
                }
                lit_stk_ptr = lit_stk_ptr + 1;
            } else if (((str_start[pop_lit2 + 1] - str_start[pop_lit2]) == 0))
                push_lit_stk(pop_lit1, 1 /*stk_str */ );
            else {

                pool_ptr = str_start[pop_lit2 + 1];
                {
                    while ((pool_ptr + (str_start[pop_lit1 + 1] - str_start[pop_lit1]) > pool_size))
                        pool_overflow();
                }
                sp_ptr = str_start[pop_lit1];
                sp_end = str_start[pop_lit1 + 1];
                while ((sp_ptr < sp_end)) {

                    {
                        str_pool[pool_ptr] = str_pool[sp_ptr];
                        pool_ptr = pool_ptr + 1;
                    }
                    sp_ptr = sp_ptr + 1;
                }
                push_lit_stk(make_string(), 1 /*stk_str */ );
            }
        } else {                /*353: */

            if ((pop_lit1 >= cmd_str_ptr)) {

                if (((str_start[pop_lit2 + 1] - str_start[pop_lit2]) == 0)) {
                    {
                        str_ptr = str_ptr + 1;
                        pool_ptr = str_start[str_ptr];
                    }
                    lit_stack[lit_stk_ptr] = pop_lit1;
                    lit_stk_ptr = lit_stk_ptr + 1;
                } else if (((str_start[pop_lit1 + 1] - str_start[pop_lit1]) == 0))
                    lit_stk_ptr = lit_stk_ptr + 1;
                else {

                    sp_length = (str_start[pop_lit1 + 1] - str_start[pop_lit1]);
                    sp2_length = (str_start[pop_lit2 + 1] - str_start[pop_lit2]);
                    {
                        while ((pool_ptr + sp_length + sp2_length > pool_size))
                            pool_overflow();
                    }
                    sp_ptr = str_start[pop_lit1 + 1];
                    sp_end = str_start[pop_lit1];
                    sp_xptr1 = sp_ptr + sp2_length;
                    while ((sp_ptr > sp_end)) {

                        sp_ptr = sp_ptr - 1;
                        sp_xptr1 = sp_xptr1 - 1;
                        str_pool[sp_xptr1] = str_pool[sp_ptr];
                    }
                    sp_ptr = str_start[pop_lit2];
                    sp_end = str_start[pop_lit2 + 1];
                    while ((sp_ptr < sp_end)) {

                        {
                            str_pool[pool_ptr] = str_pool[sp_ptr];
                            pool_ptr = pool_ptr + 1;
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    pool_ptr = pool_ptr + sp_length;
                    push_lit_stk(make_string(), 1 /*stk_str */ );
                }
            } else {            /*354: */

                if (((str_start[pop_lit1 + 1] - str_start[pop_lit1]) == 0))
                    lit_stk_ptr = lit_stk_ptr + 1;
                else if (((str_start[pop_lit2 + 1] - str_start[pop_lit2]) == 0))
                    push_lit_stk(pop_lit1, 1 /*stk_str */ );
                else {

                    {
                        while ((pool_ptr + (str_start[pop_lit1 + 1] - str_start[pop_lit1]) +
                                (str_start[pop_lit2 + 1] - str_start[pop_lit2]) > pool_size))
                            pool_overflow();
                    }
                    sp_ptr = str_start[pop_lit2];
                    sp_end = str_start[pop_lit2 + 1];
                    while ((sp_ptr < sp_end)) {

                        {
                            str_pool[pool_ptr] = str_pool[sp_ptr];
                            pool_ptr = pool_ptr + 1;
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    sp_ptr = str_start[pop_lit1];
                    sp_end = str_start[pop_lit1 + 1];
                    while ((sp_ptr < sp_end)) {

                        {
                            str_pool[pool_ptr] = str_pool[sp_ptr];
                            pool_ptr = pool_ptr + 1;
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                    push_lit_stk(make_string(), 1 /*stk_str */ );
                }
            }
        }
    }
}

void x_gets(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 2 /*stk_fn */ ))
        print_wrong_stk_lit(pop_lit1, pop_typ1, 2 /*stk_fn */ );
    else if (((!mess_with_entries)
              && ((fn_type[pop_lit1] == 6 /*str_entry_var */ ) || (fn_type[pop_lit1] == 5 /*int_entry_var */ ))))
        bst_cant_mess_with_entries_print();
    else
        switch ((fn_type[pop_lit1])) {
        case 5:
            /*
               356: */ if ((pop_typ2 != 0 /*stk_int */ ))
                print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
            else
                entry_ints[cite_ptr * num_ent_ints + ilk_info[pop_lit1]] = /*:356 */ pop_lit2;
            break;
        case 6:
            {
                if ((pop_typ2 != 1 /*stk_str */ ))
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 1 /*stk_str */ );
                else {

                    str_ent_ptr = cite_ptr * num_ent_strs + ilk_info[pop_lit1];
                    ent_chr_ptr = 0;
                    sp_ptr = str_start[pop_lit2];
                    sp_xptr1 = str_start[pop_lit2 + 1];
                    if ((sp_xptr1 - sp_ptr > ent_str_size)) {
                        {
                            bst_1print_string_size_exceeded();
                            {
                                fprintf(log_file, "%ld%s", (long)ent_str_size, ", the entry");
                                fprintf(standard_output, "%ld%s", (long)ent_str_size, ", the entry");
                            }
                            bst_2print_string_size_exceeded();
                        }
                        sp_xptr1 = sp_ptr + ent_str_size;
                    }
                    while ((sp_ptr < sp_xptr1)) {

                        entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)] = str_pool[sp_ptr];
                        ent_chr_ptr = ent_chr_ptr + 1;
                        sp_ptr = sp_ptr + 1;
                    }
                    entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ent_chr_ptr)] = 127 /*end_of_string */ ;
                }
            }
            break;
        case 7:
            if ((pop_typ2 != 0 /*stk_int */ ))
                print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
            else
                ilk_info[pop_lit1] = /*:359 */ pop_lit2;
            break;
        case 8:
            {
                if ((pop_typ2 != 1 /*stk_str */ ))
                    print_wrong_stk_lit(pop_lit2, pop_typ2, 1 /*stk_str */ );
                else {

                    str_glb_ptr = ilk_info[pop_lit1];
                    if ((pop_lit2 < cmd_str_ptr))
                        glb_str_ptr[str_glb_ptr] = pop_lit2;
                    else {

                        glb_str_ptr[str_glb_ptr] = 0;
                        glob_chr_ptr = 0;
                        sp_ptr = str_start[pop_lit2];
                        sp_end = str_start[pop_lit2 + 1];
                        if ((sp_end - sp_ptr > glob_str_size)) {
                            {
                                bst_1print_string_size_exceeded();
                                {
                                    fprintf(log_file, "%ld%s", (long)glob_str_size, ", the global");
                                    fprintf(standard_output, "%ld%s", (long)glob_str_size, ", the global");
                                }
                                bst_2print_string_size_exceeded();
                            }
                            sp_end = sp_ptr + glob_str_size;
                        }
                        while ((sp_ptr < sp_end)) {

                            global_strs[(str_glb_ptr) * (glob_str_size + 1) + (glob_chr_ptr)] = str_pool[sp_ptr];
                            glob_chr_ptr = glob_chr_ptr + 1;
                            sp_ptr = sp_ptr + 1;
                        }
                        glb_str_end[str_glb_ptr] = glob_chr_ptr;
                    }
                }
            }
            break;
        default:
            {
                {
                    Fputs(log_file, "You can't assign to type ");
                    Fputs(standard_output, "You can't assign to type ");
                }
                print_fn_class(pop_lit1);
                {
                    {
                        Fputs(log_file, ", a nonvariable function class");
                        Fputs(standard_output, ", a nonvariable function class");
                    }
                    bst_ex_warn_print();
                }
            }
            break;
        }
}

void x_add_period(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if (((str_start[pop_lit1 + 1] - str_start[pop_lit1]) == 0))
        push_lit_stk(s_null, 1 /*stk_str */ );
    else {                      /*362: */

        sp_ptr = str_start[pop_lit1 + 1];
        sp_end = str_start[pop_lit1];
        while ((sp_ptr > sp_end)) {

            sp_ptr = sp_ptr - 1;
            if ((str_pool[sp_ptr] != 125 /*right_brace */ ))
                goto lab15;
        }
 lab15:                        /*loop_exit */ switch ((str_pool[sp_ptr])) {
        case 46:
        case 63:
        case 33:
            {
                if ((lit_stack[lit_stk_ptr] >= cmd_str_ptr)) {
                    str_ptr = str_ptr + 1;
                    pool_ptr = str_start[str_ptr];
                }
                lit_stk_ptr = lit_stk_ptr + 1;
            }
            break;
        default:
            {
                if ((pop_lit1 < cmd_str_ptr)) {
                    {
                        while ((pool_ptr + (str_start[pop_lit1 + 1] - str_start[pop_lit1]) + 1 > pool_size))
                            pool_overflow();
                    }
                    sp_ptr = str_start[pop_lit1];
                    sp_end = str_start[pop_lit1 + 1];
                    while ((sp_ptr < sp_end)) {

                        {
                            str_pool[pool_ptr] = str_pool[sp_ptr];
                            pool_ptr = pool_ptr + 1;
                        }
                        sp_ptr = sp_ptr + 1;
                    }
                } else {

                    pool_ptr = str_start[pop_lit1 + 1];
                    {
                        while ((pool_ptr + 1 > pool_size))
                            pool_overflow();
                    }
                }
                {
                    str_pool[pool_ptr] = 46 /*period */ ;
                    pool_ptr = pool_ptr + 1;
                }
                push_lit_stk(make_string(), 1 /*stk_str */ );
            }
            break;
        }
    }
}

void x_change_case(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ2 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        {
            switch ((str_pool[str_start[pop_lit1]])) {
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
            if ((((str_start[pop_lit1 + 1] - str_start[pop_lit1]) != 1) || (conversion_type == 3 /*bad_conversion */ ))) {
                conversion_type = 3 /*bad_conversion */ ;
                print_a_pool_str(pop_lit1);
                {
                    {
                        Fputs(log_file, " is an illegal case-conversion string");
                        Fputs(standard_output, " is an illegal case-conversion string");
                    }
                    bst_ex_warn_print();
                }
            }
        }
        ex_buf_length = 0;
        add_buf_pool(pop_lit2);
        {
            brace_level = 0;
            ex_buf_ptr = 0;
            while ((ex_buf_ptr < ex_buf_length)) {

                if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ )) {
                    brace_level = brace_level + 1;
                    if ((brace_level != 1))
                        goto lab21;
                    if ((ex_buf_ptr + 4 > ex_buf_length))
                        goto lab21;
                    else if ((ex_buf[ex_buf_ptr + 1] != 92 /*backslash */ ))
                        goto lab21;
                    if ((conversion_type == 0 /*title_lowers */ )) {

                        if ((ex_buf_ptr == 0))
                            goto lab21;
                        else if (((prev_colon) && (lex_class[ex_buf[ex_buf_ptr - 1]] == 1 /*white_space */ )))
                            goto lab21;
                    }
                    {
                        ex_buf_ptr = ex_buf_ptr + 1;
                        while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0))) {

                            ex_buf_ptr = ex_buf_ptr + 1;
                            ex_buf_xptr = ex_buf_ptr;
                            while (((ex_buf_ptr < ex_buf_length) && (lex_class[ex_buf[ex_buf_ptr]] == 2 /*alpha */ )))
                                ex_buf_ptr = ex_buf_ptr + 1;
                            control_seq_loc =
                                str_lookup(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr, 14 /*control_seq_ilk */ ,
                                           false);
                            if ((hash_found)) { /*373: */
                                switch ((conversion_type)) {
                                case 0:
                                case 1:
                                    switch ((ilk_info[control_seq_loc])) {
                                    case 11:
                                    case 9:
                                    case 3:
                                    case 5:
                                    case 7:
                                        lower_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
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
                                        upper_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                        break;
                                    case 0:
                                    case 1:
                                    case 12:
                                        {
                                            upper_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                            while ((ex_buf_xptr < ex_buf_ptr)) {

                                                ex_buf[ex_buf_xptr - 1] = ex_buf[ex_buf_xptr];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            ex_buf_xptr = ex_buf_xptr - 1;
                                            while (((ex_buf_ptr < ex_buf_length)
                                                    && (lex_class[ex_buf[ex_buf_ptr]] == 1 /*white_space */ )))
                                                ex_buf_ptr = ex_buf_ptr + 1;
                                            tmp_ptr = ex_buf_ptr;
                                            while ((tmp_ptr < ex_buf_length)) {

                                                ex_buf[tmp_ptr - (ex_buf_ptr - ex_buf_xptr)] = ex_buf[tmp_ptr];
                                                tmp_ptr = tmp_ptr + 1;
                                            }
                                            ex_buf_length = tmp_ptr - (ex_buf_ptr - ex_buf_xptr);
                                            ex_buf_ptr = ex_buf_xptr;
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
                                    break;
                                }
                            }
                            ex_buf_xptr = ex_buf_ptr;
                            while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0)
                                    && (ex_buf[ex_buf_ptr] != 92 /*backslash */ ))) {

                                if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ ))
                                    brace_level = brace_level - 1;
                                else if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ ))
                                    brace_level = brace_level + 1;
                                ex_buf_ptr = ex_buf_ptr + 1;
                            }
                            {
                                switch ((conversion_type)) {
                                case 0:
                                case 1:
                                    lower_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                    break;
                                case 2:
                                    upper_case(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr);
                                    break;
                                case 3:
                                    ;
                                    break;
                                default:
                                    case_conversion_confusion();
                                    break;
                                }
                            }
                        }
                        ex_buf_ptr = ex_buf_ptr - 1;
                    }
 lab21:                        /*ok_pascal_i_give_up */ prev_colon = false;
                } else if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ )) {
                    decr_brace_level(pop_lit2);
                    prev_colon = false;
                } else if ((brace_level == 0)) {        /*377: */
                    switch ((conversion_type)) {
                    case 0:
                        {
                            if ((ex_buf_ptr == 0)) ;
                            else if (((prev_colon) && (lex_class[ex_buf[ex_buf_ptr - 1]] == 1 /*white_space */ ))) ;
                            else
                                lower_case(ex_buf, ex_buf_ptr, 1);
                            if ((ex_buf[ex_buf_ptr] == 58 /*colon */ ))
                                prev_colon = true;
                            else if ((lex_class[ex_buf[ex_buf_ptr]] != 1 /*white_space */ ))
                                prev_colon = false;
                        }
                        break;
                    case 1:
                        lower_case(ex_buf, ex_buf_ptr, 1);
                        break;
                    case 2:
                        upper_case(ex_buf, ex_buf_ptr, 1);
                        break;
                    case 3:
                        ;
                        break;
                    default:
                        case_conversion_confusion();
                        break;
                    }
                }
                ex_buf_ptr = ex_buf_ptr + 1;
            }
            check_brace_level(pop_lit2);
        }
        add_pool_buf_and_push();
    }
}

void x_chr_to_int(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else if (((str_start[pop_lit1 + 1] - str_start[pop_lit1]) != 1)) {
        {
            putc('"', log_file);
            putc('"', standard_output);
        }
        print_a_pool_str(pop_lit1);
        {
            {
                Fputs(log_file, "\" isn't a single character");
                Fputs(standard_output, "\" isn't a single character");
            }
            bst_ex_warn_print();
        }
        push_lit_stk(0, 0 /*stk_int */ );
    } else
        push_lit_stk(str_pool[str_start[pop_lit1]], 0 /*stk_int */ );
}

void x_cite(void)
{
    if ((!mess_with_entries))
        bst_cant_mess_with_entries_print();
    else
        push_lit_stk(cite_list[cite_ptr], 1 /*stk_str */ );
}

void x_duplicate(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        push_lit_stk(pop_lit1, pop_typ1);
        push_lit_stk(pop_lit1, pop_typ1);
    } else {

        {
            if ((lit_stack[lit_stk_ptr] >= cmd_str_ptr)) {
                str_ptr = str_ptr + 1;
                pool_ptr = str_start[str_ptr];
            }
            lit_stk_ptr = lit_stk_ptr + 1;
        }
        if ((pop_lit1 < cmd_str_ptr))
            push_lit_stk(pop_lit1, pop_typ1);
        else {

            {
                while ((pool_ptr + (str_start[pop_lit1 + 1] - str_start[pop_lit1]) > pool_size))
                    pool_overflow();
            }
            sp_ptr = str_start[pop_lit1];
            sp_end = str_start[pop_lit1 + 1];
            while ((sp_ptr < sp_end)) {

                {
                    str_pool[pool_ptr] = str_pool[sp_ptr];
                    pool_ptr = pool_ptr + 1;
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(make_string(), 1 /*stk_str */ );
        }
    }
}

void x_empty(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    switch ((pop_typ1)) {
    case 1:
        {
            sp_ptr = str_start[pop_lit1];
            sp_end = str_start[pop_lit1 + 1];
            while ((sp_ptr < sp_end)) {

                if ((lex_class[str_pool[sp_ptr]] != 1 /*white_space */ )) {
                    push_lit_stk(0, 0 /*stk_int */ );
                    goto lab10;
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(1, 0 /*stk_int */ );
        }
        break;
    case 3:
        push_lit_stk(1, 0 /*stk_int */ );
        break;
    case 4:
        push_lit_stk(0, 0 /*stk_int */ );
        break;
    default:
        {
            print_stk_lit(pop_lit1, pop_typ1);
            {
                {
                    Fputs(log_file, ", not a string or missing field,");
                    Fputs(standard_output, ", not a string or missing field,");
                }
                bst_ex_warn_print();
            }
            push_lit_stk(0, 0 /*stk_int */ );
        }
        break;
    }
 lab10:                        /*exit */ ;
}

void x_format_name(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    pop_lit_stk(pop_lit3, pop_typ3);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ3 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit3, pop_typ3, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        ex_buf_length = 0;
        add_buf_pool(pop_lit3);
        {
            ex_buf_ptr = 0;
            num_names = 0;
            while (((num_names < pop_lit2) && (ex_buf_ptr < ex_buf_length))) {

                num_names = num_names + 1;
                ex_buf_xptr = ex_buf_ptr;
                name_scan_for_and(pop_lit3);
            }
            if ((ex_buf_ptr < ex_buf_length))
                ex_buf_ptr = ex_buf_ptr - 4;
            if ((num_names < pop_lit2)) {
                if ((pop_lit2 == 1)) {
                    Fputs(log_file, "There is no name in \"");
                    Fputs(standard_output, "There is no name in \"");
                } else {

                    fprintf(log_file, "%s%ld%s", "There aren't ", (long)pop_lit2, " names in \"");
                    fprintf(standard_output, "%s%ld%s", "There aren't ", (long)pop_lit2, " names in \"");
                }
                print_a_pool_str(pop_lit3);
                {
                    {
                        putc('"', log_file);
                        putc('"', standard_output);
                    }
                    bst_ex_warn_print();
                }
            }
        }
        {
            {
                while ((ex_buf_ptr > ex_buf_xptr))
                    switch ((lex_class[ex_buf[ex_buf_ptr - 1]])) {
                    case 1:
                    case 4:
                        ex_buf_ptr = ex_buf_ptr - 1;
                        break;
                    default:
                        if ((ex_buf[ex_buf_ptr - 1] == 44 /*comma */ )) {
                            {
                                fprintf(log_file, "%s%ld%s", "Name ", (long)pop_lit2, " in \"");
                                fprintf(standard_output, "%s%ld%s", "Name ", (long)pop_lit2, " in \"");
                            }
                            print_a_pool_str(pop_lit3);
                            {
                                Fputs(log_file, "\" has a comma at the end");
                                Fputs(standard_output, "\" has a comma at the end");
                            }
                            bst_ex_warn_print();
                            ex_buf_ptr = ex_buf_ptr - 1;
                        } else
                            goto lab16;
                        break;
                    }
 lab16:                        /*loop1_exit */ ;
            }
            name_bf_ptr = 0;
            num_commas = 0;
            num_tokens = 0;
            token_starting = true;
            while ((ex_buf_xptr < ex_buf_ptr))
                switch ((ex_buf[ex_buf_xptr])) {
                case 44:
                    {
                        if ((num_commas == 2)) {
                            {
                                fprintf(log_file, "%s%ld%s", "Too many commas in name ", (long)pop_lit2, " of \"");
                                fprintf(standard_output, "%s%ld%s", "Too many commas in name ", (long)pop_lit2,
                                        " of \"");
                            }
                            print_a_pool_str(pop_lit3);
                            {
                                putc('"', log_file);
                                putc('"', standard_output);
                            }
                            bst_ex_warn_print();
                        } else {

                            num_commas = num_commas + 1;
                            if ((num_commas == 1))
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
                        if ((token_starting)) {
                            name_tok[num_tokens] = name_bf_ptr;
                            num_tokens = num_tokens + 1;
                        }
                        sv_buffer[name_bf_ptr] = ex_buf[ex_buf_xptr];
                        name_bf_ptr = name_bf_ptr + 1;
                        ex_buf_xptr = ex_buf_xptr + 1;
                        while (((brace_level > 0) && (ex_buf_xptr < ex_buf_ptr))) {

                            if ((ex_buf[ex_buf_xptr] == 125 /*right_brace */ ))
                                brace_level = brace_level - 1;
                            else if ((ex_buf[ex_buf_xptr] == 123 /*left_brace */ ))
                                brace_level = brace_level + 1;
                            sv_buffer[name_bf_ptr] = ex_buf[ex_buf_xptr];
                            name_bf_ptr = name_bf_ptr + 1;
                            ex_buf_xptr = ex_buf_xptr + 1;
                        }
                        token_starting = false;
                    }
                    break;
                case 125:
                    {
                        if ((token_starting)) {
                            name_tok[num_tokens] = name_bf_ptr;
                            num_tokens = num_tokens + 1;
                        }
                        {
                            fprintf(log_file, "%s%ld%s", "Name ", (long)pop_lit2, " of \"");
                            fprintf(standard_output, "%s%ld%s", "Name ", (long)pop_lit2, " of \"");
                        }
                        print_a_pool_str(pop_lit3);
                        {
                            {
                                Fputs(log_file, "\" isn't brace balanced");
                                Fputs(standard_output, "\" isn't brace balanced");
                            }
                            bst_ex_warn_print();
                        }
                        ex_buf_xptr = ex_buf_xptr + 1;
                        token_starting = false;
                    }
                    break;
                default:
                    switch ((lex_class[ex_buf[ex_buf_xptr]])) {
                    case 1:
                        {
                            if ((!token_starting))
                                name_sep_char[num_tokens] = 32 /*space */ ;
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = true;
                        }
                        break;
                    case 4:
                        {
                            if ((!token_starting))
                                name_sep_char[num_tokens] = ex_buf[ex_buf_xptr];
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = true;
                        }
                        break;
                    default:
                        {
                            if ((token_starting)) {
                                name_tok[num_tokens] = name_bf_ptr;
                                num_tokens = num_tokens + 1;
                            }
                            sv_buffer[name_bf_ptr] = ex_buf[ex_buf_xptr];
                            name_bf_ptr = name_bf_ptr + 1;
                            ex_buf_xptr = ex_buf_xptr + 1;
                            token_starting = false;
                        }
                        break;
                    }
                    break;
                }
            name_tok[num_tokens] = name_bf_ptr;
        }
        {
            if ((num_commas == 0)) {
                first_start = 0;
                last_end = num_tokens;
                jr_end = last_end;
                {
                    von_start = 0;
                    while ((von_start < last_end - 1)) {

                        name_bf_ptr = name_tok[von_start];
                        name_bf_xptr = name_tok[von_start + 1];
                        if ((von_token_found())) {
                            von_name_ends_and_last_name_starts_stuff();
                            goto lab52;
                        }
                        von_start = von_start + 1;
                    }
                    while ((von_start > 0)) {

                        if (((lex_class[name_sep_char[von_start]] != 4 /*sep_char */ )
                             || (name_sep_char[von_start] == 126 /*tie */ )))
                            goto lab17;
                        von_start = von_start - 1;
                    }
 lab17:                        /*loop2_exit */ von_end = von_start;
 lab52:                        /*von_found */ first_end = von_start;
                }
            } else if ((num_commas == 1)) {
                von_start = 0;
                last_end = comma1;
                jr_end = last_end;
                first_start = jr_end;
                first_end = num_tokens;
                von_name_ends_and_last_name_starts_stuff();
            } else if ((num_commas == 2)) {
                von_start = 0;
                last_end = comma1;
                jr_end = comma2;
                first_start = jr_end;
                first_end = num_tokens;
                von_name_ends_and_last_name_starts_stuff();
            } else {

                {
                    Fputs(log_file, "Illegal number of comma,s");
                    Fputs(standard_output, "Illegal number of comma,s");
                }
                print_confusion();
                longjmp(jmp9998, 1);
            }
        }
        ex_buf_length = 0;
        add_buf_pool(pop_lit1);
        figure_out_the_formatted_name();
        add_pool_buf_and_push();
    }
}

void x_int_to_chr(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if (((pop_lit1 < 0) || (pop_lit1 > 127))) {
        {
            {
                fprintf(log_file, "%ld%s", (long)pop_lit1, " isn't valid ASCII");
                fprintf(standard_output, "%ld%s", (long)pop_lit1, " isn't valid ASCII");
            }
            bst_ex_warn_print();
        }
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        {
            while ((pool_ptr + 1 > pool_size))
                pool_overflow();
        }
        {
            str_pool[pool_ptr] = pop_lit1;
            pool_ptr = pool_ptr + 1;
        }
        push_lit_stk(make_string(), 1 /*stk_str */ );
    }
}

void x_int_to_str(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        int_to_ASCII(pop_lit1, ex_buf, 0, ex_buf_length);
        add_pool_buf_and_push();
    }
}

void x_missing(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((!mess_with_entries))
        bst_cant_mess_with_entries_print();
    else if (((pop_typ1 != 1 /*stk_str */ ) && (pop_typ1 != 3 /*stk_field_missing */ ))) {
        if ((pop_typ1 != 4 /*stk_empty */ )) {
            print_stk_lit(pop_lit1, pop_typ1);
            {
                {
                    Fputs(log_file, ", not a string or missing field,");
                    Fputs(standard_output, ", not a string or missing field,");
                }
                bst_ex_warn_print();
            }
        }
        push_lit_stk(0, 0 /*stk_int */ );
    } else if ((pop_typ1 == 3 /*stk_field_missing */ ))
        push_lit_stk(1, 0 /*stk_int */ );
    else
        push_lit_stk(0, 0 /*stk_int */ );
}

void x_num_names(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else {

        ex_buf_length = 0;
        add_buf_pool(pop_lit1);
        {
            ex_buf_ptr = 0;
            num_names = 0;
            while ((ex_buf_ptr < ex_buf_length)) {

                name_scan_for_and(pop_lit1);
                num_names = num_names + 1;
            }
        }
        push_lit_stk(num_names, 0 /*stk_int */ );
    }
}

void x_preamble(void)
{
    ex_buf_length = 0;
    preamble_ptr = 0;
    while ((preamble_ptr < num_preamble_strings)) {

        add_buf_pool(s_preamble[preamble_ptr]);
        preamble_ptr = preamble_ptr + 1;
    }
    add_pool_buf_and_push();
}

void x_purify(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        ex_buf_length = 0;
        add_buf_pool(pop_lit1);
        {
            brace_level = 0;
            ex_buf_xptr = 0;
            ex_buf_ptr = 0;
            while ((ex_buf_ptr < ex_buf_length)) {

                switch ((lex_class[ex_buf[ex_buf_ptr]])) {
                case 1:
                case 4:
                    {
                        ex_buf[ex_buf_xptr] = 32 /*space */ ;
                        ex_buf_xptr = ex_buf_xptr + 1;
                    }
                    break;
                case 2:
                case 3:
                    {
                        ex_buf[ex_buf_xptr] = ex_buf[ex_buf_ptr];
                        ex_buf_xptr = ex_buf_xptr + 1;
                    }
                    break;
                default:
                    if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ )) {
                        brace_level = brace_level + 1;
                        if (((brace_level == 1) && (ex_buf_ptr + 1 < ex_buf_length))) {

                            if ((ex_buf[ex_buf_ptr + 1] == 92 /*backslash */ )) {       /*433: */
                                ex_buf_ptr = ex_buf_ptr + 1;
                                while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0))) {

                                    ex_buf_ptr = ex_buf_ptr + 1;
                                    ex_buf_yptr = ex_buf_ptr;
                                    while (((ex_buf_ptr < ex_buf_length)
                                            && (lex_class[ex_buf[ex_buf_ptr]] == 2 /*alpha */ )))
                                        ex_buf_ptr = ex_buf_ptr + 1;
                                    control_seq_loc =
                                        str_lookup(ex_buf, ex_buf_yptr, ex_buf_ptr - ex_buf_yptr,
                                                   14 /*control_seq_ilk */ , false);
                                    if ((hash_found)) { /*434: */
                                        ex_buf[ex_buf_xptr] = ex_buf[ex_buf_yptr];
                                        ex_buf_xptr = ex_buf_xptr + 1;
                                        switch ((ilk_info[control_seq_loc])) {
                                        case 2:
                                        case 3:
                                        case 4:
                                        case 5:
                                        case 12:
                                            {
                                                ex_buf[ex_buf_xptr] = ex_buf[ex_buf_yptr + 1];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            break;
                                        default:
                                            ;
                                            break;
                                        }
                                    }
                                    while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0)
                                            && (ex_buf[ex_buf_ptr] != 92 /*backslash */ ))) {

                                        switch ((lex_class[ex_buf[ex_buf_ptr]])) {
                                        case 2:
                                        case 3:
                                            {
                                                ex_buf[ex_buf_xptr] = ex_buf[ex_buf_ptr];
                                                ex_buf_xptr = ex_buf_xptr + 1;
                                            }
                                            break;
                                        default:
                                            if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ ))
                                                brace_level = brace_level - 1;
                                            else if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ ))
                                                brace_level = brace_level + 1;
                                            break;
                                        }
                                        ex_buf_ptr = ex_buf_ptr + 1;
                                    }
                                }
                                ex_buf_ptr = ex_buf_ptr - 1;
                            }
                        }
                    } else if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ )) {

                        if ((brace_level > 0))
                            brace_level = brace_level - 1;
                    }
                    break;
                }
                ex_buf_ptr = ex_buf_ptr + 1;
            }
            ex_buf_length = ex_buf_xptr;
        }
        add_pool_buf_and_push();
    }
}

void x_quote(void)
{
    {
        while ((pool_ptr + 1 > pool_size))
            pool_overflow();
    }
    {
        str_pool[pool_ptr] = 34 /*double_quote */ ;
        pool_ptr = pool_ptr + 1;
    }
    push_lit_stk(make_string(), 1 /*stk_str */ );
}

void x_substring(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    pop_lit_stk(pop_lit3, pop_typ3);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ2 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ3 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit3, pop_typ3, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        sp_length = (str_start[pop_lit3 + 1] - str_start[pop_lit3]);
        if ((pop_lit1 >= sp_length)) {

            if (((pop_lit2 == 1) || (pop_lit2 == -1))) {
                {
                    if ((lit_stack[lit_stk_ptr] >= cmd_str_ptr)) {
                        str_ptr = str_ptr + 1;
                        pool_ptr = str_start[str_ptr];
                    }
                    lit_stk_ptr = lit_stk_ptr + 1;
                }
                goto lab10;
            }
        }
        if (((pop_lit1 <= 0) || (pop_lit2 == 0) || (pop_lit2 > sp_length) || (pop_lit2 < -(integer) sp_length))) {
            push_lit_stk(s_null, 1 /*stk_str */ );
            goto lab10;
        } else {                /*439: */

            if ((pop_lit2 > 0)) {
                if ((pop_lit1 > sp_length - (pop_lit2 - 1)))
                    pop_lit1 = sp_length - (pop_lit2 - 1);
                sp_ptr = str_start[pop_lit3] + (pop_lit2 - 1);
                sp_end = sp_ptr + pop_lit1;
                if ((pop_lit2 == 1)) {

                    if ((pop_lit3 >= cmd_str_ptr)) {
                        str_start[pop_lit3 + 1] = sp_end;
                        {
                            str_ptr = str_ptr + 1;
                            pool_ptr = str_start[str_ptr];
                        }
                        lit_stk_ptr = lit_stk_ptr + 1;
                        goto lab10;
                    }
                }
            } else {

                pop_lit2 = -(integer) pop_lit2;
                if ((pop_lit1 > sp_length - (pop_lit2 - 1)))
                    pop_lit1 = sp_length - (pop_lit2 - 1);
                sp_end = str_start[pop_lit3 + 1] - (pop_lit2 - 1);
                sp_ptr = sp_end - pop_lit1;
            }
            {
                while ((pool_ptr + sp_end - sp_ptr > pool_size))
                    pool_overflow();
            }
            while ((sp_ptr < sp_end)) {

                {
                    str_pool[pool_ptr] = str_pool[sp_ptr];
                    pool_ptr = pool_ptr + 1;
                }
                sp_ptr = sp_ptr + 1;
            }
            push_lit_stk(make_string(), 1 /*stk_str */ );
        }
    }
 lab10:                        /*exit */ ;
}

void x_swap(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if (((pop_typ1 != 1 /*stk_str */ ) || (pop_lit1 < cmd_str_ptr))) {
        push_lit_stk(pop_lit1, pop_typ1);
        if (((pop_typ2 == 1 /*stk_str */ ) && (pop_lit2 >= cmd_str_ptr))) {
            str_ptr = str_ptr + 1;
            pool_ptr = str_start[str_ptr];
        }
        push_lit_stk(pop_lit2, pop_typ2);
    } else if (((pop_typ2 != 1 /*stk_str */ ) || (pop_lit2 < cmd_str_ptr))) {
        {
            str_ptr = str_ptr + 1;
            pool_ptr = str_start[str_ptr];
        }
        push_lit_stk(pop_lit1, 1 /*stk_str */ );
        push_lit_stk(pop_lit2, pop_typ2);
    } else {                    /*441: */

        ex_buf_length = 0;
        add_buf_pool(pop_lit2);
        sp_ptr = str_start[pop_lit1];
        sp_end = str_start[pop_lit1 + 1];
        while ((sp_ptr < sp_end)) {

            {
                str_pool[pool_ptr] = str_pool[sp_ptr];
                pool_ptr = pool_ptr + 1;
            }
            sp_ptr = sp_ptr + 1;
        }
        push_lit_stk(make_string(), 1 /*stk_str */ );
        add_pool_buf_and_push();
    }
}

void x_text_length(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else {

        num_text_chars = 0;
        {
            sp_ptr = str_start[pop_lit1];
            sp_end = str_start[pop_lit1 + 1];
            sp_brace_level = 0;
            while ((sp_ptr < sp_end)) {

                sp_ptr = sp_ptr + 1;
                if ((str_pool[sp_ptr - 1] == 123 /*left_brace */ )) {
                    sp_brace_level = sp_brace_level + 1;
                    if (((sp_brace_level == 1) && (sp_ptr < sp_end))) {

                        if ((str_pool[sp_ptr] == 92 /*backslash */ )) {
                            sp_ptr = sp_ptr + 1;
                            while (((sp_ptr < sp_end) && (sp_brace_level > 0))) {

                                if ((str_pool[sp_ptr] == 125 /*right_brace */ ))
                                    sp_brace_level = sp_brace_level - 1;
                                else if ((str_pool[sp_ptr] == 123 /*left_brace */ ))
                                    sp_brace_level = sp_brace_level + 1;
                                sp_ptr = sp_ptr + 1;
                            }
                            num_text_chars = num_text_chars + 1;
                        }
                    }
                } else if ((str_pool[sp_ptr - 1] == 125 /*right_brace */ )) {
                    if ((sp_brace_level > 0))
                        sp_brace_level = sp_brace_level - 1;
                } else
                    num_text_chars = num_text_chars + 1;
            }
        }
        push_lit_stk(num_text_chars, 0 /*stk_int */ );
    }
}

void x_text_prefix(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    pop_lit_stk(pop_lit2, pop_typ2);
    if ((pop_typ1 != 0 /*stk_int */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_typ2 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit2, pop_typ2, 1 /*stk_str */ );
        push_lit_stk(s_null, 1 /*stk_str */ );
    } else if ((pop_lit1 <= 0)) {
        push_lit_stk(s_null, 1 /*stk_str */ );
        goto lab10;
    } else {                    /*445: */

        sp_ptr = str_start[pop_lit2];
        sp_end = str_start[pop_lit2 + 1];
        {
            num_text_chars = 0;
            sp_brace_level = 0;
            sp_xptr1 = sp_ptr;
            while (((sp_xptr1 < sp_end) && (num_text_chars < pop_lit1))) {

                sp_xptr1 = sp_xptr1 + 1;
                if ((str_pool[sp_xptr1 - 1] == 123 /*left_brace */ )) {
                    sp_brace_level = sp_brace_level + 1;
                    if (((sp_brace_level == 1) && (sp_xptr1 < sp_end))) {

                        if ((str_pool[sp_xptr1] == 92 /*backslash */ )) {
                            sp_xptr1 = sp_xptr1 + 1;
                            while (((sp_xptr1 < sp_end) && (sp_brace_level > 0))) {

                                if ((str_pool[sp_xptr1] == 125 /*right_brace */ ))
                                    sp_brace_level = sp_brace_level - 1;
                                else if ((str_pool[sp_xptr1] == 123 /*left_brace */ ))
                                    sp_brace_level = sp_brace_level + 1;
                                sp_xptr1 = sp_xptr1 + 1;
                            }
                            num_text_chars = num_text_chars + 1;
                        }
                    }
                } else if ((str_pool[sp_xptr1 - 1] == 125 /*right_brace */ )) {
                    if ((sp_brace_level > 0))
                        sp_brace_level = sp_brace_level - 1;
                } else
                    num_text_chars = num_text_chars + 1;
            }
            sp_end = sp_xptr1;
        }
        {
            while ((pool_ptr + sp_brace_level + sp_end - sp_ptr > pool_size))
                pool_overflow();
        }
        if ((pop_lit2 >= cmd_str_ptr))
            pool_ptr = sp_end;
        else
            while ((sp_ptr < sp_end)) {

                {
                    str_pool[pool_ptr] = str_pool[sp_ptr];
                    pool_ptr = pool_ptr + 1;
                }
                sp_ptr = sp_ptr + 1;
            }
        while ((sp_brace_level > 0)) {

            {
                str_pool[pool_ptr] = 125 /*right_brace */ ;
                pool_ptr = pool_ptr + 1;
            }
            sp_brace_level = sp_brace_level - 1;
        }
        push_lit_stk(make_string(), 1 /*stk_str */ );
    }
 lab10:                        /*exit */ ;
}

void x_type(void)
{
    if ((!mess_with_entries))
        bst_cant_mess_with_entries_print();
    else if (((type_list[cite_ptr] == undefined) || (type_list[cite_ptr] == 0 /*empty */ )))
        push_lit_stk(s_null, 1 /*stk_str */ );
    else
        push_lit_stk(hash_text[type_list[cite_ptr]], 1 /*stk_str */ );
}

void x_warning(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ ))
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
    else {

        {
            Fputs(log_file, "Warning--");
            Fputs(standard_output, "Warning--");
        }
        print_lit(pop_lit1, pop_typ1);
        mark_warning();
    }
}

void x_width(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ )) {
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
        push_lit_stk(0, 0 /*stk_int */ );
    } else {

        ex_buf_length = 0;
        add_buf_pool(pop_lit1);
        string_width = 0;
        {
            brace_level = 0;
            ex_buf_ptr = 0;
            while ((ex_buf_ptr < ex_buf_length)) {

                if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ )) {
                    brace_level = brace_level + 1;
                    if (((brace_level == 1) && (ex_buf_ptr + 1 < ex_buf_length))) {

                        if ((ex_buf[ex_buf_ptr + 1] == 92 /*backslash */ )) {   /*453: */
                            ex_buf_ptr = ex_buf_ptr + 1;
                            while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0))) {

                                ex_buf_ptr = ex_buf_ptr + 1;
                                ex_buf_xptr = ex_buf_ptr;
                                while (((ex_buf_ptr < ex_buf_length)
                                        && (lex_class[ex_buf[ex_buf_ptr]] == 2 /*alpha */ )))
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                if (((ex_buf_ptr < ex_buf_length) && (ex_buf_ptr == ex_buf_xptr)))
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                else {

                                    control_seq_loc =
                                        str_lookup(ex_buf, ex_buf_xptr, ex_buf_ptr - ex_buf_xptr,
                                                   14 /*control_seq_ilk */ , false);
                                    if ((hash_found)) { /*454: */
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
                                            string_width = string_width + char_width[ex_buf[ex_buf_xptr]];
                                            break;
                                        }
                                    }
                                }
                                while (((ex_buf_ptr < ex_buf_length)
                                        && (lex_class[ex_buf[ex_buf_ptr]] == 1 /*white_space */ )))
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                while (((ex_buf_ptr < ex_buf_length) && (brace_level > 0)
                                        && (ex_buf[ex_buf_ptr] != 92 /*backslash */ ))) {

                                    if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ ))
                                        brace_level = brace_level - 1;
                                    else if ((ex_buf[ex_buf_ptr] == 123 /*left_brace */ ))
                                        brace_level = brace_level + 1;
                                    else
                                        string_width = string_width + char_width[ex_buf[ex_buf_ptr]];
                                    ex_buf_ptr = ex_buf_ptr + 1;
                                }
                            }
                            ex_buf_ptr = ex_buf_ptr - 1;
                        } else
                            string_width = string_width + char_width[123 /*left_brace */ ];
                    } else
                        string_width = string_width + char_width[123 /*left_brace */ ];
                } else if ((ex_buf[ex_buf_ptr] == 125 /*right_brace */ )) {
                    decr_brace_level(pop_lit1);
                    string_width = string_width + char_width[125 /*right_brace */ ];
                } else
                    string_width = string_width + char_width[ex_buf[ex_buf_ptr]];
                ex_buf_ptr = ex_buf_ptr + 1;
            }
            check_brace_level(pop_lit1);
        }
        push_lit_stk(string_width, 0 /*stk_int */ );
    }
}

void x_write(void)
{
    pop_lit_stk(pop_lit1, pop_typ1);
    if ((pop_typ1 != 1 /*stk_str */ ))
        print_wrong_stk_lit(pop_lit1, pop_typ1, 1 /*stk_str */ );
    else
        add_out_pool(pop_lit1);
}

void zexecute_fn(hash_loc ex_fn_loc)
{
    integer r_pop_lt1, r_pop_lt2;
    stk_type r_pop_tp1, r_pop_tp2;
    wiz_fn_loc wiz_ptr;
    ;

#ifdef TRACE
    {
        Fputs(log_file, "execute_fn `");
    }
    {
        out_pool_str(log_file, hash_text[ex_fn_loc]);
    }
    {
        {
            putc('\'', log_file);
            putc('\n', log_file);
        }
    }

#endif                          /* TRACE */
    switch ((fn_type[ex_fn_loc])) {
    case 0:
        {
            ;

#ifndef NO_BIBTEX_STAT
            execution_count[ilk_info[ex_fn_loc]] = execution_count[ilk_info[ex_fn_loc]] + 1;

#endif                          /* not NO_BIBTEX_STAT */
            switch ((ilk_info[ex_fn_loc])) {
            case 0:
                x_equals();
                break;
            case 1:
                x_greater_than();
                break;
            case 2:
                x_less_than();
                break;
            case 3:
                x_plus();
                break;
            case 4:
                x_minus();
                break;
            case 5:
                x_concatenate();
                break;
            case 6:
                x_gets();
                break;
            case 7:
                x_add_period();
                break;
            case 8:
                {
                    if ((!mess_with_entries))
                        bst_cant_mess_with_entries_print();
                    else if ((type_list[cite_ptr] == undefined))
                        execute_fn(b_default);
                    else if ((type_list[cite_ptr] == 0 /*empty */ )) ;
                    else
                        execute_fn(type_list[cite_ptr]);
                }
                break;
            case 9:
                x_change_case();
                break;
            case 10:
                x_chr_to_int();
                break;
            case 11:
                x_cite();
                break;
            case 12:
                x_duplicate();
                break;
            case 13:
                x_empty();
                break;
            case 14:
                x_format_name();
                break;
            case 15:
                {
                    pop_lit_stk(pop_lit1, pop_typ1);
                    pop_lit_stk(pop_lit2, pop_typ2);
                    pop_lit_stk(pop_lit3, pop_typ3);
                    if ((pop_typ1 != 2 /*stk_fn */ ))
                        print_wrong_stk_lit(pop_lit1, pop_typ1, 2 /*stk_fn */ );
                    else if ((pop_typ2 != 2 /*stk_fn */ ))
                        print_wrong_stk_lit(pop_lit2, pop_typ2, 2 /*stk_fn */ );
                    else if ((pop_typ3 != 0 /*stk_int */ ))
                        print_wrong_stk_lit(pop_lit3, pop_typ3, 0 /*stk_int */ );
                    else if ((pop_lit3 > 0))
                        execute_fn(pop_lit2);
                    else
                        execute_fn(pop_lit1);
                }
                break;
            case 16:
                x_int_to_chr();
                break;
            case 17:
                x_int_to_str();
                break;
            case 18:
                x_missing();
                break;
            case 19:
                {
                    output_bbl_line();
                }
                break;
            case 20:
                x_num_names();
                break;
            case 21:
                {
                    pop_lit_stk(pop_lit1, pop_typ1);
                }
                break;
            case 22:
                x_preamble();
                break;
            case 23:
                x_purify();
                break;
            case 24:
                x_quote();
                break;
            case 25:
                {
                    ;
                }
                break;
            case 26:
                {
                    pop_whole_stack();
                }
                break;
            case 27:
                x_substring();
                break;
            case 28:
                x_swap();
                break;
            case 29:
                x_text_length();
                break;
            case 30:
                x_text_prefix();
                break;
            case 31:
                {
                    pop_top_and_print();
                }
                break;
            case 32:
                x_type();
                break;
            case 33:
                x_warning();
                break;
            case 34:
                {
                    pop_lit_stk(r_pop_lt1, r_pop_tp1);
                    pop_lit_stk(r_pop_lt2, r_pop_tp2);
                    if ((r_pop_tp1 != 2 /*stk_fn */ ))
                        print_wrong_stk_lit(r_pop_lt1, r_pop_tp1, 2 /*stk_fn */ );
                    else if ((r_pop_tp2 != 2 /*stk_fn */ ))
                        print_wrong_stk_lit(r_pop_lt2, r_pop_tp2, 2 /*stk_fn */ );
                    else
                        while (true) {

                            execute_fn(r_pop_lt2);
                            pop_lit_stk(pop_lit1, pop_typ1);
                            if ((pop_typ1 != 0 /*stk_int */ )) {
                                print_wrong_stk_lit(pop_lit1, pop_typ1, 0 /*stk_int */ );
                                goto lab51;
                            } else if ((pop_lit1 > 0))
                                execute_fn(r_pop_lt1);
                            else
                                goto lab51;
                        }
 lab51:                        /*end_while */ ;
                }
                break;
            case 35:
                x_width();
                break;
            case 36:
                x_write();
                break;
            default:
                {
                    {
                        Fputs(log_file, "Unknown built-in function");
                        Fputs(standard_output, "Unknown built-in function");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
                break;
            }
        }
        break;
    case 1:
        {
            wiz_ptr = ilk_info[ex_fn_loc];
            while ((wiz_functions[wiz_ptr] != end_of_def)) {

                if ((wiz_functions[wiz_ptr] != quote_next_fn))
                    execute_fn(wiz_functions[wiz_ptr]);
                else {

                    wiz_ptr = wiz_ptr + 1;
                    push_lit_stk(wiz_functions[wiz_ptr], 2 /*stk_fn */ );
                }
                wiz_ptr = wiz_ptr + 1;
            }
        }
        break;
    case 2:
        push_lit_stk(ilk_info[ex_fn_loc], 0 /*stk_int */ );
        break;
    case 3:
        push_lit_stk(hash_text[ex_fn_loc], 1 /*stk_str */ );
        break;
    case 4:
        {
            if ((!mess_with_entries))
                bst_cant_mess_with_entries_print();
            else {

                field_ptr = cite_ptr * num_fields + ilk_info[ex_fn_loc];
                if ((field_ptr >= max_fields)) {
                    {
                        Fputs(log_file, "field_info index is out of range");
                        Fputs(standard_output, "field_info index is out of range");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
                if ((field_info[field_ptr] == 0 /*missing */ ))
                    push_lit_stk(hash_text[ex_fn_loc], 3 /*stk_field_missing */ );
                else
                    push_lit_stk(field_info[field_ptr], 1 /*stk_str */ );
            }
        }
        break;
    case 5:
        {
            if ((!mess_with_entries))
                bst_cant_mess_with_entries_print();
            else
                push_lit_stk(entry_ints[cite_ptr * num_ent_ints + ilk_info[ex_fn_loc]], 0 /*stk_int */ );
        }
        break;
    case 6:
        {
            if ((!mess_with_entries))
                bst_cant_mess_with_entries_print();
            else {

                str_ent_ptr = cite_ptr * num_ent_strs + ilk_info[ex_fn_loc];
                ex_buf_ptr = 0;
                while ((entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ex_buf_ptr)] != 127 /*end_of_string */ )) {

                    ex_buf[ex_buf_ptr] = entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (ex_buf_ptr)];
                    ex_buf_ptr = ex_buf_ptr + 1;
                }
                ex_buf_length = ex_buf_ptr;
                add_pool_buf_and_push();
            }
        }
        break;
    case 7:
        push_lit_stk(ilk_info[ex_fn_loc], 0 /*stk_int */ );
        break;
    case 8:
        {
            str_glb_ptr = ilk_info[ex_fn_loc];
            if ((glb_str_ptr[str_glb_ptr] > 0))
                push_lit_stk(glb_str_ptr[str_glb_ptr], 1 /*stk_str */ );
            else {

                {
                    while ((pool_ptr + glb_str_end[str_glb_ptr] > pool_size))
                        pool_overflow();
                }
                glob_chr_ptr = 0;
                while ((glob_chr_ptr < glb_str_end[str_glb_ptr])) {

                    {
                        str_pool[pool_ptr] = global_strs[(str_glb_ptr) * (glob_str_size + 1) + (glob_chr_ptr)];
                        pool_ptr = pool_ptr + 1;
                    }
                    glob_chr_ptr = glob_chr_ptr + 1;
                }
                push_lit_stk(make_string(), 1 /*stk_str */ );
            }
        }
        break;
    default:
        unknwn_function_class_confusion();
        break;
    }
}

void get_the_top_level_aux_file_name(void)
{
    parse_arguments();
    name_of_file = xmalloc_array(ASCII_code, strlen(cmdline(optind)) + 5);
    strcpy(stringcast(name_of_file + 1), cmdline(optind));
    aux_name_length = strlen(stringcast(name_of_file + 1));
    {
        if (((aux_name_length + (str_start[s_aux_extension + 1] - str_start[s_aux_extension]) > maxint)
             || (aux_name_length + (str_start[s_log_extension + 1] - str_start[s_log_extension]) > maxint)
             || (aux_name_length + (str_start[s_bbl_extension + 1] - str_start[s_bbl_extension]) > maxint))) {
            sam_too_long_file_name_print();
            goto lab46;
        }
        {
            name_length = aux_name_length;
            if ((name_length < 4) || (strcmp(stringcast(name_of_file + 1 + name_length - 4), ".aux") != 0))
                add_extension(s_aux_extension);
            else
                aux_name_length = aux_name_length - 4;
            aux_ptr = 0;
            if ((!kpse_in_name_ok(stringcast(name_of_file + 1))
                 || !a_open_in(aux_file[aux_ptr], -1 /*no_file_path */ ))) {
                sam_wrong_file_name_print();
                goto lab46;
            }
            name_length = aux_name_length;
            add_extension(s_log_extension);
            if ((!kpse_out_name_ok(stringcast(name_of_file + 1)) || !a_open_out(log_file))) {
                sam_wrong_file_name_print();
                goto lab46;
            }
            name_length = aux_name_length;
            add_extension(s_bbl_extension);
            if ((!kpse_out_name_ok(stringcast(name_of_file + 1)) || !a_open_out(bbl_file))) {
                sam_wrong_file_name_print();
                goto lab46;
            }
        }
        {
            name_length = aux_name_length;
            add_extension(s_aux_extension);
            name_ptr = 1;
            while ((name_ptr <= name_length)) {

                buffer[name_ptr] = xord[name_of_file[name_ptr]];
                name_ptr = name_ptr + 1;
            }
            top_lev_str = hash_text[str_lookup(buffer, 1, aux_name_length, 0 /*text_ilk */ , true)];
            aux_list[aux_ptr] = hash_text[str_lookup(buffer, 1, name_length, 3 /*aux_file_ilk */ , true)];
            if ((hash_found)) {
                ;

#ifdef TRACE
                print_aux_name();

#endif                          /* TRACE */
                {
                    {
                        Fputs(log_file, "Already encountered auxiliary file");
                        Fputs(standard_output, "Already encountered auxiliary file");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
            }
            aux_ln_stack[aux_ptr] = 0;
        }
        goto lab41;
    }
 lab46:                        /*aux_not_found */ uexit(1);
 lab41:                        /*aux_found */ ;
}

void aux_bib_data_command(void)
{
    if ((bib_seen)) {
        aux_err_illegal_another_print(0 /*n_aux_bibdata */ );
        {
            aux_err_print();
            goto lab10;
        }
    }
    bib_seen = true;
    while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

        buf_ptr2 = buf_ptr2 + 1;
        if ((!scan2_white(125 /*right_brace */ , 44 /*comma */ ))) {
            aux_err_no_right_brace_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        if ((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )) {
            aux_err_white_space_in_argument_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        if (((last > buf_ptr2 + 1) && (buffer[buf_ptr2] == 125 /*right_brace */ ))) {
            aux_err_stuff_after_right_brace_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        {
            if ((bib_ptr == max_bib_files)) {
                BIB_XRETALLOC_NOSET("bib_list", bib_list, str_number, max_bib_files, max_bib_files + MAX_BIB_FILES);
                BIB_XRETALLOC_NOSET("bib_file", bib_file, alpha_file, max_bib_files, max_bib_files + MAX_BIB_FILES);
                BIB_XRETALLOC("s_preamble", s_preamble, str_number, max_bib_files, max_bib_files + MAX_BIB_FILES);
            }
            bib_list[bib_ptr] =
                hash_text[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 6 /*bib_file_ilk */ , true)];
            if ((hash_found)) {
                {
                    Fputs(log_file, "This database file appears more than once: ");
                    Fputs(standard_output, "This database file appears more than once: ");
                }
                print_bib_name();
                {
                    aux_err_print();
                    goto lab10;
                }
            }
            start_name(bib_list[bib_ptr]);
            if ((!kpse_in_name_ok(stringcast(name_of_file + 1)) || !a_open_in(bib_file[bib_ptr], kpse_bib_format))) {
                {
                    Fputs(log_file, "I couldn't open database file ");
                    Fputs(standard_output, "I couldn't open database file ");
                }
                print_bib_name();
                {
                    aux_err_print();
                    goto lab10;
                }
            };

#ifdef TRACE
            {
                out_pool_str(log_file, bib_list[bib_ptr]);
            }
            {
                out_pool_str(log_file, s_bib_extension);
            }
            {
                fprintf(log_file, "%s\n", " is a bibdata file");
            }

#endif                          /* TRACE */
            bib_ptr = bib_ptr + 1;
        }
    }
 lab10:                        /*exit */ ;
}

void aux_bib_style_command(void)
{
    if ((bst_seen)) {
        aux_err_illegal_another_print(1 /*n_aux_bibstyle */ );
        {
            aux_err_print();
            goto lab10;
        }
    }
    bst_seen = true;
    buf_ptr2 = buf_ptr2 + 1;
    if ((!scan1_white(125 /*right_brace */ ))) {
        aux_err_no_right_brace_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    if ((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )) {
        aux_err_white_space_in_argument_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    if ((last > buf_ptr2 + 1)) {
        aux_err_stuff_after_right_brace_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    {
        bst_str = hash_text[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 5 /*bst_file_ilk */ , true)];
        if ((hash_found)) {
            ;

#ifdef TRACE
            print_bst_name();

#endif                          /* TRACE */
            {
                {
                    Fputs(log_file, "Already encountered style file");
                    Fputs(standard_output, "Already encountered style file");
                }
                print_confusion();
                longjmp(jmp9998, 1);
            }
        }
        start_name(bst_str);
        if ((!kpse_in_name_ok(stringcast(name_of_file + 1)) || !a_open_in(bst_file, kpse_bst_format))) {
            {
                Fputs(log_file, "I couldn't open style file ");
                Fputs(standard_output, "I couldn't open style file ");
            }
            print_bst_name();
            bst_str = 0;
            {
                aux_err_print();
                goto lab10;
            }
        }
        if (verbose) {
            {
                Fputs(log_file, "The style file: ");
                Fputs(standard_output, "The style file: ");
            }
            print_bst_name();
        } else {

            {
                Fputs(log_file, "The style file: ");
            }
            log_pr_bst_name();
        }
    }
 lab10:                        /*exit */ ;
}

void aux_citation_command(void)
{
    citation_seen = true;
    while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

        buf_ptr2 = buf_ptr2 + 1;
        if ((!scan2_white(125 /*right_brace */ , 44 /*comma */ ))) {
            aux_err_no_right_brace_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        if ((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )) {
            aux_err_white_space_in_argument_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        if (((last > buf_ptr2 + 1) && (buffer[buf_ptr2] == 125 /*right_brace */ ))) {
            aux_err_stuff_after_right_brace_print();
            {
                aux_err_print();
                goto lab10;
            }
        }
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                Fputs(log_file, " cite key encountered");
            }

#endif                          /* TRACE */
            {
                if (((buf_ptr2 - buf_ptr1) == 1)) {

                    if ((buffer[buf_ptr1] == 42 /*star */ )) {
                        ;

#ifdef TRACE
                        {
                            fprintf(log_file, "%s\n", "---entire database to be included");
                        }

#endif                          /* TRACE */
                        if ((all_entries)) {
                            {
                                fprintf(log_file, "%s\n", "Multiple inclusions of entire database");
                                fprintf(standard_output, "%s\n", "Multiple inclusions of entire database");
                            }
                            {
                                aux_err_print();
                                goto lab10;
                            }
                        } else {

                            all_entries = true;
                            all_marker = cite_ptr;
                            goto lab23;
                        }
                    }
                }
            }
            tmp_ptr = buf_ptr1;
            while ((tmp_ptr < buf_ptr2)) {

                ex_buf[tmp_ptr] = buffer[tmp_ptr];
                tmp_ptr = tmp_ptr + 1;
            }
            lower_case(ex_buf, buf_ptr1, (buf_ptr2 - buf_ptr1));
            lc_cite_loc = str_lookup(ex_buf, buf_ptr1, (buf_ptr2 - buf_ptr1), 10 /*lc_cite_ilk */ , true);
            if ((hash_found)) { /*136: */
                ;

#ifdef TRACE
                {
                    fprintf(log_file, "%s\n", " previously");
                }

#endif                          /* TRACE */
                dummy_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 9 /*cite_ilk */ , false);
                if ((!hash_found)) {
                    {
                        Fputs(log_file, "Case mismatch error between cite keys ");
                        Fputs(standard_output, "Case mismatch error between cite keys ");
                    }
                    print_a_token();
                    {
                        Fputs(log_file, " and ");
                        Fputs(standard_output, " and ");
                    }
                    print_a_pool_str(cite_list[ilk_info[ilk_info[lc_cite_loc]]]);
                    print_a_newline();
                    {
                        aux_err_print();
                        goto lab10;
                    }
                }
            } else {            /*137: */

                ;

#ifdef TRACE
                {
                    putc('\n', log_file);
                }

#endif                          /* TRACE */
                cite_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 9 /*cite_ilk */ , true);
                if ((hash_found))
                    hash_cite_confusion();
                check_cite_overflow(cite_ptr);
                cite_list[cite_ptr] = hash_text[cite_loc];
                ilk_info[cite_loc] = cite_ptr;
                ilk_info[lc_cite_loc] = cite_loc;
                cite_ptr = cite_ptr + 1;
            }
        }
 lab23:                        /*next_cite */ ;
    }
 lab10:                        /*exit */ ;
}

void aux_input_command(void)
{
    boolean aux_extension_ok;
    buf_ptr2 = buf_ptr2 + 1;
    if ((!scan1_white(125 /*right_brace */ ))) {
        aux_err_no_right_brace_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    if ((lex_class[buffer[buf_ptr2]] == 1 /*white_space */ )) {
        aux_err_white_space_in_argument_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    if ((last > buf_ptr2 + 1)) {
        aux_err_stuff_after_right_brace_print();
        {
            aux_err_print();
            goto lab10;
        }
    }
    {
        aux_ptr = aux_ptr + 1;
        if ((aux_ptr == aux_stack_size)) {
            print_a_token();
            {
                Fputs(log_file, ": ");
                Fputs(standard_output, ": ");
            }
            {
                print_overflow();
                {
                    fprintf(log_file, "%s%ld\n", "auxiliary file depth ", (long)aux_stack_size);
                    fprintf(standard_output, "%s%ld\n", "auxiliary file depth ", (long)aux_stack_size);
                }
                longjmp(jmp9998, 1);
            }
        }
        aux_extension_ok = true;
        if (((buf_ptr2 - buf_ptr1) < (str_start[s_aux_extension + 1] - str_start[s_aux_extension])))
            aux_extension_ok = false;
        else if ((!str_eq_buf
                  (s_aux_extension, buffer, buf_ptr2 - (str_start[s_aux_extension + 1] - str_start[s_aux_extension]),
                   (str_start[s_aux_extension + 1] - str_start[s_aux_extension]))))
            aux_extension_ok = false;
        if ((!aux_extension_ok)) {
            print_a_token();
            {
                Fputs(log_file, " has a wrong extension");
                Fputs(standard_output, " has a wrong extension");
            }
            aux_ptr = aux_ptr - 1;
            {
                aux_err_print();
                goto lab10;
            }
        }
        aux_list[aux_ptr] = hash_text[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 3 /*aux_file_ilk */ , true)];
        if ((hash_found)) {
            {
                Fputs(log_file, "Already encountered file ");
                Fputs(standard_output, "Already encountered file ");
            }
            print_aux_name();
            aux_ptr = aux_ptr - 1;
            {
                aux_err_print();
                goto lab10;
            }
        }
        {
            start_name(aux_list[aux_ptr]);
            name_ptr = name_length + 1;
            name_of_file[name_ptr] = 0;
            if ((!kpse_in_name_ok(stringcast(name_of_file + 1))
                 || !a_open_in(aux_file[aux_ptr], -1 /*no_file_path */ ))) {
                {
                    Fputs(log_file, "I couldn't open auxiliary file ");
                    Fputs(standard_output, "I couldn't open auxiliary file ");
                }
                print_aux_name();
                aux_ptr = aux_ptr - 1;
                {
                    aux_err_print();
                    goto lab10;
                }
            }
            {
                fprintf(log_file, "%s%ld%s", "A level-", (long)aux_ptr, " auxiliary file: ");
                fprintf(standard_output, "%s%ld%s", "A level-", (long)aux_ptr, " auxiliary file: ");
            }
            print_aux_name();
            aux_ln_stack[aux_ptr] = 0;
        }
    }
 lab10:                        /*exit */ ;
}

void pop_the_aux_stack(void)
{
    a_close(aux_file[aux_ptr]);
    if ((aux_ptr == 0)) {
        lab31 = 1;
        return;
    } else
        aux_ptr = aux_ptr - 1;
}

void get_aux_command_and_process(void)
{
    buf_ptr2 = 0;
    if ((!scan1(123 /*left_brace */ )))
        goto lab10;
    command_num = ilk_info[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 2 /*aux_command_ilk */ , false)];
    if ((hash_found))
        switch ((command_num)) {
        case 0:
            aux_bib_data_command();
            break;
        case 1:
            aux_bib_style_command();
            break;
        case 2:
            aux_citation_command();
            break;
        case 3:
            aux_input_command();
            break;
        default:
            {
                {
                    Fputs(log_file, "Unknown auxiliary-file command");
                    Fputs(standard_output, "Unknown auxiliary-file command");
                }
                print_confusion();
                longjmp(jmp9998, 1);
            }
            break;
        }
 lab10:                        /*exit */ ;
}

void last_check_for_aux_errors(void)
{
    num_cites = cite_ptr;
    num_bib_files = bib_ptr;
    if ((!citation_seen)) {
        aux_end1_err_print();
        {
            Fputs(log_file, "\\citation commands");
            Fputs(standard_output, "\\citation commands");
        }
        aux_end2_err_print();
    } else if (((num_cites == 0) && (!all_entries))) {
        aux_end1_err_print();
        {
            Fputs(log_file, "cite keys");
            Fputs(standard_output, "cite keys");
        }
        aux_end2_err_print();
    }
    if ((!bib_seen)) {
        aux_end1_err_print();
        {
            Fputs(log_file, "\\bibdata command");
            Fputs(standard_output, "\\bibdata command");
        }
        aux_end2_err_print();
    } else if ((num_bib_files == 0)) {
        aux_end1_err_print();
        {
            Fputs(log_file, "database files");
            Fputs(standard_output, "database files");
        }
        aux_end2_err_print();
    }
    if ((!bst_seen)) {
        aux_end1_err_print();
        {
            Fputs(log_file, "\\bibstyle command");
            Fputs(standard_output, "\\bibstyle command");
        }
        aux_end2_err_print();
    } else if ((bst_str == 0)) {
        aux_end1_err_print();
        {
            Fputs(log_file, "style file");
            Fputs(standard_output, "style file");
        }
        aux_end2_err_print();
    }
}

void bst_entry_command(void)
{
    if ((entry_seen)) {
        {
            Fputs(log_file, "Illegal, another entry command");
            Fputs(standard_output, "Illegal, another entry command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    entry_seen = true;
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "entry");
                    Fputs(standard_output, "entry");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

            {
                scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
                if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                else {

                    bst_id_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
            {
                ;

#ifdef TRACE
                {
                    out_token(log_file);
                }
                {
                    fprintf(log_file, "%s\n", " is a field");
                }

#endif                          /* TRACE */
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
                {
                    if ((hash_found)) {
                        already_seen_function_print(fn_loc);
                        goto lab10;
                    }
                }
                fn_type[fn_loc] = 4 /*field */ ;
                ilk_info[fn_loc] = num_fields;
                num_fields = num_fields + 1;
            }
            {
                if ((!eat_bst_white_space())) {
                    eat_bst_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "entry");
                    Fputs(standard_output, "entry");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    if ((num_fields == num_pre_defined_fields)) {
        {
            Fputs(log_file, "Warning--I didn't find any fields");
            Fputs(standard_output, "Warning--I didn't find any fields");
        }
        bst_warn_print();
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

            {
                scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
                if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                else {

                    bst_id_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
            {
                ;

#ifdef TRACE
                {
                    out_token(log_file);
                }
                {
                    fprintf(log_file, "%s\n", " is an integer entry-variable");
                }

#endif                          /* TRACE */
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
                {
                    if ((hash_found)) {
                        already_seen_function_print(fn_loc);
                        goto lab10;
                    }
                }
                fn_type[fn_loc] = 5 /*int_entry_var */ ;
                ilk_info[fn_loc] = num_ent_ints;
                num_ent_ints = num_ent_ints + 1;
            }
            {
                if ((!eat_bst_white_space())) {
                    eat_bst_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "entry");
                    Fputs(standard_output, "entry");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "entry");
                        Fputs(standard_output, "entry");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

            {
                scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
                if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                else {

                    bst_id_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
            {
                ;

#ifdef TRACE
                {
                    out_token(log_file);
                }
                {
                    fprintf(log_file, "%s\n", " is a string entry-variable");
                }

#endif                          /* TRACE */
                lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
                {
                    if ((hash_found)) {
                        already_seen_function_print(fn_loc);
                        goto lab10;
                    }
                }
                fn_type[fn_loc] = 6 /*str_entry_var */ ;
                ilk_info[fn_loc] = num_ent_strs;
                num_ent_strs = num_ent_strs + 1;
            }
            {
                if ((!eat_bst_white_space())) {
                    eat_bst_print();
                    {
                        {
                            Fputs(log_file, "entry");
                            Fputs(standard_output, "entry");
                        }
                        {
                            bst_err_print_and_look_for_blank_line();
                            goto lab10;
                        }
                    }
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
 lab10:                        /*exit */ ;
}

boolean bad_argument_token(void)
{
    register boolean Result;
    Result = true;
    lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
    fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , false);
    if ((!hash_found)) {
        print_a_token();
        {
            {
                Fputs(log_file, " is an unknown function");
                Fputs(standard_output, " is an unknown function");
            }
            {
                bst_err_print_and_look_for_blank_line();
                goto lab10;
            }
        }
    } else if (((fn_type[fn_loc] != 0 /*built_in */ ) && (fn_type[fn_loc] != 1 /*wiz_defined */ ))) {
        print_a_token();
        {
            Fputs(log_file, " has bad function type ");
            Fputs(standard_output, " has bad function type ");
        }
        print_fn_class(fn_loc);
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    Result = false;
 lab10:                        /*exit */ ;
    return Result;
}

void bst_execute_command(void)
{
    if ((!read_seen)) {
        {
            Fputs(log_file, "Illegal, execute command before read command");
            Fputs(standard_output, "Illegal, execute command before read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
        if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
        else {

            bst_id_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        ;

#ifdef TRACE
        {
            out_token(log_file);
        }
        {
            fprintf(log_file, "%s\n", " is a to be executed function");
        }

#endif                          /* TRACE */
        if ((bad_argument_token()))
            goto lab10;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
            bst_right_brace_print();
            {
                {
                    Fputs(log_file, "execute");
                    Fputs(standard_output, "execute");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        init_command_execution();
        mess_with_entries = false;
        execute_fn(fn_loc);
        check_command_execution();
    }
 lab10:                        /*exit */ ;
}

void bst_function_command(void)
{
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "function");
                    Fputs(standard_output, "function");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
            if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
            else {

                bst_id_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                fprintf(log_file, "%s\n", " is a wizard-defined function");
            }

#endif                          /* TRACE */
            lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
            wiz_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
            {
                if ((hash_found)) {
                    already_seen_function_print(wiz_loc);
                    goto lab10;
                }
            }
            fn_type[wiz_loc] = 1 /*wiz_defined */ ;
            if ((hash_text[wiz_loc] == s_default))
                b_default = wiz_loc;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
                bst_right_brace_print();
                {
                    {
                        Fputs(log_file, "function");
                        Fputs(standard_output, "function");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "function");
                    Fputs(standard_output, "function");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "function");
                    Fputs(standard_output, "function");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    scan_fn_def(wiz_loc);
 lab10:                        /*exit */ ;
}

void bst_integers_command(void)
{
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "integers");
                    Fputs(standard_output, "integers");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "integers");
                    Fputs(standard_output, "integers");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "integers");
                    Fputs(standard_output, "integers");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

        {
            scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
            if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
            else {

                bst_id_print();
                {
                    {
                        Fputs(log_file, "integers");
                        Fputs(standard_output, "integers");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                fprintf(log_file, "%s\n", " is an integer global-variable");
            }

#endif                          /* TRACE */
            lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
            fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
            {
                if ((hash_found)) {
                    already_seen_function_print(fn_loc);
                    goto lab10;
                }
            }
            fn_type[fn_loc] = 7 /*int_global_var */ ;
            ilk_info[fn_loc] = 0;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "integers");
                        Fputs(standard_output, "integers");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
    }
    buf_ptr2 = buf_ptr2 + 1;
 lab10:                        /*exit */ ;
}

void bst_iterate_command(void)
{
    if ((!read_seen)) {
        {
            Fputs(log_file, "Illegal, iterate command before read command");
            Fputs(standard_output, "Illegal, iterate command before read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
        if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
        else {

            bst_id_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        ;

#ifdef TRACE
        {
            out_token(log_file);
        }
        {
            fprintf(log_file, "%s\n", " is a to be iterated function");
        }

#endif                          /* TRACE */
        if ((bad_argument_token()))
            goto lab10;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
            bst_right_brace_print();
            {
                {
                    Fputs(log_file, "iterate");
                    Fputs(standard_output, "iterate");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        init_command_execution();
        mess_with_entries = true;
        sort_cite_ptr = 0;
        while ((sort_cite_ptr < num_cites)) {

            cite_ptr = cite_info[sort_cite_ptr];
            ;

#ifdef TRACE
            {
                out_pool_str(log_file, hash_text[fn_loc]);
            }
            {
                Fputs(log_file, " to be iterated on ");
            }
            {
                out_pool_str(log_file, cite_list[cite_ptr]);
            }
            {
                putc('\n', log_file);
            }

#endif                          /* TRACE */
            execute_fn(fn_loc);
            check_command_execution();
            sort_cite_ptr = sort_cite_ptr + 1;
        }
    }
 lab10:                        /*exit */ ;
}

void bst_macro_command(void)
{
    if ((read_seen)) {
        {
            Fputs(log_file, "Illegal, macro command after read command");
            Fputs(standard_output, "Illegal, macro command after read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "macro");
                    Fputs(standard_output, "macro");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
            if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
            else {

                bst_id_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                fprintf(log_file, "%s\n", " is a macro");
            }

#endif                          /* TRACE */
            lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
            macro_name_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 13 /*macro_ilk */ , true);
            if ((hash_found)) {
                print_a_token();
                {
                    {
                        Fputs(log_file, " is already defined as a macro");
                        Fputs(standard_output, " is already defined as a macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            ilk_info[macro_name_loc] = hash_text[macro_name_loc];
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
                bst_right_brace_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "macro");
                    Fputs(standard_output, "macro");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        {
            if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
                bst_left_brace_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        if ((buffer[buf_ptr2] != 34 /*double_quote */ )) {
            {
                fprintf(log_file, "%s%c%s", "A macro definition must be ", xchr[34 /*double_quote */ ], "-delimited");
                fprintf(standard_output, "%s%c%s", "A macro definition must be ", xchr[34 /*double_quote */ ],
                        "-delimited");
            }
            {
                bst_err_print_and_look_for_blank_line();
                goto lab10;
            }
        }
        {
            buf_ptr2 = buf_ptr2 + 1;
            if ((!scan1(34 /*double_quote */ ))) {
                {
                    fprintf(log_file, "%s%c%s", "There's no `", xchr[34 /*double_quote */ ],
                            "' to end macro definition");
                    fprintf(standard_output, "%s%c%s", "There's no `", xchr[34 /*double_quote */ ],
                            "' to end macro definition");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            };

#ifdef TRACE
            {
                putc('"', log_file);
            }
            {
                out_token(log_file);
            }
            {
                putc('"', log_file);
            }
            {
                fprintf(log_file, "%s\n", " is a macro string");
            }

#endif                          /* TRACE */
            macro_def_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 0 /*text_ilk */ , true);
            fn_type[macro_def_loc] = 3 /*str_literal */ ;
            ilk_info[macro_name_loc] = hash_text[macro_def_loc];
            buf_ptr2 = buf_ptr2 + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
                bst_right_brace_print();
                {
                    {
                        Fputs(log_file, "macro");
                        Fputs(standard_output, "macro");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
            buf_ptr2 = buf_ptr2 + 1;
        }
    }
 lab10:                        /*exit */ ;
}

void get_bib_command_or_entry_and_process(void)
{
    at_bib_command = false;
    while ((!scan1(64 /*at_sign */ ))) {

        if ((!input_ln(bib_file[bib_ptr])))
            goto lab10;
        bib_line_num = bib_line_num + 1;
        buf_ptr2 = 0;
    }
    {
        if ((buffer[buf_ptr2] != 64 /*at_sign */ )) {
            {
                fprintf(log_file, "%s%c%s", "An \"", xchr[64 /*at_sign */ ], "\" disappeared");
                fprintf(standard_output, "%s%c%s", "An \"", xchr[64 /*at_sign */ ], "\" disappeared");
            }
            print_confusion();
            longjmp(jmp9998, 1);
        }
        buf_ptr2 = buf_ptr2 + 1;
        {
            if ((!eat_bib_white_space())) {
                eat_bib_print();
                goto lab10;
            }
        }
        scan_identifier(123 /*left_brace */ , 40 /*left_paren */ , 40 /*left_paren */ );
        {
            if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
            else {

                bib_id_print();
                {
                    {
                        Fputs(log_file, "an entry type");
                        Fputs(standard_output, "an entry type");
                    }
                    bib_err_print();
                    goto lab10;
                }
            }
        }
        ;

#ifdef TRACE
        {
            out_token(log_file);
        }
        {
            fprintf(log_file, "%s\n", " is an entry type or a database-file command");
        }

#endif                          /* TRACE */
        lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
        command_num = ilk_info[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 12 /*bib_command_ilk */ , false)];
        if ((hash_found)) {     /*240: */
            at_bib_command = true;
            switch ((command_num)) {
            case 0:
                {
                    goto lab10;
                }
                break;
            case 1:
                {
                    if ((preamble_ptr == max_bib_files)) {
                        BIB_XRETALLOC_NOSET("bib_list", bib_list, str_number, max_bib_files,
                                            max_bib_files + MAX_BIB_FILES);
                        BIB_XRETALLOC_NOSET("bib_file", bib_file, alpha_file, max_bib_files,
                                            max_bib_files + MAX_BIB_FILES);
                        BIB_XRETALLOC("s_preamble", s_preamble, str_number, max_bib_files,
                                      max_bib_files + MAX_BIB_FILES);
                    }
                    {
                        if ((!eat_bib_white_space())) {
                            eat_bib_print();
                            goto lab10;
                        }
                    }
                    if ((buffer[buf_ptr2] == 123 /*left_brace */ ))
                        right_outer_delim = 125 /*right_brace */ ;
                    else if ((buffer[buf_ptr2] == 40 /*left_paren */ ))
                        right_outer_delim = 41 /*right_paren */ ;
                    else {

                        bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */ );
                        goto lab10;
                    }
                    buf_ptr2 = buf_ptr2 + 1;
                    {
                        if ((!eat_bib_white_space())) {
                            eat_bib_print();
                            goto lab10;
                        }
                    }
                    store_field = true;
                    if ((!scan_and_store_the_field_value_and_eat_white()))
                        goto lab10;
                    if ((buffer[buf_ptr2] != right_outer_delim)) {
                        {
                            fprintf(log_file, "%s%c%s", "Missing \"", xchr[right_outer_delim],
                                    "\" in preamble command");
                            fprintf(standard_output, "%s%c%s", "Missing \"", xchr[right_outer_delim],
                                    "\" in preamble command");
                        }
                        bib_err_print();
                        goto lab10;
                    }
                    buf_ptr2 = buf_ptr2 + 1;
                    goto lab10;
                }
                break;
            case 2:
                {
                    {
                        if ((!eat_bib_white_space())) {
                            eat_bib_print();
                            goto lab10;
                        }
                    }
                    {
                        if ((buffer[buf_ptr2] == 123 /*left_brace */ ))
                            right_outer_delim = 125 /*right_brace */ ;
                        else if ((buffer[buf_ptr2] == 40 /*left_paren */ ))
                            right_outer_delim = 41 /*right_paren */ ;
                        else {

                            bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */ );
                            goto lab10;
                        }
                        buf_ptr2 = buf_ptr2 + 1;
                        {
                            if ((!eat_bib_white_space())) {
                                eat_bib_print();
                                goto lab10;
                            }
                        }
                        scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */ );
                        {
                            if (((scan_result == 3 /*white_adjacent */ )
                                 || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                            else {

                                bib_id_print();
                                {
                                    {
                                        Fputs(log_file, "a string name");
                                        Fputs(standard_output, "a string name");
                                    }
                                    bib_err_print();
                                    goto lab10;
                                }
                            }
                        }
                        {
                            ;

#ifdef TRACE
                            {
                                out_token(log_file);
                            }
                            {
                                fprintf(log_file, "%s\n", " is a database-defined macro");
                            }

#endif                          /* TRACE */
                            lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                            cur_macro_loc =
                                str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 13 /*macro_ilk */ , true);
                            ilk_info[cur_macro_loc] = hash_text[cur_macro_loc];
/*if(hash_found)then begin macro_warn_print;
begin begin write_ln(log_file,'having its definition overwritten');write_ln(standard_output,'having its definition overwritten');end;
bib_warn_print;end;end;*/ }
                    }
                    {
                        if ((!eat_bib_white_space())) {
                            eat_bib_print();
                            goto lab10;
                        }
                    }
                    {
                        if ((buffer[buf_ptr2] != 61 /*equals_sign */ )) {
                            bib_equals_sign_print();
                            goto lab10;
                        }
                        buf_ptr2 = buf_ptr2 + 1;
                        {
                            if ((!eat_bib_white_space())) {
                                eat_bib_print();
                                goto lab10;
                            }
                        }
                        store_field = true;
                        if ((!scan_and_store_the_field_value_and_eat_white()))
                            goto lab10;
                        if ((buffer[buf_ptr2] != right_outer_delim)) {
                            {
                                fprintf(log_file, "%s%c%s", "Missing \"", xchr[right_outer_delim],
                                        "\" in string command");
                                fprintf(standard_output, "%s%c%s", "Missing \"", xchr[right_outer_delim],
                                        "\" in string command");
                            }
                            bib_err_print();
                            goto lab10;
                        }
                        buf_ptr2 = buf_ptr2 + 1;
                    }
                    goto lab10;
                }
                break;
            default:
                bib_cmd_confusion();
                break;
            }
        } else {

            entry_type_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , false);
            if (((!hash_found) || (fn_type[entry_type_loc] != 1 /*wiz_defined */ )))
                type_exists = false;
            else
                type_exists = true;
        }
    }
    {
        if ((!eat_bib_white_space())) {
            eat_bib_print();
            goto lab10;
        }
    }
    {
        if ((buffer[buf_ptr2] == 123 /*left_brace */ ))
            right_outer_delim = 125 /*right_brace */ ;
        else if ((buffer[buf_ptr2] == 40 /*left_paren */ ))
            right_outer_delim = 41 /*right_paren */ ;
        else {

            bib_one_of_two_print(123 /*left_brace */ , 40 /*left_paren */ );
            goto lab10;
        }
        buf_ptr2 = buf_ptr2 + 1;
        {
            if ((!eat_bib_white_space())) {
                eat_bib_print();
                goto lab10;
            }
        }
        if ((right_outer_delim == 41 /*right_paren */ )) {
            if ((scan1_white(44 /*comma */ ))) ;
        } else if ((scan2_white(44 /*comma */ , 125 /*right_brace */ ))) ;
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                fprintf(log_file, "%s\n", " is a database key");
            }

#endif                          /* TRACE */
            tmp_ptr = buf_ptr1;
            while ((tmp_ptr < buf_ptr2)) {

                ex_buf[tmp_ptr] = buffer[tmp_ptr];
                tmp_ptr = tmp_ptr + 1;
            }
            lower_case(ex_buf, buf_ptr1, (buf_ptr2 - buf_ptr1));
            if ((all_entries))
                lc_cite_loc = str_lookup(ex_buf, buf_ptr1, (buf_ptr2 - buf_ptr1), 10 /*lc_cite_ilk */ , true);
            else
                lc_cite_loc = str_lookup(ex_buf, buf_ptr1, (buf_ptr2 - buf_ptr1), 10 /*lc_cite_ilk */ , false);
            if ((hash_found)) {
                entry_cite_ptr = ilk_info[ilk_info[lc_cite_loc]];
                {
                    if (((!all_entries) || (entry_cite_ptr < all_marker) || (entry_cite_ptr >= old_num_cites))) {
                        if ((type_list[entry_cite_ptr] == 0 /*empty */ )) {
                            {
                                if (((!all_entries) && (entry_cite_ptr >= old_num_cites))) {
                                    cite_loc =
                                        str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 9 /*cite_ilk */ , true);
                                    if ((!hash_found)) {
                                        ilk_info[lc_cite_loc] = cite_loc;
                                        ilk_info[cite_loc] = entry_cite_ptr;
                                        cite_list[entry_cite_ptr] = hash_text[cite_loc];
                                        hash_found = true;
                                    }
                                }
                            }
                            goto lab26;
                        }
                    } else if ((!entry_exists[entry_cite_ptr])) {
                        {
                            ex_buf_ptr = 0;
                            tmp_ptr = str_start[cite_info[entry_cite_ptr]];
                            tmp_end_ptr = str_start[cite_info[entry_cite_ptr] + 1];
                            while ((tmp_ptr < tmp_end_ptr)) {

                                ex_buf[ex_buf_ptr] = str_pool[tmp_ptr];
                                ex_buf_ptr = ex_buf_ptr + 1;
                                tmp_ptr = tmp_ptr + 1;
                            }
                            lower_case(ex_buf, 0,
                                       (str_start[cite_info[entry_cite_ptr] + 1] -
                                        str_start[cite_info[entry_cite_ptr]]));
                            lc_xcite_loc =
                                str_lookup(ex_buf, 0,
                                           (str_start[cite_info[entry_cite_ptr] + 1] -
                                            str_start[cite_info[entry_cite_ptr]]), 10 /*lc_cite_ilk */ , false);
                            if ((!hash_found))
                                cite_key_disappeared_confusion();
                        }
                        if ((lc_xcite_loc == lc_cite_loc))
                            goto lab26;
                    }
                    if ((type_list[entry_cite_ptr] == 0 /*empty */ )) {
                        {
                            Fputs(log_file, "The cite list is messed up");
                            Fputs(standard_output, "The cite list is messed up");
                        }
                        print_confusion();
                        longjmp(jmp9998, 1);
                    }
                    {
                        {
                            Fputs(log_file, "Repeated entry");
                            Fputs(standard_output, "Repeated entry");
                        }
                        bib_err_print();
                        goto lab10;
                    }
 lab26:                        /*first_time_entry */ ;
                }
            }
            store_entry = true;
            if ((all_entries)) {        /*273: */
                if ((hash_found)) {
                    if ((entry_cite_ptr < all_marker))
                        goto lab22;
                    else {

                        entry_exists[entry_cite_ptr] = true;
                        cite_loc = ilk_info[lc_cite_loc];
                    }
                } else {

                    cite_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 9 /*cite_ilk */ , true);
                    if ((hash_found))
                        hash_cite_confusion();
                }
                entry_cite_ptr = cite_ptr;
                add_database_cite(cite_ptr);
 lab22:                        /*cite_already_set */ ;
            } else if ((!hash_found))
                store_entry = false;
            if ((store_entry)) {        /*274: */
/*dummy_loc:=str_lookup(buffer,buf_ptr1,(buf_ptr2-buf_ptr1),9[cite_ilk],false);
if(not hash_found)then begin begin write(log_file,'Warning--case mismatch, database key "');
write(standard_output,'Warning--case mismatch, database key "');end;print_a_token;begin write(log_file,'", cite key "');
write(standard_output,'", cite key "');end;print_a_pool_str(cite_list[entry_cite_ptr]);begin begin write_ln(log_file,'"');
write_ln(standard_output,'"');end;bib_warn_print;end;end;*/ if ((type_exists))
                    type_list[entry_cite_ptr] = entry_type_loc;
                else {

                    type_list[entry_cite_ptr] = undefined;
                    {
                        Fputs(log_file, "Warning--entry type for \"");
                        Fputs(standard_output, "Warning--entry type for \"");
                    }
                    print_a_token();
                    {
                        {
                            fprintf(log_file, "%s\n", "\" isn't style-file defined");
                            fprintf(standard_output, "%s\n", "\" isn't style-file defined");
                        }
                        bib_warn_print();
                    }
                }
            }
        }
    }
    {
        if ((!eat_bib_white_space())) {
            eat_bib_print();
            goto lab10;
        }
    }
    {
        while ((buffer[buf_ptr2] != right_outer_delim)) {

            if ((buffer[buf_ptr2] != 44 /*comma */ )) {
                bib_one_of_two_print(44 /*comma */ , right_outer_delim);
                goto lab10;
            }
            buf_ptr2 = buf_ptr2 + 1;
            {
                if ((!eat_bib_white_space())) {
                    eat_bib_print();
                    goto lab10;
                }
            }
            if ((buffer[buf_ptr2] == right_outer_delim))
                goto lab15;
            {
                scan_identifier(61 /*equals_sign */ , 61 /*equals_sign */ , 61 /*equals_sign */ );
                {
                    if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
                    else {

                        bib_id_print();
                        {
                            {
                                Fputs(log_file, "a field name");
                                Fputs(standard_output, "a field name");
                            }
                            bib_err_print();
                            goto lab10;
                        }
                    }
                }
                ;

#ifdef TRACE
                {
                    out_token(log_file);
                }
                {
                    fprintf(log_file, "%s\n", " is a field name");
                }

#endif                          /* TRACE */
                store_field = false;
                if ((store_entry)) {
                    lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
                    field_name_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , false);
                    if ((hash_found)) {

                        if ((fn_type[field_name_loc] == 4 /*field */ ))
                            store_field = true;
                    }
                }
                {
                    if ((!eat_bib_white_space())) {
                        eat_bib_print();
                        goto lab10;
                    }
                }
                if ((buffer[buf_ptr2] != 61 /*equals_sign */ )) {
                    bib_equals_sign_print();
                    goto lab10;
                }
                buf_ptr2 = buf_ptr2 + 1;
            }
            {
                if ((!eat_bib_white_space())) {
                    eat_bib_print();
                    goto lab10;
                }
            }
            if ((!scan_and_store_the_field_value_and_eat_white()))
                goto lab10;
        }
 lab15:                        /*loop_exit */ buf_ptr2 = buf_ptr2 + 1;
    }
 lab10:                        /*exit */ ;
}

void bst_read_command(void)
{
    if ((read_seen)) {
        {
            Fputs(log_file, "Illegal, another read command");
            Fputs(standard_output, "Illegal, another read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    read_seen = true;
    if ((!entry_seen)) {
        {
            Fputs(log_file, "Illegal, read command before entry command");
            Fputs(standard_output, "Illegal, read command before entry command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    sv_ptr1 = buf_ptr2;
    sv_ptr2 = last;
    tmp_ptr = sv_ptr1;
    while ((tmp_ptr < sv_ptr2)) {

        sv_buffer[tmp_ptr] = buffer[tmp_ptr];
        tmp_ptr = tmp_ptr + 1;
    }
    {
        {
            {
                check_field_overflow(num_fields * num_cites);
                field_ptr = 0;
                while ((field_ptr < max_fields)) {

                    field_info[field_ptr] = 0 /*missing */ ;
                    field_ptr = field_ptr + 1;
                }
            }
            {
                cite_ptr = 0;
                while ((cite_ptr < max_cites)) {

                    type_list[cite_ptr] = 0 /*empty */ ;
                    cite_info[cite_ptr] = 0 /*any_value */ ;
                    cite_ptr = cite_ptr + 1;
                }
                old_num_cites = num_cites;
                if ((all_entries)) {
                    cite_ptr = all_marker;
                    while ((cite_ptr < old_num_cites)) {

                        cite_info[cite_ptr] = cite_list[cite_ptr];
                        entry_exists[cite_ptr] = false;
                        cite_ptr = cite_ptr + 1;
                    }
                    cite_ptr = all_marker;
                } else {

                    cite_ptr = num_cites;
                    all_marker = 0 /*any_value */ ;
                }
            }
        }
        read_performed = true;
        bib_ptr = 0;
        while ((bib_ptr < num_bib_files)) {

            if (verbose) {
                {
                    fprintf(log_file, "%s%ld%s", "Database file #", (long)bib_ptr + 1, ": ");
                    fprintf(standard_output, "%s%ld%s", "Database file #", (long)bib_ptr + 1, ": ");
                }
                print_bib_name();
            } else {

                {
                    fprintf(log_file, "%s%ld%s", "Database file #", (long)bib_ptr + 1, ": ");
                }
                log_pr_bib_name();
            }
            bib_line_num = 0;
            buf_ptr2 = last;
            while ((!eof(bib_file[bib_ptr])))
                get_bib_command_or_entry_and_process();
            a_close(bib_file[bib_ptr]);
            bib_ptr = bib_ptr + 1;
        }
        reading_completed = true;
        ;

#ifdef TRACE
        {
            fprintf(log_file, "%s\n", "Finished reading the database file(s)");
        }

#endif                          /* TRACE */
        {
            num_cites = cite_ptr;
            num_preamble_strings = preamble_ptr;
            {
                if (((num_cites - 1) * num_fields + crossref_num >= max_fields)) {
                    {
                        Fputs(log_file, "field_info index is out of range");
                        Fputs(standard_output, "field_info index is out of range");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
                cite_ptr = 0;
                while ((cite_ptr < num_cites)) {

                    field_ptr = cite_ptr * num_fields + crossref_num;
                    if ((field_info[field_ptr] != 0 /*missing */ )) {

                        if ((find_cite_locs_for_this_cite_key(field_info[field_ptr]))) {
                            cite_loc = ilk_info[lc_cite_loc];
                            field_info[field_ptr] = hash_text[cite_loc];
                            cite_parent_ptr = ilk_info[cite_loc];
                            field_ptr = cite_ptr * num_fields + num_pre_defined_fields;
                            field_end_ptr = field_ptr - num_pre_defined_fields + num_fields;
                            field_parent_ptr = cite_parent_ptr * num_fields + num_pre_defined_fields;
                            while ((field_ptr < field_end_ptr)) {

                                if ((field_info[field_ptr] == 0 /*missing */ ))
                                    field_info[field_ptr] = field_info[field_parent_ptr];
                                field_ptr = field_ptr + 1;
                                field_parent_ptr = field_parent_ptr + 1;
                            }
                        }
                    }
                    cite_ptr = cite_ptr + 1;
                }
            }
            {
                if (((num_cites - 1) * num_fields + crossref_num >= max_fields)) {
                    {
                        Fputs(log_file, "field_info index is out of range");
                        Fputs(standard_output, "field_info index is out of range");
                    }
                    print_confusion();
                    longjmp(jmp9998, 1);
                }
                cite_ptr = 0;
                while ((cite_ptr < num_cites)) {

                    field_ptr = cite_ptr * num_fields + crossref_num;
                    if ((field_info[field_ptr] != 0 /*missing */ )) {

                        if ((!find_cite_locs_for_this_cite_key(field_info[field_ptr]))) {
                            if ((cite_hash_found))
                                hash_cite_confusion();
                            nonexistent_cross_reference_error();
                            field_info[field_ptr] = 0 /*missing */ ;
                        } else {

                            if ((cite_loc != ilk_info[lc_cite_loc]))
                                hash_cite_confusion();
                            cite_parent_ptr = ilk_info[cite_loc];
                            if ((type_list[cite_parent_ptr] == 0 /*empty */ )) {
                                nonexistent_cross_reference_error();
                                field_info[field_ptr] = 0 /*missing */ ;
                            } else {

                                field_parent_ptr = cite_parent_ptr * num_fields + crossref_num;
                                if ((field_info[field_parent_ptr] != 0 /*missing */ )) {        /*283: */
                                    {
                                        Fputs(log_file, "Warning--you've nested cross references");
                                        Fputs(standard_output, "Warning--you've nested cross references");
                                    }
                                    bad_cross_reference_print(cite_list[cite_parent_ptr]);
                                    {
                                        fprintf(log_file, "%s\n", "\", which also refers to something");
                                        fprintf(standard_output, "%s\n", "\", which also refers to something");
                                    }
                                    mark_warning();
                                }
                                if (((!all_entries) && (cite_parent_ptr >= old_num_cites)
                                     && (cite_info[cite_parent_ptr] < min_crossrefs)))
                                    field_info[field_ptr] = 0 /*missing */ ;
                            }
                        }
                    }
                    cite_ptr = cite_ptr + 1;
                }
            }
            {
                cite_ptr = 0;
                while ((cite_ptr < num_cites)) {

                    if ((type_list[cite_ptr] == 0 /*empty */ ))
                        print_missing_entry(cite_list[cite_ptr]);
                    else if (((all_entries) || (cite_ptr < old_num_cites) || (cite_info[cite_ptr] >= min_crossrefs))) {
                        if ((cite_ptr > cite_xptr)) {   /*286: */
                            if (((cite_xptr + 1) * num_fields > max_fields)) {
                                {
                                    Fputs(log_file, "field_info index is out of range");
                                    Fputs(standard_output, "field_info index is out of range");
                                }
                                print_confusion();
                                longjmp(jmp9998, 1);
                            }
                            cite_list[cite_xptr] = cite_list[cite_ptr];
                            type_list[cite_xptr] = type_list[cite_ptr];
                            if ((!find_cite_locs_for_this_cite_key(cite_list[cite_ptr])))
                                cite_key_disappeared_confusion();
                            if (((!cite_hash_found) || (cite_loc != ilk_info[lc_cite_loc])))
                                hash_cite_confusion();
                            ilk_info[cite_loc] = cite_xptr;
                            field_ptr = cite_xptr * num_fields;
                            field_end_ptr = field_ptr + num_fields;
                            tmp_ptr = cite_ptr * num_fields;
                            while ((field_ptr < field_end_ptr)) {

                                field_info[field_ptr] = field_info[tmp_ptr];
                                field_ptr = field_ptr + 1;
                                tmp_ptr = tmp_ptr + 1;
                            }
                        }
                        cite_xptr = cite_xptr + 1;
                    }
                    cite_ptr = cite_ptr + 1;
                }
                num_cites = cite_xptr;
                if ((all_entries)) {    /*287: */
                    cite_ptr = all_marker;
                    while ((cite_ptr < old_num_cites)) {

                        if ((!entry_exists[cite_ptr]))
                            print_missing_entry(cite_info[cite_ptr]);
                        cite_ptr = cite_ptr + 1;
                    }
                }
            }
            {
                entry_ints = XTALLOC((num_ent_ints + 1) * (num_cites + 1), integer);
                int_ent_ptr = 0;
                while ((int_ent_ptr < num_ent_ints * num_cites)) {

                    entry_ints[int_ent_ptr] = 0;
                    int_ent_ptr = int_ent_ptr + 1;
                }
            }
            {
                entry_strs = XTALLOC((num_ent_strs + 1) * (num_cites + 1) * (ent_str_size + 1), ASCII_code);
                str_ent_ptr = 0;
                while ((str_ent_ptr < num_ent_strs * num_cites)) {

                    entry_strs[(str_ent_ptr) * (ent_str_size + 1) + (0)] = 127 /*end_of_string */ ;
                    str_ent_ptr = str_ent_ptr + 1;
                }
            }
            {
                cite_ptr = 0;
                while ((cite_ptr < num_cites)) {

                    cite_info[cite_ptr] = cite_ptr;
                    cite_ptr = cite_ptr + 1;
                }
            }
        }
        read_completed = true;
    }
    buf_ptr2 = sv_ptr1;
    last = sv_ptr2;
    tmp_ptr = buf_ptr2;
    while ((tmp_ptr < last)) {

        buffer[tmp_ptr] = sv_buffer[tmp_ptr];
        tmp_ptr = tmp_ptr + 1;
    }
 lab10:                        /*exit */ ;
}

void bst_reverse_command(void)
{
    if ((!read_seen)) {
        {
            Fputs(log_file, "Illegal, reverse command before read command");
            Fputs(standard_output, "Illegal, reverse command before read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
        if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
        else {

            bst_id_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        ;

#ifdef TRACE
        {
            out_token(log_file);
        }
        {
            fprintf(log_file, "%s\n", " is a to be iterated in reverse function");
        }

#endif                          /* TRACE */
        if ((bad_argument_token()))
            goto lab10;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 125 /*right_brace */ )) {
            bst_right_brace_print();
            {
                {
                    Fputs(log_file, "reverse");
                    Fputs(standard_output, "reverse");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        init_command_execution();
        mess_with_entries = true;
        if ((num_cites > 0)) {
            sort_cite_ptr = num_cites;
            do {
                sort_cite_ptr = sort_cite_ptr - 1;
                cite_ptr = cite_info[sort_cite_ptr];
                ;

#ifdef TRACE
                {
                    out_pool_str(log_file, hash_text[fn_loc]);
                }
                {
                    Fputs(log_file, " to be iterated in reverse on ");
                }
                {
                    out_pool_str(log_file, cite_list[cite_ptr]);
                }
                {
                    putc('\n', log_file);
                }

#endif                          /* TRACE */
                execute_fn(fn_loc);
                check_command_execution();
            } while (!((sort_cite_ptr == 0)));
        }
    }
 lab10:                        /*exit */ ;
}

void bst_sort_command(void)
{
    if ((!read_seen)) {
        {
            Fputs(log_file, "Illegal, sort command before read command");
            Fputs(standard_output, "Illegal, sort command before read command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    {
        ;

#ifdef TRACE
        {
            fprintf(log_file, "%s\n", "Sorting the entries");
        }

#endif                          /* TRACE */
        if ((num_cites > 1))
            quick_sort(0, num_cites - 1);
        ;

#ifdef TRACE
        {
            fprintf(log_file, "%s\n", "Done sorting");
        }

#endif                          /* TRACE */
    }
 lab10:                        /*exit */ ;
}

void bst_strings_command(void)
{
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "strings");
                    Fputs(standard_output, "strings");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    {
        if ((buffer[buf_ptr2] != 123 /*left_brace */ )) {
            bst_left_brace_print();
            {
                {
                    Fputs(log_file, "strings");
                    Fputs(standard_output, "strings");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
        buf_ptr2 = buf_ptr2 + 1;
    }
    {
        if ((!eat_bst_white_space())) {
            eat_bst_print();
            {
                {
                    Fputs(log_file, "strings");
                    Fputs(standard_output, "strings");
                }
                {
                    bst_err_print_and_look_for_blank_line();
                    goto lab10;
                }
            }
        }
    }
    while ((buffer[buf_ptr2] != 125 /*right_brace */ )) {

        {
            scan_identifier(125 /*right_brace */ , 37 /*comment */ , 37 /*comment */ );
            if (((scan_result == 3 /*white_adjacent */ ) || (scan_result == 1 /*specified_char_adjacent */ ))) ;
            else {

                bst_id_print();
                {
                    {
                        Fputs(log_file, "strings");
                        Fputs(standard_output, "strings");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
        {
            ;

#ifdef TRACE
            {
                out_token(log_file);
            }
            {
                fprintf(log_file, "%s\n", " is a string global-variable");
            }

#endif                          /* TRACE */
            lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
            fn_loc = str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 11 /*bst_fn_ilk */ , true);
            {
                if ((hash_found)) {
                    already_seen_function_print(fn_loc);
                    goto lab10;
                }
            }
            fn_type[fn_loc] = 8 /*str_global_var */ ;
            ilk_info[fn_loc] = num_glb_strs;
            if ((num_glb_strs == max_glob_strs)) {
                BIB_XRETALLOC_NOSET("glb_str_ptr", glb_str_ptr, str_number, max_glob_strs,
                                    max_glob_strs + MAX_GLOB_STRS);
                BIB_XRETALLOC_STRING("global_strs", global_strs, glob_str_size, max_glob_strs,
                                     max_glob_strs + MAX_GLOB_STRS);
                BIB_XRETALLOC("glb_str_end", glb_str_end, integer, max_glob_strs, max_glob_strs + MAX_GLOB_STRS);
                str_glb_ptr = num_glb_strs;
                while ((str_glb_ptr < max_glob_strs)) {

                    glb_str_ptr[str_glb_ptr] = 0;
                    glb_str_end[str_glb_ptr] = 0;
                    str_glb_ptr = str_glb_ptr + 1;
                }
            }
            num_glb_strs = num_glb_strs + 1;
        }
        {
            if ((!eat_bst_white_space())) {
                eat_bst_print();
                {
                    {
                        Fputs(log_file, "strings");
                        Fputs(standard_output, "strings");
                    }
                    {
                        bst_err_print_and_look_for_blank_line();
                        goto lab10;
                    }
                }
            }
        }
    }
    buf_ptr2 = buf_ptr2 + 1;
 lab10:                        /*exit */ ;
}

void get_bst_command_and_process(void)
{
    if ((!scan_alpha())) {
        {
            fprintf(log_file, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" can't start a style-file command");
            fprintf(standard_output, "%c%c%s", '"', xchr[buffer[buf_ptr2]], "\" can't start a style-file command");
        }
        {
            bst_err_print_and_look_for_blank_line();
            goto lab10;
        }
    }
    lower_case(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1));
    command_num = ilk_info[str_lookup(buffer, buf_ptr1, (buf_ptr2 - buf_ptr1), 4 /*bst_command_ilk */ , false)];
    if ((!hash_found)) {
        print_a_token();
        {
            {
                Fputs(log_file, " is an illegal style-file command");
                Fputs(standard_output, " is an illegal style-file command");
            }
            {
                bst_err_print_and_look_for_blank_line();
                goto lab10;
            }
        }
    }
    switch ((command_num)) {
    case 0:
        bst_entry_command();
        break;
    case 1:
        bst_execute_command();
        break;
    case 2:
        bst_function_command();
        break;
    case 3:
        bst_integers_command();
        break;
    case 4:
        bst_iterate_command();
        break;
    case 5:
        bst_macro_command();
        break;
    case 6:
        bst_read_command();
        break;
    case 7:
        bst_reverse_command();
        break;
    case 8:
        bst_sort_command();
        break;
    case 9:
        bst_strings_command();
        break;
    default:
        {
            {
                Fputs(log_file, "Unknown style-file command");
                Fputs(standard_output, "Unknown style-file command");
            }
            print_confusion();
            longjmp(jmp9998, 1);
        }
        break;
    }
 lab10:                        /*exit */ ;
}

void setup_params(void)
{
    integer bound_default;
    const_cstring bound_name;
    kpse_set_program_name(argv[0], "bibtex");
    bound_default = ENT_STR_SIZE;
    bound_name = "ent_str_size";
    setup_bound_variable(address_of(ent_str_size), bound_name, bound_default);
    if (ent_str_size < bound_default)
        ent_str_size = bound_default;
    bound_default = GLOB_STR_SIZE;
    bound_name = "glob_str_size";
    setup_bound_variable(address_of(glob_str_size), bound_name, bound_default);
    if (glob_str_size < bound_default)
        glob_str_size = bound_default;
    bound_default = MAX_STRINGS;
    bound_name = "max_strings";
    setup_bound_variable(address_of(max_strings), bound_name, bound_default);
    if (max_strings < bound_default)
        max_strings = bound_default;
    hash_size = max_strings;
    if (hash_size < 5000 /*HASH_SIZE */ )
        hash_size = 5000 /*HASH_SIZE */ ;
    hash_max = hash_size + hash_base - 1;
    end_of_def = hash_max + 1;
    undefined = hash_max + 1;
}

void compute_hash_prime(void)
{
    integer hash_want;
    integer k;
    integer j;
    integer o;
    integer square;
    integer n;
    boolean j_prime;
    hash_want = (hash_size / 20) * 17;
    j = 1;
    k = 1;
    hash_prime = 2;
    hash_next[k] = hash_prime;
    o = 2;
    square = 9;
    while (hash_prime < hash_want) {

        do {
            j = j + 2;
            if (j == square) {
                hash_text[o] = j;
                j = j + 2;
                o = o + 1;
                square = hash_next[o] * hash_next[o];
            }
            n = 2;
            j_prime = true;
            while ((n < o) && j_prime) {

                while (hash_text[n] < j)
                    hash_text[n] = hash_text[n] + 2 * hash_next[n];
                if (hash_text[n] == j)
                    j_prime = false;
                n = n + 1;
            }
        } while (!(j_prime));
        k = k + 1;
        hash_prime = j;
        hash_next[k] = hash_prime;
    }
}

void initialize(void)
{
    integer i;
    hash_loc k;
    bad = 0;
    if ((min_print_line < 3))
        bad = 1;
    if ((max_print_line <= min_print_line))
        bad = 10 * bad + 2;
    if ((max_print_line >= buf_size))
        bad = 10 * bad + 3;
    if ((hash_prime < 128))
        bad = 10 * bad + 4;
    if ((hash_prime > hash_size))
        bad = 10 * bad + 5;
    if ((hash_base != 1))
        bad = 10 * bad + 6;
    if ((max_strings > hash_size))
        bad = 10 * bad + 7;
    if ((max_cites > max_strings))
        bad = 10 * bad + 8;
    if ((10 /*short_list */  < 2 * 4 /*end_offset */  + 2))
        bad = 100 * bad + 22;
    if ((bad > 0)) {
        fprintf(standard_output, "%ld%s\n", (long)bad, " is a bad bad");
        uexit(1);
    }
    history = 0 /*spotless */ ;
    xchr[32] = ' ';
    xchr[33] = '!';
    xchr[34] = '"';
    xchr[35] = '#';
    xchr[36] = '$';
    xchr[37] = '%';
    xchr[38] = '&';
    xchr[39] = '\'';
    xchr[40] = '(';
    xchr[41] = ')';
    xchr[42] = '*';
    xchr[43] = '+';
    xchr[44] = ',';
    xchr[45] = '-';
    xchr[46] = '.';
    xchr[47] = '/';
    xchr[48] = '0';
    xchr[49] = '1';
    xchr[50] = '2';
    xchr[51] = '3';
    xchr[52] = '4';
    xchr[53] = '5';
    xchr[54] = '6';
    xchr[55] = '7';
    xchr[56] = '8';
    xchr[57] = '9';
    xchr[58] = ':';
    xchr[59] = ';';
    xchr[60] = '<';
    xchr[61] = '=';
    xchr[62] = '>';
    xchr[63] = '?';
    xchr[64] = '@';
    xchr[65] = 'A';
    xchr[66] = 'B';
    xchr[67] = 'C';
    xchr[68] = 'D';
    xchr[69] = 'E';
    xchr[70] = 'F';
    xchr[71] = 'G';
    xchr[72] = 'H';
    xchr[73] = 'I';
    xchr[74] = 'J';
    xchr[75] = 'K';
    xchr[76] = 'L';
    xchr[77] = 'M';
    xchr[78] = 'N';
    xchr[79] = 'O';
    xchr[80] = 'P';
    xchr[81] = 'Q';
    xchr[82] = 'R';
    xchr[83] = 'S';
    xchr[84] = 'T';
    xchr[85] = 'U';
    xchr[86] = 'V';
    xchr[87] = 'W';
    xchr[88] = 'X';
    xchr[89] = 'Y';
    xchr[90] = 'Z';
    xchr[91] = '[';
    xchr[92] = '\\';
    xchr[93] = ']';
    xchr[94] = '^';
    xchr[95] = '_';
    xchr[96] = '`';
    xchr[97] = 'a';
    xchr[98] = 'b';
    xchr[99] = 'c';
    xchr[100] = 'd';
    xchr[101] = 'e';
    xchr[102] = 'f';
    xchr[103] = 'g';
    xchr[104] = 'h';
    xchr[105] = 'i';
    xchr[106] = 'j';
    xchr[107] = 'k';
    xchr[108] = 'l';
    xchr[109] = 'm';
    xchr[110] = 'n';
    xchr[111] = 'o';
    xchr[112] = 'p';
    xchr[113] = 'q';
    xchr[114] = 'r';
    xchr[115] = 's';
    xchr[116] = 't';
    xchr[117] = 'u';
    xchr[118] = 'v';
    xchr[119] = 'w';
    xchr[120] = 'x';
    xchr[121] = 'y';
    xchr[122] = 'z';
    xchr[123] = '{';
    xchr[124] = '|';
    xchr[125] = '}';
    xchr[126] = '~';
    xchr[0] = ' ';
    xchr[127] = ' ';
    {
        register integer for_end;
        i = 0;
        for_end = 31;
        if (i <= for_end)
            do
                xchr[i] = chr(i);
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 127;
        for_end = 255;
        if (i <= for_end)
            do
                xchr[i] = chr(i);
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 0 /*first_text_char */ ;
        for_end = 255 /*last_text_char */ ;
        if (i <= for_end)
            do
                xord[xchr[i]] = i;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 0;
        for_end = 127;
        if (i <= for_end)
            do
                lex_class[i] = 5 /*other_lex */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 128;
        for_end = 255;
        if (i <= for_end)
            do
                lex_class[i] = 2 /*alpha */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 0;
        for_end = 31;
        if (i <= for_end)
            do
                lex_class[i] = 0 /*illegal */ ;
            while (i++ < for_end);
    }
    lex_class[127 /*invalid_code */ ] = 0 /*illegal */ ;
    lex_class[9 /*tab */ ] = 1 /*white_space */ ;
    lex_class[13] = 1 /*white_space */ ;
    lex_class[32 /*space */ ] = 1 /*white_space */ ;
    lex_class[126 /*tie */ ] = 4 /*sep_char */ ;
    lex_class[45 /*hyphen */ ] = 4 /*sep_char */ ;
    {
        register integer for_end;
        i = 48;
        for_end = 57;
        if (i <= for_end)
            do
                lex_class[i] = 3 /*numeric */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 65;
        for_end = 90;
        if (i <= for_end)
            do
                lex_class[i] = 2 /*alpha */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 97;
        for_end = 122;
        if (i <= for_end)
            do
                lex_class[i] = 2 /*alpha */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 0;
        for_end = 255;
        if (i <= for_end)
            do
                id_class[i] = 1 /*legal_id_char */ ;
            while (i++ < for_end);
    }
    {
        register integer for_end;
        i = 0;
        for_end = 31;
        if (i <= for_end)
            do
                id_class[i] = 0 /*illegal_id_char */ ;
            while (i++ < for_end);
    }
    id_class[32 /*space */ ] = 0 /*illegal_id_char */ ;
    id_class[9 /*tab */ ] = 0 /*illegal_id_char */ ;
    id_class[34 /*double_quote */ ] = 0 /*illegal_id_char */ ;
    id_class[35 /*number_sign */ ] = 0 /*illegal_id_char */ ;
    id_class[37 /*comment */ ] = 0 /*illegal_id_char */ ;
    id_class[39 /*single_quote */ ] = 0 /*illegal_id_char */ ;
    id_class[40 /*left_paren */ ] = 0 /*illegal_id_char */ ;
    id_class[41 /*right_paren */ ] = 0 /*illegal_id_char */ ;
    id_class[44 /*comma */ ] = 0 /*illegal_id_char */ ;
    id_class[61 /*equals_sign */ ] = 0 /*illegal_id_char */ ;
    id_class[123 /*left_brace */ ] = 0 /*illegal_id_char */ ;
    id_class[125 /*right_brace */ ] = 0 /*illegal_id_char */ ;
    {
        register integer for_end;
        i = 0;
        for_end = 127;
        if (i <= for_end)
            do
                char_width[i] = 0;
            while (i++ < for_end);
    }
    char_width[32] = 278;
    char_width[33] = 278;
    char_width[34] = 500;
    char_width[35] = 833;
    char_width[36] = 500;
    char_width[37] = 833;
    char_width[38] = 778;
    char_width[39] = 278;
    char_width[40] = 389;
    char_width[41] = 389;
    char_width[42] = 500;
    char_width[43] = 778;
    char_width[44] = 278;
    char_width[45] = 333;
    char_width[46] = 278;
    char_width[47] = 500;
    char_width[48] = 500;
    char_width[49] = 500;
    char_width[50] = 500;
    char_width[51] = 500;
    char_width[52] = 500;
    char_width[53] = 500;
    char_width[54] = 500;
    char_width[55] = 500;
    char_width[56] = 500;
    char_width[57] = 500;
    char_width[58] = 278;
    char_width[59] = 278;
    char_width[60] = 278;
    char_width[61] = 778;
    char_width[62] = 472;
    char_width[63] = 472;
    char_width[64] = 778;
    char_width[65] = 750;
    char_width[66] = 708;
    char_width[67] = 722;
    char_width[68] = 764;
    char_width[69] = 681;
    char_width[70] = 653;
    char_width[71] = 785;
    char_width[72] = 750;
    char_width[73] = 361;
    char_width[74] = 514;
    char_width[75] = 778;
    char_width[76] = 625;
    char_width[77] = 917;
    char_width[78] = 750;
    char_width[79] = 778;
    char_width[80] = 681;
    char_width[81] = 778;
    char_width[82] = 736;
    char_width[83] = 556;
    char_width[84] = 722;
    char_width[85] = 750;
    char_width[86] = 750;
    char_width[87] = 1028;
    char_width[88] = 750;
    char_width[89] = 750;
    char_width[90] = 611;
    char_width[91] = 278;
    char_width[92] = 500;
    char_width[93] = 278;
    char_width[94] = 500;
    char_width[95] = 278;
    char_width[96] = 278;
    char_width[97] = 500;
    char_width[98] = 556;
    char_width[99] = 444;
    char_width[100] = 556;
    char_width[101] = 444;
    char_width[102] = 306;
    char_width[103] = 500;
    char_width[104] = 556;
    char_width[105] = 278;
    char_width[106] = 306;
    char_width[107] = 528;
    char_width[108] = 278;
    char_width[109] = 833;
    char_width[110] = 556;
    char_width[111] = 500;
    char_width[112] = 556;
    char_width[113] = 528;
    char_width[114] = 392;
    char_width[115] = 394;
    char_width[116] = 389;
    char_width[117] = 556;
    char_width[118] = 528;
    char_width[119] = 722;
    char_width[120] = 528;
    char_width[121] = 528;
    char_width[122] = 444;
    char_width[123] = 500;
    char_width[124] = 1000;
    char_width[125] = 500;
    char_width[126] = 500;
    {
        register integer for_end;
        k = hash_base;
        for_end = hash_max;
        if (k <= for_end)
            do {
                hash_next[k] = 0 /*empty */ ;
                hash_text[k] = 0;
            }
            while (k++ < for_end);
    }
    hash_used = hash_max + 1;
    pool_ptr = 0;
    str_ptr = 1;
    str_start[str_ptr] = pool_ptr;
    bib_ptr = 0;
    bib_seen = false;
    bst_str = 0;
    bst_seen = false;
    cite_ptr = 0;
    citation_seen = false;
    all_entries = false;
    wiz_def_ptr = 0;
    num_ent_ints = 0;
    num_ent_strs = 0;
    num_fields = 0;
    str_glb_ptr = 0;
    while ((str_glb_ptr < max_glob_strs)) {

        glb_str_ptr[str_glb_ptr] = 0;
        glb_str_end[str_glb_ptr] = 0;
        str_glb_ptr = str_glb_ptr + 1;
    }
    num_glb_strs = 0;
    entry_seen = false;
    read_seen = false;
    read_performed = false;
    reading_completed = false;
    read_completed = false;
    impl_fn_num = 0;
    out_buf_length = 0;
    pre_def_certain_strings();
    get_the_top_level_aux_file_name();
}

void parse_arguments(void)
{

#define n_options ( 4 )
    getopt_struct long_options[n_options + 1];
    integer getopt_return_val;
    c_int_type option_index;
    integer current_option;
    verbose = true;
    min_crossrefs = 2;
    current_option = 0;
    long_options[0].name = "terse";
    long_options[0].has_arg = 0;
    long_options[0].flag = address_of(verbose);
    long_options[0].val = 0;
    current_option = current_option + 1;
    long_options[current_option].name = "min-crossrefs";
    long_options[current_option].has_arg = 1;
    long_options[current_option].flag = 0;
    long_options[current_option].val = 0;
    current_option = current_option + 1;
    long_options[current_option].name = "help";
    long_options[current_option].has_arg = 0;
    long_options[current_option].flag = 0;
    long_options[current_option].val = 0;
    current_option = current_option + 1;
    long_options[current_option].name = "version";
    long_options[current_option].has_arg = 0;
    long_options[current_option].flag = 0;
    long_options[current_option].val = 0;
    current_option = current_option + 1;
    long_options[current_option].name = 0;
    long_options[current_option].has_arg = 0;
    long_options[current_option].flag = 0;
    long_options[current_option].val = 0;
    do {
        getopt_return_val = getopt_long_only(argc, argv, "", long_options, address_of(option_index));
        if (getopt_return_val == -1) {
            ;
        } else if (getopt_return_val == 63 /*"?" */ ) {
            usage("bibtex");
        } else if ((strcmp(long_options[option_index].name, "min-crossrefs") == 0)) {
            min_crossrefs = atoi(optarg);
        } else if ((strcmp(long_options[option_index].name, "help") == 0)) {
            usage_help(BIBTEX_HELP, nil);
        } else if ((strcmp(long_options[option_index].name, "version") == 0)) {
            print_version_and_exit("This is BibTeX, Version 0.99d", "Oren Patashnik", nil, nil);
        }
    } while (!(getopt_return_val == -1));
    if ((optind + 1 != argc)) {
        fprintf(stderr, "%s%s\n", "bibtex", ": Need exactly one file argument.");
        usage("bibtex");
    }
}

void main_body(void)
{

    standard_input = stdin;
    standard_output = stdout;
    pool_size = POOL_SIZE;
    buf_size = BUF_SIZE;
    max_bib_files = MAX_BIB_FILES;
    max_glob_strs = MAX_GLOB_STRS;
    max_fields = MAX_FIELDS;
    max_cites = MAX_CITES;
    wiz_fn_space = WIZ_FN_SPACE;
    lit_stk_size = LIT_STK_SIZE;
    setup_params();
    bib_file = XTALLOC(max_bib_files + 1, alpha_file);
    bib_list = XTALLOC(max_bib_files + 1, str_number);
    entry_ints = nil;
    entry_strs = nil;
    wiz_functions = XTALLOC(wiz_fn_space + 1, hash_ptr2);
    field_info = XTALLOC(max_fields + 1, str_number);
    s_preamble = XTALLOC(max_bib_files + 1, str_number);
    str_pool = XTALLOC(pool_size + 1, ASCII_code);
    buffer = XTALLOC(buf_size + 1, ASCII_code);
    sv_buffer = XTALLOC(buf_size + 1, ASCII_code);
    ex_buf = XTALLOC(buf_size + 1, ASCII_code);
    out_buf = XTALLOC(buf_size + 1, ASCII_code);
    name_tok = XTALLOC(buf_size + 1, buf_pointer);
    name_sep_char = XTALLOC(buf_size + 1, ASCII_code);
    glb_str_ptr = XTALLOC(max_glob_strs, str_number);
    global_strs = XTALLOC(max_glob_strs * (glob_str_size + 1), ASCII_code);
    glb_str_end = XTALLOC(max_glob_strs, integer);
    cite_list = XTALLOC(max_cites + 1, str_number);
    type_list = XTALLOC(max_cites + 1, hash_ptr2);
    entry_exists = XTALLOC(max_cites + 1, boolean);
    cite_info = XTALLOC(max_cites + 1, str_number);
    str_start = XTALLOC(max_strings + 1, pool_pointer);
    hash_next = XTALLOC(hash_max + 1, hash_pointer);
    hash_text = XTALLOC(hash_max + 1, str_number);
    hash_ilk = XTALLOC(hash_max + 1, str_ilk);
    ilk_info = XTALLOC(hash_max + 1, integer);
    fn_type = XTALLOC(hash_max + 1, fn_class);
    lit_stack = XTALLOC(lit_stk_size + 1, integer);
    lit_stk_type = XTALLOC(lit_stk_size + 1, stk_type);
    compute_hash_prime();
    initialize();
    if (setjmp(jmp9998) == 1)
        goto lab9998;
    if (verbose) {
        {
            Fputs(log_file, "This is BibTeX, Version 0.99d");
            Fputs(standard_output, "This is BibTeX, Version 0.99d");
        }
        {
            fprintf(log_file, "%s\n", version_string);
            fprintf(standard_output, "%s\n", version_string);
        }
    } else {

        {
            Fputs(log_file, "This is BibTeX, Version 0.99d");
        }
        {
            fprintf(log_file, "%s\n", version_string);
        }
    }
    {
        fprintf(log_file, "%s%ld%s%ld%s%ld\n", "Capacity: max_strings=", (long)max_strings, ", hash_size=",
                (long)hash_size, ", hash_prime=", (long)hash_prime);
    }
    if (verbose) {
        {
            Fputs(log_file, "The top-level auxiliary file: ");
            Fputs(standard_output, "The top-level auxiliary file: ");
        }
        print_aux_name();
    } else {

        {
            Fputs(log_file, "The top-level auxiliary file: ");
        }
        log_pr_aux_name();
    }
    while (true) {

        aux_ln_stack[aux_ptr] = aux_ln_stack[aux_ptr] + 1;
        if ((!input_ln(aux_file[aux_ptr])))
            pop_the_aux_stack();
        else
            get_aux_command_and_process();
    }

#ifdef TRACE
    {
        fprintf(log_file, "%s\n", "Finished reading the auxiliary file(s)");
    }

#endif                          /* TRACE */
    last_check_for_aux_errors();
    if ((bst_str == 0))
        goto lab9932;
    bst_line_num = 0;
    bbl_line_num = 1;
    buf_ptr2 = last;
    if (setjmp(jmp32) == 0)
        for (;;) {
            if ((!eat_bst_white_space()))
                break;
            get_bst_command_and_process();
        }
    a_close(bst_file);
 lab9932:                      /*no_bst_file */ a_close(bbl_file);
 lab9998:                      /*close_up_shop *//*456: */  {

        if (((read_performed) && (!reading_completed))) {
            {
                fprintf(log_file, "%s%ld%s", "Aborted at line ", (long)bib_line_num, " of file ");
                fprintf(standard_output, "%s%ld%s", "Aborted at line ", (long)bib_line_num, " of file ");
            }
            print_bib_name();
        }
        trace_and_stat_printing();
        switch ((history)) {
        case 0:
            ;
            break;
        case 1:
            {
                if ((err_count == 1)) {
                    fprintf(log_file, "%s\n", "(There was 1 warning)");
                    fprintf(standard_output, "%s\n", "(There was 1 warning)");
                } else {

                    fprintf(log_file, "%s%ld%s\n", "(There were ", (long)err_count, " warnings)");
                    fprintf(standard_output, "%s%ld%s\n", "(There were ", (long)err_count, " warnings)");
                }
            }
            break;
        case 2:
            {
                if ((err_count == 1)) {
                    fprintf(log_file, "%s\n", "(There was 1 error message)");
                    fprintf(standard_output, "%s\n", "(There was 1 error message)");
                } else {

                    fprintf(log_file, "%s%ld%s\n", "(There were ", (long)err_count, " error messages)");
                    fprintf(standard_output, "%s%ld%s\n", "(There were ", (long)err_count, " error messages)");
                }
            }
            break;
        case 3:
            {
                fprintf(log_file, "%s\n", "(That was a fatal error)");
                fprintf(standard_output, "%s\n", "(That was a fatal error)");
            }
            break;
        default:
            {
                {
                    Fputs(log_file, "History is bunk");
                    Fputs(standard_output, "History is bunk");
                }
                print_confusion();
            }
            break;
        }
        a_close(log_file);
    }
    if ((history > 1))
        uexit(history);
}
