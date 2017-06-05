/* tectonic/xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016 The Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEXD_H
#define TECTONIC_XETEXD_H

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/XeTeXOTMath.h>
#include <tectonic/TECkit_Common.h>
#include <tectonic/XeTeX_ext.h>
#include <tectonic/core-bridge.h>

#ifdef XETEX_MAC
/* include this here to avoid conflict between clang's emmintrin.h and
 * texmfmem.h. Should be removed once a fixed clang is widely available
 * http://llvm.org/bugs/show_bug.cgi?id=14964 */
#include <ApplicationServices/ApplicationServices.h>
#endif

#define odd(x) ((x) & 1)

/* Extra stuff used in various change files for various reasons.  */

/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
#define xmalloc_array(type, size) ((type*) xmalloc((size + 1) * sizeof(type)))
#define xcalloc_array(type, size) ((type*) xcalloc(size + 1, sizeof(type)))
#define xrealloc_array(ptr, type, size) ((type*) xrealloc(ptr, (size + 1) * sizeof(type)))

/* We use this rather than a simple fputs so that the string will end up
   in the .log file, too.  */
#define print_c_string(STR)      \
  do {                           \
    const_string ch_ptr = (STR); \
    while (*ch_ptr)              \
      print_char(*(ch_ptr++));   \
  } while (0)

/* Declarations for the routines we provide ourselves in lib/.  */

#ifndef PRIdPTR
#define PRIdPTR "ld"
#endif
#ifndef PRIxPTR
#define PRIxPTR "lx"
#endif

/*11:*/
#define trie_op_size 35111L
#define neg_trie_op_size -35111L
#define min_trie_op 0
#define max_trie_op 65535L

/*18: */
typedef unsigned short UTF16_code;
typedef unsigned char UTF8_code;
typedef integer UnicodeScalar;
typedef unsigned char eight_bits;
typedef integer pool_pointer;
typedef integer str_number;
typedef unsigned short packed_UTF16_code;
typedef short small_number;
typedef unsigned char two_choices;
typedef unsigned char four_choices;

/* texmfmem.h: the memory_word type, which is too hard to translate
   automatically from Pascal.  We have to make sure the byte-swapping
   that the (un)dumping routines do suffices to put things in the right
   place in memory.

   A memory_word can be broken up into a `two_halves' or a
   `four_quarters', and a `two_halves' can be further broken up.  Here is
   a picture.  ..._M = most significant byte, ..._L = least significant
   byte.

   The halfword fields are four bytes if we are building a big TeX or MF;
   this leads to further complications:

   BigEndian:
   two_halves.v:  RH_MM RH_ML RH_LM RH_LL LH_MM LH_ML LH_LM LH_LL
   two_halves.u:  ---------JUNK----------  B0         B1
   four_quarters:   B0    B1    B2    B3

   LittleEndian:
   two_halves.v:  LH_LL LH_LM LH_ML LH_MM RH_LL RH_LM RH_ML RH_MM
   two_halves.u:  B1          B0
   four_quarters: ---------JUNK----------  B3    B2    B1    B0

   I guess TeX and Metafont never refer to the B1 and B0 in the
   four_quarters structure as the B1 and B0 in the two_halves.u structure.

   The B0 and B1 fields are declared short instead of quarterword,
   because they are used in character nodes to store a font number and a
   character.  If left as a quarterword (which is a single byte), we
   couldn't support more than 256 fonts. (If shorts aren't two bytes,
   this will lose.)

   In the old four-byte memory structure (something more needs to be
   done to handle >256 fonts):

   If BigEndian:
   two_halves.v:  RH_M  RH_L  LH_M  LH_L
   two_halves.u:  JNK1  JNK2    B0    B1
   four_quarters:   B0    B1    B2    B3

   If LittleEndian:
   two_halves.v:  LH_L  LH_M  RH_L  RH_M
   two_halves.u:    B1    B0  JNK1  JNK2
   four_quarters:   B3    B2    B1    B0

   In Aleph, quarterwords are two octets, so the picture becomes simpler:

   BigEndian:
   two_halves.v:  RH_MM RH_ML RH_LM RH_LL LH_MM LH_ML LH_LM LH_LL
   two_halves.u:  ---------JUNK---------- ----B0----- ----B1-----
   four_quarters: ----B0----- ----B1----- ----B2----- ----B3-----
   twoints:       ---------CINT0--------- ---------CINT1---------

   LittleEndian:
   two_halves.v:  LH_LL LH_LM LH_ML LH_MM RH_LL RH_LM RH_ML RH_MM
   two_halves.u:  ----B1----- ----B0-----
   four_quarters: ----B3----- ----B2----- ----B1----- ----B0-----
   twoints:       ---------CINT1--------- ---------CINT0---------

   This file can't be part of texmf.h, because texmf.h gets included by
   {tex,mf,mp}d.h before the `halfword' etc. types are defined.  So we
   include it from the change file instead.
*/

