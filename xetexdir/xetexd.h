#undef TRIP
#undef TRAP
#define STAT
#define INI
#define INITEX
#define TeX
#define XeTeX
/*common.defines.  Public domain. This file is concatenated by ./convert to the beginning of the Pascal
 code that tangle outputs.  The idea is to put all these symbols, which
 can be defined as macros or functions, or as part of standard C, into
 web2c's symbol table, so that we can access them from a change file. Some are standard Pascal functions, others are simply used in our
                                                                  implementation. web2c.yacc can parse these @define statements.*//*The fields in the memory_word structure, defined in
                                                                                                                                                                                                                                                                                                                                               `mfware/gftodmem.h' and `common/texmf.h'. *//*These fields are the ones defined by the getopt library. *//*This is used by \TeX--XeT. *//*@define @field rh; *//*For BibTeX. *//*can't keep |break|, since it's a reserved word *//*for gftodvi, TeX, and Metafont *//*These are all set by getopt.  optiontable is really _getopt_long_options. *//*This file defines symbols in addition to those in `common.defines',
                                                                                                                                                                                  for use in the TeX, Metafont, and MetaPost change files.  Public domain. *//*`qqqq' is already defined, in ./common.defines, because of gftodvi. *//*For TeX; see openclose.c. *//*
                                                                                                                                                                                  Copyright (c) 2008, 2009 jerome DOT laurens AT u-bourgogne DOT fr

                                                                                                                                                                                  This file is part of the SyncTeX package. Latest Revision: Wed Jul  1 11:18:05 UTC 2009

                                                                                                                                                                                  License:
                                                                                                                                                                                  --------
                                                                                                                                                                                  Permission is hereby granted, free of charge, to any person
                                                                                                                                                                                  obtaining a copy of this software and associated documentation
                                                                                                                                                                                  files (the "Software"), to deal in the Software without
                                                                                                                                                                                  restriction, including without limitation the rights to use,
                                                                                                                                                                                  copy, modify, merge, publish, distribute, sublicense, and/or sell
                                                                                                                                                                                  copies of the Software, and to permit persons to whom the
                                                                                                                                                                                  Software is furnished to do so, subject to the following
                                                                                                                                                                                  conditions:

                                                                                                                                                                                  The above copyright notice and this permission notice shall be
                                                                                                                                                                                  included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
                                                                                                                                                                                  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
                                                                                                                                                                                  OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
                                                                                                                                                                                  NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
                                                                                                                                                                                  HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
                                                                                                                                                                                  WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
                                                                                                                                                                                  FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
                                                                                                                                                                                  OTHER DEALINGS IN THE SOFTWARE

                                                                                                                                                                                  Except as contained in this notice, the name of the copyright holder  
                                                                                                                                                                                  shall not be used in advertising or otherwise to promote the sale,  
                                                                                                                                                                                  use or other dealings in this Software without prior written  
                                                                                                                                                                                  authorization from the copyright holder. Acknowledgments:
                                                                                                                                                                                  ----------------
                                                                                                                                                                                  The author received useful remarks from the pdfTeX developers, especially Hahn The Thanh,
                                                                                                                                                                                  and significant help from XeTeX developer Jonathan Kew

                                                                                                                                                                                  Nota Bene:
                                                                                                                                                                                  ----------
                                                                                                                                                                                  If you include or use a significant part of the synctex package into a software,
                                                                                                                                                                                  I would appreciate to be listed as contributor and see "SyncTeX" highlighted. Version 1
                                                                                                                                                                                  Latest Revision: Wed Jul  1 08:17:41 UTC 2009

                                                                                                                                                                                  Notice:
                                                                                                                                                                                  -------
                                                                                                                                                                                  This file is an interface to the synctex system for web2c. It declares the public functions API of synctex.c. It is always embedded as common definitions when convert'ing
                                                                                                                                                                  from web to c (See the convert shell script). *//* functions from the synctex controller in synctex.c *//* end of synctex.defines *//* vim: set syntax=web : *//*
                                                                                                                                                                  Part of the XeTeX typesetting system
                                                                                                                                                                  Copyright (c) 1994-2008 by SIL International
                                                                                                                                                                  Copyright (c) 2009 by Jonathan Kew

                                                                                                                                                                  SIL Author(s): Jonathan Kew

                                                                                                                                                                  Permission is hereby granted, free of charge, to any person obtaining
                                                                                                                                                                  a copy of this software and associated documentation files (the
                                                                                                                                                                  "Software"), to deal in the Software without restriction, including
                                                                                                                                                                  without limitation the rights to use, copy, modify, merge, publish,
                                                                                                                                                                  distribute, sublicense, and/or sell copies of the Software, and to
                                                                                                                                                                  permit persons to whom the Software is furnished to do so, subject to
                                                                                                                                                                  the following conditions:

                                                                                                                                                                  The above copyright notice and this permission notice shall be
                                                                                                                                                                  included in all copies or substantial portions of the Software. THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
                                                                                                                                                                  EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
                                                                                                                                                                  MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
                                                                                                                                                                  NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
                                                                                                                                                                  FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
                                                                                                                                                                  CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
                                                                                                                                                                  WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. Except as contained in this notice, the name of the copyright holders
                                                                                                                                                                  shall not be used in advertising or otherwise to promote the sale,
                                                                                                                                                                  use or other dealings in this Software without prior written
                                                                                                                                         authorization from the copyright holders. *//* xetex.defines: additions for xetex *//* extra stuff used in picfile code *//*4: *//*9: *//*$C-,A+,D- */