typedef union
{
  struct
  {
#ifdef WORDS_BIGENDIAN
    int32_t RH, LH;
#else
    int32_t LH, RH;
#endif
  } v;

  struct
  { /* Make B0,B1 overlap the most significant bytes of LH.  */
#ifdef WORDS_BIGENDIAN
    int32_t junk;
    short B0, B1;
#else /* not WORDS_BIGENDIAN */
    short B1, B0;
#endif /* LittleEndian */
  } u;
} two_halves;


typedef struct
{
  struct
  {
#ifdef WORDS_BIGENDIAN
    uint16_t B0, B1, B2, B3;
#else
    uint16_t B3, B2, B1, B0;
#endif
  } u;
} four_quarters;

typedef union
{
  double gr;
  two_halves hh;
  void *ptr;
#ifdef WORDS_BIGENDIAN
  integer cint;
  four_quarters qqqq;
#else /* not WORDS_BIGENDIAN */
  struct
  {
    int32_t junk;
    integer CINT;
  } u;

  struct
  {
    four_quarters QQQQ;
  } v;
#endif /* not WORDS_BIGENDIAN */
} memory_word;


/* fmemory_word for font_list; needs to be only four bytes.  This saves
   significant space in the .fmt files. (Not true in XeTeX, actually!) */
typedef union
{
#ifdef WORDS_BIGENDIAN
  integer cint;
  four_quarters qqqq;
#else /* not WORDS_BIGENDIAN */
  struct
  {
    int32_t junk; /* quarterword is really 16 bits in XeTeX, so integer does not fill the union */
    integer CINT;
  } u;

  struct
  {
    four_quarters QQQQ;
  } v;
#endif /* not WORDS_BIGENDIAN */
} fmemory_word;


#ifndef WORDS_BIGENDIAN
#define cint u.CINT
#define qqqq v.QQQQ
#endif

/* end of former texmfmem.h */

typedef unsigned char glue_ord; /* enum: normal .. filll */
typedef unsigned char group_code;
typedef integer internal_font_number;
typedef integer font_index;
typedef integer nine_bits; /* range: 0 .. 0x1FF */
typedef integer dvi_index;
typedef integer trie_pointer;
typedef unsigned short trie_opcode;
typedef unsigned short hyph_pointer;
typedef integer save_pointer;

typedef struct {
    short mode; /* which mode we are: horz, vert, etc. */
    int32_t head; /* pointer to head of list being built */
    int32_t tail; /* pointer to tail of list being built */
    int32_t eTeX_aux; /* LR_save or LR_box or delim_ptr */
    integer pg; /* sometimes prev_graf: number of lines that have already been put into the current vlist */
    integer ml; /* mode_line: source line number at which this level was entered */
    memory_word aux; /* prev_depth or space_factor/clang or incompleat_noad */
} list_state_record;

typedef struct {
    uint16_t state; /* tokenizer state: mid_line, skip_blanks, new_line */
    uint16_t index; /* index of this level of input in input_file array */
    int32_t start; /* position of beginning of current line in `buffer` */
    int32_t loc; /* position of next character to read in `buffer` */
    int32_t limit; /* position of end of line in `buffer` */
    int32_t name; /* string number: name of current file or magic value for terminal, etc. */
    integer synctex_tag;
} input_state_t;

/* Functions originating in texmfmp.c */

extern void getmd5sum(integer s, int file);
extern boolean input_line (UFILE *);
extern void get_date_and_time (integer *, integer *, integer *, integer *);

extern str_number get_job_name(str_number);
extern str_number make_full_name_string(void);
extern string gettexstring(str_number);
extern boolean is_new_source(str_number, int);
extern pool_pointer make_src_special(str_number, int);
extern void remember_source_info(str_number, int);

/* variables! */