#ifdef TEXMF_DEBUG
/*$C+,D+*/
#endif                          /* TEXMF_DEBUG */
/*:9*/
#define XETEXCOERCE
#include "texmfmp.h"
#define /*11:*/hash_offset ( 514 )
#define trie_op_size ( 35111L )
#define neg_trie_op_size ( -35111L )
#define min_trie_op ( 0 )
#define max_trie_op ( 65535L )
#define pool_name ( TEXMF_POOL_NAME )
#define engine_name ( TEXMF_ENGINE_NAME )
#define inf_mem_bot ( 0 )
#define sup_mem_bot ( 1 )
#define inf_main_memory ( 3000 )
#define sup_main_memory ( 256000000L )
#define inf_trie_size ( 8000 )
#define sup_trie_size ( 4194303L )
#define inf_max_strings ( 3000 )
#define sup_max_strings ( 2097151L )
#define inf_strings_free ( 100 )
#define sup_strings_free ( sup_max_strings )
#define inf_buf_size ( 500 )
#define sup_buf_size ( 30000000L )
#define inf_nest_size ( 40 )
#define sup_nest_size ( 4000 )
#define inf_max_in_open ( 6 )
#define sup_max_in_open ( 127 )
#define inf_param_size ( 60 )
#define sup_param_size ( 32767 )
#define inf_save_size ( 600 )
#define sup_save_size ( 80000L )
#define inf_stack_size ( 200 )
#define sup_stack_size ( 30000 )
#define inf_dvi_buf_size ( 800 )
#define sup_dvi_buf_size ( 65536L )
#define inf_font_mem_size ( 20000 )
#define sup_font_mem_size ( 147483647L )
#define sup_font_max ( 9000 /*max_font_max*/)
#define inf_font_max ( 50 )
#define inf_pool_size ( 32000 )
#define sup_pool_size ( 40000000L )
#define inf_pool_free ( 1000 )
#define sup_pool_free ( sup_pool_size )
#define inf_string_vacancies ( 8000 )
#define sup_string_vacancies ( sup_pool_size - 23000 )
#define sup_hash_extra ( sup_max_strings )
#define inf_hash_extra ( 0 )
#define sup_hyph_size ( 65535L )
#define inf_hyph_size ( 610 )
#define inf_expand_depth ( 10 )
#define sup_expand_depth ( 10000000L )
typedef /*18: */ unsigned short /*biggest_char */ UTF16_code;
typedef unsigned char UTF8_code;
typedef integer /*biggest_usv */ UnicodeScalar;
typedef unsigned char eight_bits;
typedef text /* of  UTF16_code */ alpha_file;
typedef text /* of  eight_bits */ byte_file;
typedef integer pool_pointer;
typedef integer str_number;
typedef unsigned short /*biggest_char */ packed_UTF16_code;
typedef integer scaled;
typedef integer nonnegative_integer;
typedef short /*hyphenatable_length_limit */ small_number;
typedef /*min_quarterword */ unsigned short /*max_quarterword */ quarterword;
typedef integer halfword;
typedef unsigned char two_choices;
typedef unsigned char four_choices;

#include "texmfmem.h"
typedef gzFile word_file;
typedef /*normal */ unsigned char /*filll */ glue_ord;
typedef struct {
    short mode_field;
    halfword head_field, tail_field;
    halfword eTeX_aux_field;
    integer pg_field, ml_field;
    memory_word aux_field;
} list_state_record;
typedef unsigned char /*max_group_code */ group_code;
typedef struct {
    quarterword state_field, index_field;
    halfword start_field, loc_field, limit_field, name_field;
    integer synctex_tag_field;
} in_state_record;
typedef integer internal_font_number;
typedef integer font_index;
typedef /*min_quarterword */ integer /*too_big_char */ nine_bits;
typedef integer dvi_index;
typedef integer trie_pointer;
typedef unsigned short trie_opcode;
typedef unsigned short hyph_pointer;
typedef integer save_pointer;
integer bad;
UTF16_code *xchr;
UTF8_code *name_of_file;
UTF16_code *name_of_file16;
integer name_length;
integer name_length16;
UnicodeScalar *buffer;
integer first;
integer last;
integer max_buf_stack;
       /*:30*//*32: */