memory_word *the_eqtb;
#define CACHE_THE_EQTB register memory_word *eqtb = the_eqtb

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
boolean in_initex_mode;
boolean dump_line;
const_string dump_name;
integer bound_default;
const_string bound_name;
integer mem_top;
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
int file_line_error_style_p;
int halt_on_error_p;
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
rust_output_handle_t rust_stdout;
rust_output_handle_t log_file;
selector_t selector;
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
unsigned char interaction;
boolean deletions_allowed;
boolean set_box_allowed;
tt_history_t history;
signed char error_count;
str_number help_line[6];
unsigned char help_ptr;
boolean use_err_help;
boolean arith_error;
scaled tex_remainder;
int32_t temp_ptr;
memory_word *yzmem;
memory_word *zmem;
int32_t lo_mem_max;
int32_t hi_mem_min;
integer var_used, dyn_used;
int32_t avail;
int32_t mem_end;
int32_t rover;
int32_t last_leftmost_char;
int32_t last_rightmost_char;
int32_t hlist_stack[513];
short hlist_stack_level;
int32_t first_p;
int32_t global_prev_p;
integer font_in_short_display;
integer depth_threshold;
integer breadth_max;
list_state_record *nest;
integer nest_ptr;
integer max_nest_stack;
list_state_record cur_list;
short shown_mode;
unsigned char old_setting;
two_halves *hash;
two_halves *yhash;
int32_t hash_used;
int32_t hash_extra;
int32_t hash_top;
int32_t eqtb_top;
int32_t hash_high;
boolean no_new_control_sequence;
integer cs_count;
two_halves prim[501];
int32_t prim_used;
memory_word prim_eqtb[501];
memory_word *save_stack;
integer save_ptr;
integer max_save_stack;
uint16_t cur_level;
group_code cur_group;
integer cur_boundary;
integer mag_set;
eight_bits cur_cmd;
int32_t cur_chr;
int32_t cur_cs;
int32_t cur_tok;
input_state_t *input_stack;
integer input_ptr;
integer max_in_stack;
input_state_t cur_input;
integer in_open;
integer open_parens;
UFILE **input_file;
integer line;
integer *line_stack;
str_number *source_filename_stack;
str_number *full_source_filename_stack;
unsigned char scanner_status;
int32_t warning_index;
int32_t def_ref;
int32_t *param_stack;
integer param_ptr;
integer max_param_stack;
integer align_state;
integer base_ptr;
int32_t par_loc;
int32_t par_token;
boolean force_eof;
integer expand_depth_count;
boolean is_in_csname;
int32_t cur_mark[5];
unsigned char long_state;
int32_t pstack[9];
integer cur_val;
integer cur_val1;
unsigned char cur_val_level;
small_number radix;
glue_ord cur_order;
UFILE *read_file[16];
unsigned char read_open[17];
int32_t cond_ptr;
unsigned char if_limit;
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
string TEX_format_default;
boolean name_in_progress;
str_number job_name;
boolean log_opened;
str_number output_file_extension;
rust_output_handle_t dvi_file;
str_number output_file_name;
str_number texmf_log_name;
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
int32_t *font_glue;
boolean *font_used;
integer *hyphen_char;
integer *skew_char;
font_index *bchar_label;
nine_bits *font_bchar;
nine_bits *font_false_bchar;
void **font_layout_engine;
void **font_mapping;
char *font_flags;
scaled *font_letter_space;
void *loaded_font_mapping;
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
uint16_t c;
internal_font_number f;
scaled rule_ht, rule_dp, rule_wd;
int32_t g;
integer lq, lr;
eight_bits *dvi_buf;
integer half_buf;
integer dvi_limit;
integer dvi_ptr;
integer dvi_offset;
integer dvi_gone;
int32_t down_ptr, right_ptr;
scaled dvi_h, dvi_v;
scaled cur_h, cur_v;
internal_font_number dvi_f;
integer cur_s;
scaled total_stretch[4], total_shrink[4];
integer last_badness;
int32_t adjust_tail;
int32_t pre_adjust_tail;
integer pack_begin_line;
two_halves empty;
four_quarters null_delimiter;
int32_t cur_mlist;
small_number cur_style;
integer cur_size;
scaled cur_mu;
boolean mlist_penalties;
internal_font_number cur_f;
integer cur_c;
four_quarters cur_i;
integer magic_offset;
int32_t cur_align;
int32_t cur_span;
int32_t cur_loop;
int32_t align_ptr;
int32_t cur_head, cur_tail;
int32_t cur_pre_head, cur_pre_tail;
int32_t just_box;
int32_t passive;
int32_t printed_node;
int32_t pass_number;
scaled active_width[7];
scaled cur_active_width[7];
scaled background[7];
scaled break_width[7];
boolean no_shrink_error_yet;
int32_t cur_p;
boolean second_pass;
boolean final_pass;
integer threshold;
integer minimal_demerits[4];
integer minimum_demerits;
int32_t best_place[4];
int32_t best_pl_line[4];
scaled disc_width;
int32_t easy_line;
int32_t last_special_line;
scaled first_width;
scaled second_width;
scaled first_indent;
scaled second_indent;
int32_t best_bet;
integer fewest_demerits;
int32_t best_line;
integer actual_looseness;
integer line_diff;
integer hc[4099];
small_number hn;
int32_t ha, hb;
internal_font_number hf;
integer hu[4097];
integer hyf_char;
unsigned char cur_lang, init_cur_lang;
integer l_hyf, r_hyf, init_l_hyf, init_r_hyf;
int32_t hyf_bchar;
integer max_hyph_char;
unsigned char hyf[4097];
int32_t init_list;
boolean init_lig;
boolean init_lft;
small_number hyphen_passed;
int32_t cur_l, cur_r;
int32_t cur_q;
int32_t lig_stack;
boolean ligature_present;
boolean lft_hit, rt_hit;
trie_pointer *trie_trl;
trie_pointer *trie_tro;
uint16_t *trie_trc;
small_number hyf_distance[trie_op_size + 1];
small_number hyf_num[trie_op_size + 1];
trie_opcode hyf_next[trie_op_size + 1];
integer op_start[256];
str_number *hyph_word;
int32_t *hyph_list;
hyph_pointer *hyph_link;
integer hyph_count;
integer hyph_next;
trie_opcode trie_used[256];
unsigned char trie_op_lang[trie_op_size + 1];
trie_opcode trie_op_val[trie_op_size + 1];
integer trie_op_ptr;
trie_opcode max_op_used;
boolean small_op;
packed_UTF16_code *trie_c;
trie_opcode *trie_o;
trie_pointer *trie_l;
trie_pointer *trie_r;
trie_pointer trie_ptr;
trie_pointer *trie_hash;
boolean *trie_taken;
trie_pointer trie_min[65536];
trie_pointer trie_max;
boolean trie_not_ready;
scaled best_height_plus_depth;
int32_t page_tail;
unsigned char page_contents;
scaled page_max_depth;
int32_t best_page_break;
integer least_page_cost;
scaled best_size;
scaled page_so_far[8];
int32_t last_glue;
integer last_penalty;
scaled last_kern;
integer last_node_type;
integer insert_penalties;
boolean output_active;
internal_font_number main_f;
four_quarters main_i;
four_quarters main_j;
font_index main_k;
int32_t main_p;
int32_t main_pp, main_ppp;
int32_t main_h;
boolean is_hyph;
integer space_class;
integer prev_class;
integer main_s;
int32_t bchar;
int32_t false_bchar;
boolean cancel_boundary;
boolean ins_disc;
int32_t cur_box;
int32_t after_token;
boolean long_help_seen;
str_number format_ident;
rust_output_handle_t write_file[16];
boolean write_open[18];
int32_t write_loc;
scaled cur_page_width;
scaled cur_page_height;
scaled cur_h_offset;
scaled cur_v_offset;
integer pdf_last_x_pos;
integer pdf_last_y_pos;
boolean *eof_seen;
int32_t LR_ptr;
integer LR_problems;
small_number cur_dir;
int32_t pseudo_files;
save_pointer *grp_stack;
int32_t *if_stack;
int32_t max_reg_num;
str_number max_reg_help_line;
int32_t sa_root[8];
int32_t cur_ptr;
memory_word sa_null;
int32_t sa_chain;
uint16_t sa_level;
int32_t last_line_fill;
boolean do_last_line_fit;
small_number active_node_size;
scaled fill_width[3];
scaled best_pl_short[4];
scaled best_pl_glue[4];
trie_pointer hyph_start;
trie_pointer hyph_index;
int32_t disc_ptr[4];
pool_pointer edit_name_start;
integer edit_name_length, edit_line;
boolean stop_at_space;
str_number save_str_ptr;
pool_pointer save_pool_ptr;
char *output_comment;
unsigned char k, l;
integer native_font_type_flag;
boolean xtx_ligature_present;
integer accent_c, base_c, replace_c;
four_quarters ia_c, ib_c;
double base_slant, accent_slant;
scaled base_x_height;
scaled base_width, base_height;
scaled accent_width, accent_height;
scaled delta;
int synctex_enabled;

/*:1683*/

/* It looks like these arrays are set up so that they can be safely indexed
 * with negative indices. The underlying arrays used to be named "zzzaa" and
 * "zzzbb". */

uint16_t _xeq_level_array[1114731];
#define xeq_level (_xeq_level_array - 8938740)

integer _trie_op_hash_array[trie_op_size - neg_trie_op_size + 1];
#define trie_op_hash (_trie_op_hash_array - (int) neg_trie_op_size)

/* the former xetexcoerce.h: */

void print_ln(void);
void print_raw_char(UTF16_code s, boolean incr_offset);
void print_char(integer s);
void print(integer s);
void print_nl(str_number s);
void print_esc(str_number s);
void print_int(integer n);
void print_cs(integer p);
void sprint_cs(int32_t p);
void print_file_name(integer n, integer a, integer e);
void print_size(integer s);
void print_write_whatsit(str_number s, int32_t p);
void print_native_word(int32_t p);
void print_sa_num(int32_t q);
void print_file_line(void);
void print_two(integer n);
void print_hex(integer n);
void print_roman_int(integer n);
void print_scaled(scaled s);

void error(void);
NORETURN void fatal_error(str_number s);
NORETURN void overflow(str_number s, integer n);
NORETURN void confusion(str_number s);
NORETURN void pdf_error(str_number t, str_number p);