#ifdef INITEX
boolean ini_version;
boolean dump_option;
boolean dump_line;

#endif                          /* INITEX */
const_cstring dump_name;
unicode_file term_in;
integer bound_default;
const_cstring bound_name;
integer mem_bot;
integer main_memory;
integer extra_mem_bot;
integer mem_min;
integer mem_top;
integer extra_mem_top;
integer mem_max;
integer error_line;
integer half_error_line;
integer max_print_line;
integer max_strings;
integer strings_free;
integer string_vacancies;
integer pool_size;
integer pool_free;
integer font_mem_size;
integer font_max;
integer font_k;
integer hyph_size;
integer trie_size;
integer buf_size;
integer stack_size;
integer max_in_open;
integer param_size;
integer nest_size;
integer save_size;
integer dvi_buf_size;
integer expand_depth;
cinttype parse_first_line_p;
cinttype file_line_error_style_p;
cinttype eight_bit_p;
cinttype halt_on_error_p;
boolean quoted_filename;
boolean src_specials_p;
boolean insert_src_special_auto;
boolean insert_src_special_every_par;
boolean insert_src_special_every_parend;
boolean insert_src_special_every_cr;
boolean insert_src_special_every_math;
boolean insert_src_special_every_hbox;
boolean insert_src_special_every_vbox;
boolean insert_src_special_every_display;
packed_UTF16_code *str_pool;
pool_pointer *str_start;
pool_pointer pool_ptr;
str_number str_ptr;
pool_pointer init_pool_ptr;
str_number init_str_ptr;
       /*:39*//*50: */
#ifdef INITEX
alpha_file pool_file;

#endif                          /* INITEX */
alpha_file log_file;
unsigned char /*max_selector */ selector;
unsigned char dig[23];
integer tally;
integer term_offset;
integer file_offset;
UTF16_code trick_buf[256];
integer trick_count;
integer first_count;
boolean doing_special;
UTF16_code *native_text;
integer native_text_size;
integer native_len;
integer save_native_len;
unsigned char /*error_stop_mode */ interaction;
unsigned char /*unspecified_mode */ interaction_option;
boolean deletions_allowed;
boolean set_box_allowed;
unsigned char /*fatal_error_stop */ history;
schar error_count;
str_number help_line[6];
unsigned char help_ptr;
boolean use_err_help;
integer interrupt;
boolean OK_to_interrupt;
boolean arith_error;
scaled tex_remainder;
halfword temp_ptr;
memory_word *yzmem;
memory_word *zmem;
halfword lo_mem_max;
halfword hi_mem_min;
integer var_used, dyn_used;
halfword avail;
halfword mem_end;
halfword rover;
halfword last_leftmost_char;
halfword last_rightmost_char;
halfword hlist_stack[513];
short /*max_hlist_stack */ hlist_stack_level;
halfword first_p;
halfword global_prev_p;
        /*:163*//*172: */
#ifdef TEXMF_DEBUG
boolean free_arr[10];
boolean was_free[10];
halfword was_mem_end, was_lo_max, was_hi_min;
boolean panicking;

#endif                          /* TEXMF_DEBUG */
integer font_in_short_display;
integer depth_threshold;
integer breadth_max;
list_state_record *nest;
integer nest_ptr;
integer max_nest_stack;
list_state_record cur_list;
short /*mmode */ shown_mode;
unsigned char /*max_selector */ old_setting;
memory_word *zeqtb;
quarterword
#define xeq_level (zzzaa -8938740)
    zzzaa[1114731];