integer length(str_number s);
str_number make_string(void);
void append_str(str_number s);
boolean str_eq_buf(str_number s, integer k);
boolean str_eq_str(str_number s, str_number t);
str_number search_string(str_number search);
str_number slow_make_string(void);
void print_current_string(void);
int32_t badness(scaled t, scaled s);
void print_word(memory_word w);
void show_token_list(integer p, integer q, integer l);
void runaway(void);
int32_t get_avail(void);
void flush_list(int32_t p);
int32_t get_node(integer s);
void free_node(int32_t p, int32_t s);
int32_t new_null_box(void);
int32_t new_rule(void);
int32_t new_ligature(internal_font_number f, uint16_t c, int32_t q);
int32_t new_lig_item(uint16_t c);
int32_t new_disc(void);
void copy_native_glyph_info(int32_t src, int32_t dest);
int32_t new_math(scaled w, small_number s);
int32_t new_spec(int32_t p);
int32_t new_param_glue(small_number n);
int32_t new_glue(int32_t q);
int32_t new_skip_param(small_number n);
int32_t new_kern(scaled w);
int32_t new_penalty(integer m);
void check_mem(boolean print_locs);
void search_mem(int32_t p);
int32_t prev_rightmost(int32_t s, int32_t e);
scaled round_xn_over_d(scaled x, integer n, integer d);
void short_display(integer p);
void print_font_and_char(integer p);
void print_mark(integer p);
void print_rule_dimen(scaled d);
void print_glue(scaled d, integer order, str_number s);
void print_spec(integer p, str_number s);
void print_fam_and_char(int32_t p);
void print_delimiter(int32_t p);
void print_subsidiary_data(int32_t p, UTF16_code c);
void print_style(integer c);
void print_skip_param(integer n);
void show_node_list(integer p);
void show_box(int32_t p);
void short_display_n(integer p, integer m);
void delete_token_ref(int32_t p);
void delete_glue_ref(int32_t p);
void flush_node_list(int32_t p);
int32_t copy_node_list(int32_t p);
void print_mode(integer m);
void print_in_mode(integer m);
void push_nest(void);
void pop_nest(void);
void show_activities(void);
void print_param(integer n);
void begin_diagnostic(void);
void end_diagnostic(boolean blank_line);
void print_length_param(integer n);
void print_cmd_chr(uint16_t cmd, int32_t chr_code);
void not_aat_font_error(integer cmd, integer c, integer f);
void not_aat_gr_font_error(integer cmd, integer c, integer f);
void not_ot_font_error(integer cmd, integer c, integer f);
void not_native_font_error(integer cmd, integer c, integer f);
void show_eqtb(int32_t n);
int32_t id_lookup(integer j, integer l);
int32_t prim_lookup(str_number s);
void restore_trace(int32_t p, str_number s);
void print_group(boolean e);
void group_trace(boolean e);
boolean pseudo_input(void);
void pseudo_close(void);
void group_warning(void);
void if_warning(void);
void file_warning(void);
void delete_sa_ref(int32_t q);
void show_sa(int32_t p, str_number s);
void sa_save(int32_t p);
void sa_destroy(int32_t p);
void sa_def(int32_t p, int32_t e);
void sa_w_def(int32_t p, integer w);
void gsa_def(int32_t p, int32_t e);
void gsa_w_def(int32_t p, integer w);
void sa_restore(void);
void new_save_level(group_code c);
void eq_destroy(memory_word w);
void eq_save(int32_t p, uint16_t l);
void eq_define(int32_t p, uint16_t t, int32_t e);
void eq_word_define(int32_t p, integer w);
void geq_define(int32_t p, uint16_t t, int32_t e);
void geq_word_define(int32_t p, integer w);
void save_for_after(int32_t t);
void unsave(void);
void prepare_mag(void);
void token_show(int32_t p);
void print_meaning(void);
void show_cur_cmd_chr(void);
void show_context(void);
void begin_token_list(int32_t p, uint16_t t);
void end_token_list(void);
void back_input(void);
void back_error(void);
void ins_error(void);
void begin_file_reading(void);
void end_file_reading(void);
void check_outer_validity(void);
void get_next(void);
void get_token(void);
void macro_call(void);
void insert_relax(void);
void new_index(uint16_t i, int32_t q);
void find_sa_element(small_number t, int32_t n, boolean w);
void expand(void);
void get_x_token(void);
void x_token(void);
void scan_left_brace(void);
void scan_optional_equals(void);
boolean scan_keyword(str_number s);
void mu_error(void);
void scan_glyph_number(internal_font_number f);
void scan_char_class(void);
void scan_char_class_not_ignored(void);
void scan_eight_bit_int(void);
void scan_usv_num(void);
void scan_char_num(void);
void scan_xetex_math_char_int(void);
void scan_math_class_int(void);
void scan_math_fam_int(void);
void scan_four_bit_int(void);
void scan_fifteen_bit_int(void);
void scan_delimiter_int(void);
void scan_register_num(void);
void scan_four_bit_int_or_18(void);
void get_x_or_protected(void);
integer effective_char(boolean err_p, internal_font_number f, uint16_t c);
void scan_font_ident(void);
void find_font_dimen(boolean writing);
void scan_something_internal(small_number level, boolean negative);
void scan_int(void);
void xetex_scan_dimen(boolean mu, boolean inf, boolean shortcut, boolean requires_units);
void scan_dimen(boolean mu, boolean inf, boolean shortcut);
void scan_decimal(void);
void scan_glue(small_number level);
integer add_or_sub(integer x, integer y, integer max_answer, boolean negative);
integer quotient(integer n, integer d);
integer fract(integer x, integer n, integer d, integer max_answer);
void scan_expr(void);
void scan_normal_glue(void);
void scan_mu_glue(void);
int32_t scan_rule_spec(void);
void scan_general_text(void);
void pseudo_start(void);
int32_t str_toks_cat(pool_pointer b, small_number cat);
int32_t str_toks(pool_pointer b);
int32_t the_toks(void);
void ins_the_toks(void);
void conv_toks(void);
int32_t scan_toks(boolean macro_def, boolean xpand);
void read_toks(integer n, int32_t r, int32_t j);
void pass_text(void);
void change_if_limit(small_number l, int32_t p);
void conditional(void);
void begin_name(void);
boolean more_name(UTF16_code c);
void end_name(void);
void pack_file_name(str_number n, str_number a, str_number e);
str_number make_name_string(void);
void scan_file_name(void);
void pack_job_name(str_number s);
void open_log_file(void);
void start_input(void);
four_quarters effective_char_info(internal_font_number f, uint16_t c);
void char_warning(internal_font_number f, integer c);
int32_t new_native_word_node(internal_font_number f, integer n);
int32_t new_native_character(internal_font_number f, UnicodeScalar c);
void font_feature_warning(void *featureNameP, integer featLen, void *settingNameP, integer setLen);
void font_mapping_warning(void *mappingNameP, integer mappingNameLen, integer warningType);
void graphite_warning(void);
internal_font_number load_native_font(int32_t u, str_number nom, str_number aire, scaled s);
void do_locale_linebreaks(integer s, integer len);
void bad_utf8_warning(void);
integer get_input_normalization_state(void);
integer get_tracing_fonts_state(void);
internal_font_number read_font_info(int32_t u, str_number nom, str_number aire, scaled s);
int32_t new_character(internal_font_number f, UTF16_code c);
void dvi_swap(void);
void dvi_four(integer x);
void dvi_two(UTF16_code s);
void dvi_pop(integer l);
void dvi_native_font_def(internal_font_number f);
void dvi_font_def(internal_font_number f);
void movement(scaled w, eight_bits o);
void prune_movements(integer l);
void special_out(int32_t p);
void write_out(int32_t p);
void pic_out(int32_t p);
void out_what(int32_t p);
int32_t new_edge(small_number s, scaled w);
int32_t reverse(int32_t this_box, int32_t t, scaled * cur_g, double * cur_glue);
void hlist_out(void);
void vlist_out(void);
void ship_out(int32_t p);
void scan_spec(group_code c, boolean three_codes);
scaled char_pw(int32_t p, small_number side);
int32_t new_margin_kern(scaled w, int32_t p, small_number side);
int32_t hpack(int32_t p, scaled w, small_number m);
int32_t vpackage(int32_t p, scaled h, small_number m, scaled l);
void append_to_vlist(int32_t b);
int32_t new_noad(void);
int32_t new_style(small_number s);
int32_t new_choice(void);
void show_info(void);
scaled math_x_height(integer size_code);
scaled math_quad(integer size_code);
scaled num1(integer size_code);
scaled num2(integer size_code);
scaled num3(integer size_code);
scaled denom1(integer size_code);
scaled denom2(integer size_code);
scaled sup1(integer size_code);
scaled sup2(integer size_code);
scaled sup3(integer size_code);
scaled sub1(integer size_code);
scaled sub2(integer size_code);
scaled sup_drop(integer size_code);
scaled sub_drop(integer size_code);
scaled delim1(integer size_code);
scaled delim2(integer size_code);
scaled axis_height(integer size_code);
scaled default_rule_thickness(void);
scaled big_op_spacing1(void);
scaled big_op_spacing2(void);
scaled big_op_spacing3(void);
scaled big_op_spacing4(void);
scaled big_op_spacing5(void);
int32_t fraction_rule(scaled t);
int32_t overbar(int32_t b, scaled k, scaled t);
int32_t char_box(internal_font_number f, integer c);
void stack_into_box(int32_t b, internal_font_number f, uint16_t c);
scaled height_plus_depth(internal_font_number f, uint16_t c);
void stack_glyph_into_box(int32_t b, internal_font_number f, integer g);
void stack_glue_into_box(int32_t b, scaled min, scaled max);
int32_t build_opentype_assembly(internal_font_number f, void *a, scaled s, boolean horiz);
int32_t var_delimiter(int32_t d, integer s, scaled v);
int32_t rebox(int32_t b, scaled w);
int32_t math_glue(int32_t g, scaled m);
void math_kern(int32_t p, scaled m);
void flush_math(void);
int32_t clean_box(int32_t p, small_number s);
void fetch(int32_t a);
void make_over(int32_t q);
void make_under(int32_t q);
void make_vcenter(int32_t q);
void make_radical(int32_t q);
scaled compute_ot_math_accent_pos(int32_t p);
void make_math_accent(int32_t q);
void make_fraction(int32_t q);
scaled make_op(int32_t q);
void make_ord(int32_t q);
int32_t attach_hkern_to_new_hlist(int32_t q, scaled delta);
void make_scripts(int32_t q, scaled delta);
small_number make_left_right(int32_t q, small_number style, scaled max_d, scaled max_h);
void mlist_to_hlist(void);
void push_alignment(void);
void pop_alignment(void);
void get_preamble_token(void);
void init_align(void);
void init_span(int32_t p);
void init_row(void);
void init_col(void);
boolean fin_col(void);
void fin_row(void);
void fin_align(void);
void align_peek(void);
int32_t finite_shrink(int32_t p);
void push_node(int32_t p);
int32_t pop_node(void);
int32_t find_protchar_left(int32_t l, boolean d);
int32_t find_protchar_right(int32_t l, int32_t r);
scaled total_pw(int32_t q, int32_t p);
void try_break(integer pi, small_number break_type);
void post_line_break(boolean d);
small_number reconstitute(small_number j, small_number n, int32_t bchar, int32_t hchar);
void hyphenate(void);
integer max_hyphenatable_length(void);
trie_opcode new_trie_op(small_number d, small_number n, trie_opcode v);
trie_pointer trie_node(trie_pointer p);
trie_pointer compress_trie(trie_pointer p);
void first_fit(trie_pointer p);
void trie_pack(trie_pointer p);
void trie_fix(trie_pointer p);
void init_trie(void);
void line_break(boolean d);
boolean eTeX_enabled(boolean b, uint16_t j, int32_t k);
void show_save_groups(void);
int32_t prune_page_top(int32_t p, boolean s);
int32_t vert_break(int32_t p, scaled h, scaled d);
boolean do_marks(small_number a, small_number l, int32_t q);
int32_t vsplit(int32_t n, scaled h);
void print_totals(void);
void freeze_page_specs(small_number s);
void box_error(eight_bits n);
void ensure_vbox(eight_bits n);
void fire_up(int32_t c);
void build_page(void);
void app_space(void);
void insert_dollar_sign(void);
void you_cant(void);
void report_illegal_case(void);
boolean privileged(void);
boolean its_all_over(void);
void append_glue(void);
void append_kern(void);
void off_save(void);
void extra_right_brace(void);
void normal_paragraph(void);
void box_end(integer box_context);
void begin_box(integer box_context);
void scan_box(integer box_context);
void package(small_number c);
small_number norm_min(integer h);
void new_graf(boolean indented);
void indent_in_hmode(void);
void head_for_vmode(void);
void end_graf(void);
void begin_insert_or_adjust(void);
void make_mark(void);
void append_penalty(void);
void delete_last(void);
void unpackage(void);
void append_italic_correction(void);
void append_discretionary(void);
void build_discretionary(void);
void make_accent(void);
void align_error(void);
void no_align_error(void);
void omit_error(void);
void do_endv(void);
void cs_error(void);
void push_math(group_code c);
void just_copy(int32_t p, int32_t h, int32_t t);
void just_reverse(int32_t p);
void init_math(void);
void start_eq_no(void);
void scan_math(int32_t p);
void set_math_char(integer c);
void math_limit_switch(void);
void scan_delimiter(int32_t p, boolean r);
void math_radical(void);
void math_ac(void);
void append_choices(void);
int32_t fin_mlist(int32_t p);
void build_choices(void);
void sub_sup(void);
void math_fraction(void);
void math_left_right(void);
void app_display(int32_t j, int32_t b, scaled d);
void after_math(void);
void resume_after_display(void);
void get_r_token(void);
void trap_zero_glue(void);
void do_register_command(small_number a);
void alter_aux(void);
void alter_prev_graf(void);
void alter_page_so_far(void);
void alter_integer(void);
void alter_box_dimen(void);
void new_font(small_number a);
void new_interaction(void);
void prefixed_command(void);
void do_assignments(void);
void open_or_close_in(void);
void issue_message(void);
void shift_case(void);
void show_whatever(void);
void new_whatsit(small_number s, small_number w);
void new_write_whatsit(small_number w);
void load_picture(boolean is_pdf);
void scan_and_pack_name(void);
void do_extension(void);
void fix_language(void);
void insert_src_special(void);
void append_src_special(void);
void handle_right_brace(void);
void main_control(void);
void give_err_help(void);
void close_files_and_terminate(void);
void debug_help(void);
void flush_str(str_number s);
str_number tokens_to_string(int32_t p);
void scan_pdf_ext_toks(void);
void compare_strings(void);