two_halves *hash;
two_halves *yhash;
halfword hash_used;
halfword hash_extra;
halfword hash_top;
halfword eqtb_top;
halfword hash_high;
boolean no_new_control_sequence;
integer cs_count;
two_halves prim[501];
halfword prim_used;
memory_word prim_eqtb[501];
memory_word *save_stack;
integer save_ptr;
integer max_save_stack;
quarterword cur_level;
group_code cur_group;
integer cur_boundary;
integer mag_set;
eight_bits cur_cmd;
halfword cur_chr;
halfword cur_cs;
halfword cur_tok;
in_state_record *input_stack;
integer input_ptr;
integer max_in_stack;
in_state_record cur_input;
integer in_open;
integer open_parens;
unicode_file *input_file;
integer line;
integer *line_stack;
str_number *source_filename_stack;
str_number *full_source_filename_stack;
unsigned char /*absorbing */ scanner_status;
halfword warning_index;
halfword def_ref;
halfword *param_stack;
integer param_ptr;
integer max_param_stack;
integer align_state;
integer base_ptr;
halfword par_loc;
halfword par_token;
boolean force_eof;
integer expand_depth_count;
boolean is_in_csname;
halfword cur_mark[5];
unsigned char /*long_outer_call */ long_state;
halfword pstack[9];
integer cur_val;
integer cur_val1;
unsigned char /*tok_val */ cur_val_level;
small_number radix;
glue_ord cur_order;
unicode_file read_file[16];
unsigned char /*closed */ read_open[17];
halfword cond_ptr;
unsigned char /*or_code */ if_limit;
small_number cur_if;
integer if_line;
integer skip_line;
str_number cur_name;
str_number cur_area;
str_number cur_ext;
pool_pointer area_delimiter;
pool_pointer ext_delimiter;
UTF16_code file_name_quote_char;
integer format_default_length;
cstring TEX_format_default;
boolean name_in_progress;
str_number job_name;
boolean log_opened;
str_number output_file_extension;
boolean no_pdf_output;
byte_file dvi_file;
str_number output_file_name;
str_number texmf_log_name;
byte_file tfm_file;
fmemory_word *font_info;
font_index fmem_ptr;
internal_font_number font_ptr;
four_quarters *font_check;
scaled *font_size;
scaled *font_dsize;
font_index *font_params;
str_number *font_name;
str_number *font_area;
UTF16_code *font_bc;
UTF16_code *font_ec;
halfword *font_glue;
boolean *font_used;
integer *hyphen_char;
integer *skew_char;
font_index *bchar_label;
nine_bits *font_bchar;
nine_bits *font_false_bchar;
void_pointer *font_layout_engine;
void_pointer *font_mapping;
char *font_flags;
scaled *font_letter_space;
void_pointer loaded_font_mapping;
char loaded_font_flags;
scaled loaded_font_letter_space;
scaled loaded_font_design_size;
UTF16_code *mapped_text;
char *xdv_buffer;
integer *char_base;
integer *width_base;
integer *height_base;
integer *depth_base;
integer *italic_base;
integer *lig_kern_base;
integer *kern_base;
integer *exten_base;
integer *param_base;
four_quarters null_character;
integer total_pages;
scaled max_v;
scaled max_h;
integer max_push;
integer last_bop;
integer dead_cycles;
boolean doing_leaders;
quarterword c;
internal_font_number f;
scaled rule_ht, rule_dp, rule_wd;
halfword g;
integer lq, lr;
eight_bits *dvi_buf;
integer half_buf;
integer dvi_limit;
integer dvi_ptr;
integer dvi_offset;
integer dvi_gone;
halfword down_ptr, right_ptr;
scaled dvi_h, dvi_v;
scaled cur_h, cur_v;
internal_font_number dvi_f;
integer cur_s;
scaled total_stretch[4], total_shrink[4];
integer last_badness;
halfword adjust_tail;
halfword pre_adjust_tail;
integer pack_begin_line;
two_halves empty_field;
four_quarters null_delimiter;
halfword cur_mlist;
small_number cur_style;
integer cur_size;
scaled cur_mu;
boolean mlist_penalties;
internal_font_number cur_f;
integer cur_c;
four_quarters cur_i;
integer magic_offset;
halfword cur_align;
halfword cur_span;
halfword cur_loop;
halfword align_ptr;
halfword cur_head, cur_tail;
halfword cur_pre_head, cur_pre_tail;
halfword just_box;
halfword passive;
halfword printed_node;
halfword pass_number;
scaled active_width[7];
scaled cur_active_width[7];
scaled background[7];
scaled break_width[7];
boolean no_shrink_error_yet;
halfword cur_p;
boolean second_pass;
boolean final_pass;
integer threshold;
integer minimal_demerits[4];
integer minimum_demerits;
halfword best_place[4];
halfword best_pl_line[4];
scaled disc_width;
halfword easy_line;
halfword last_special_line;
scaled first_width;
scaled second_width;
scaled first_indent;
scaled second_indent;
halfword best_bet;
integer fewest_demerits;
halfword best_line;
integer actual_looseness;
integer line_diff;
integer /*number_usvs */ hc[4099];
small_number hn;
halfword ha, hb;
internal_font_number hf;
integer /*too_big_char */ hu[4097];
integer hyf_char;
unsigned char /*biggest_lang */ cur_lang, init_cur_lang;
integer l_hyf, r_hyf, init_l_hyf, init_r_hyf;
halfword hyf_bchar;
integer max_hyph_char;
unsigned char hyf[4097];
halfword init_list;
boolean init_lig;
boolean init_lft;
small_number hyphen_passed;
halfword cur_l, cur_r;
halfword cur_q;
halfword lig_stack;
boolean ligature_present;
boolean lft_hit, rt_hit;
trie_pointer *trie_trl;
trie_pointer *trie_tro;
quarterword *trie_trc;
small_number hyf_distance[trie_op_size + 1];
small_number hyf_num[trie_op_size + 1];
trie_opcode hyf_next[trie_op_size + 1];
integer op_start[256];
str_number *hyph_word;
halfword *hyph_list;
hyph_pointer *hyph_link;
integer hyph_count;
integer hyph_next;
        /*:961*//*978: */