/* formerly xetex.h: */
/* additional declarations we want to slip in for xetex */

#define native_node_text(p) ((unsigned short*) &mem[(p) + NATIVE_NODE_SIZE])
#define get_native_char(p,i) native_node_text(p)[i]
#define set_native_char(p,i,v) native_node_text(p)[i] = v
#define get_native_usv(p,i) \
  ((native_node_text(p)[i] >= 0xd800 && native_node_text(p)[i] < 0xdc00) ? \
    0x10000 + (native_node_text(p)[i] - 0xd800) * 0x400 + native_node_text(p)[(i)+1] - 0xdc00 : \
    native_node_text(p)[i])

/* p is native_word node; g is XeTeX_use_glyph_metrics flag */
#define set_native_metrics(p,g)               measure_native_node(&(mem[p]), g)
#define set_native_glyph_metrics(p,g)         measure_native_glyph(&(mem[p]), g)
#define set_justified_native_glyphs(p)        store_justified_native_glyphs(&(mem[p]))
#define get_native_italic_correction(p)       real_get_native_italic_correction(&(mem[p]))
#define get_native_glyph_italic_correction(p) real_get_native_glyph_italic_correction(&(mem[p]))
#define get_native_glyph(p,i)                 real_get_native_glyph(&(mem[p]), i)
#define make_xdv_glyph_array_data(p)          makeXDVGlyphArrayData(&(mem[p]))
#define get_native_word_cp(p,s)               real_get_native_word_cp(&(mem[p]), s)

#define pic_path_byte(p,i) ((unsigned char*) &mem[(p) + PIC_NODE_SIZE])[i]

/* easier to do the bit-twiddling here than in Pascal */
/* read fields from a 32-bit math code */
#define math_fam(x)   (((unsigned)(x) >> 24) & 0xFF)
#define math_class(x) (((unsigned)(x) >> 21) & 0x07)
#define math_char(x)  ((unsigned)(x) & 0x1FFFFF)
/* calculate pieces to assign to a math code */
#define set_family(x) (((unsigned)(x) & 0xFF) << 24)
#define set_class(x)  (((unsigned)(x) & 0x07) << 21)

/* Unicode file reading modes */
#define AUTO       0 /* default: will become one of 1..3 at file open time, after sniffing */
#define UTF8       1
#define UTF16BE    2
#define UTF16LE    3
#define RAW        4
#define ICUMAPPING 5

#endif /* TECTONIC_XETEXD_H */