#ifdef INITEX
integer
#define trie_op_hash (zzzab - (int)(neg_trie_op_size))
    zzzab[trie_op_size - neg_trie_op_size + 1];
trie_opcode trie_used[256];
unsigned char /*biggest_lang */ trie_op_lang[trie_op_size + 1];
trie_opcode trie_op_val[trie_op_size + 1];
integer trie_op_ptr;

#endif                          /* INITEX */
trie_opcode max_op_used;
boolean small_op;
        /*:978*//*982: */
#ifdef INITEX
packed_UTF16_code *trie_c;
trie_opcode *trie_o;
trie_pointer *trie_l;
trie_pointer *trie_r;
trie_pointer trie_ptr;
trie_pointer *trie_hash;

#endif                          /* INITEX */
        /*:982*//*985: */
#ifdef INITEX
boolean *trie_taken;
trie_pointer trie_min[65536];
trie_pointer trie_max;
boolean trie_not_ready;

#endif                          /* INITEX */
scaled best_height_plus_depth;
halfword page_tail;
unsigned char /*box_there */ page_contents;
scaled page_max_depth;
halfword best_page_break;
integer least_page_cost;
scaled best_size;
scaled page_so_far[8];
halfword last_glue;
integer last_penalty;
scaled last_kern;
integer last_node_type;
integer insert_penalties;
boolean output_active;
internal_font_number main_f;
four_quarters main_i;
four_quarters main_j;
font_index main_k;
halfword main_p;
halfword main_pp, main_ppp;
halfword main_h;
boolean is_hyph;
integer space_class;
integer prev_class;
integer main_s;
halfword bchar;
halfword false_bchar;
boolean cancel_boundary;
boolean ins_disc;
halfword cur_box;
halfword after_token;
boolean long_help_seen;
str_number format_ident;
word_file fmt_file;
integer ready_already;
alpha_file write_file[16];
boolean write_open[18];
halfword write_loc;
scaled cur_page_width;
scaled cur_page_height;
scaled cur_h_offset;
scaled cur_v_offset;
integer pdf_last_x_pos;
integer pdf_last_y_pos;
unsigned char eTeX_mode;
boolean etex_p;
boolean *eof_seen;
halfword LR_ptr;
integer LR_problems;
small_number cur_dir;
halfword pseudo_files;
save_pointer *grp_stack;
halfword *if_stack;
halfword max_reg_num;
str_number max_reg_help_line;
halfword sa_root[8];
halfword cur_ptr;
memory_word sa_null;
halfword sa_chain;
quarterword sa_level;
halfword last_line_fill;
boolean do_last_line_fit;
small_number active_node_size;
scaled fill_width[3];
scaled best_pl_short[4];
scaled best_pl_glue[4];
trie_pointer hyph_start;
trie_pointer hyph_index;
halfword disc_ptr[4];
pool_pointer edit_name_start;
integer edit_name_length, edit_line;
cinttype ipc_on;
boolean stop_at_space;
str_number save_str_ptr;
pool_pointer save_pool_ptr;
cinttype shellenabledp;
cinttype restrictedshell;
char *output_comment;
unsigned char k, l;
boolean debug_format_file;
boolean mltex_p;
boolean mltex_enabled_p;
integer native_font_type_flag;
boolean xtx_ligature_present;
integer accent_c, base_c, replace_c;
four_quarters ia_c, ib_c;
real base_slant, accent_slant;
scaled base_x_height;
scaled base_width, base_height;
scaled accent_width, accent_height;
scaled delta;
integer synctexoption;
integer synctexoffset;
/*:1683*/
#include "xetexcoerce.h"
