/* tectonic/xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEXD_H
#define TECTONIC_XETEXD_H

#include "tectonic.h"
#include "internals.h"
#include "XeTeXOTMath.h"
#include "TECkit_Common.h"
#include "XeTeX_ext.h"
#include "core-bridge.h"

#ifdef XETEX_MAC
/* include this here to avoid conflict between clang's emmintrin.h and
 * texmfmem.h. Should be removed once a fixed clang is widely available
 * http://llvm.org/bugs/show_bug.cgi?id=14964 */
#include <ApplicationServices/ApplicationServices.h>
#endif

#define odd(x) ((x) & 1)

/* Extra stuff used in various change files for various reasons.  */

/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
#define xmalloc_array(type, size) (xmalloc((size + 1) * sizeof(type)))
#define xcalloc_array(type, size) (xcalloc(size + 1, sizeof(type)))
#define xrealloc_array(ptr, type, size) ((type*) xrealloc(ptr, (size + 1) * sizeof(type)))

/* We use this rather than a simple fputs so that the string will end up
   in the .log file, too.  */
#define print_c_string(STR)      \
  do {                           \
    const char *ch_ptr = (STR); \
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

/* The annoying `memory_word` type. We have to make sure the byte-swapping
 * that the (un)dumping routines do suffices to put things in the right place
 * in memory.
 *
 * This set of data used to be a huge mess (see comment after the
 * definitions). It is now (IMO) a lot more reasonable, but there will no
 * doubt be carryover weird terminology around the code.
 *
 * ## ENDIANNESS (cheat sheet because I'm lame)
 *
 * Intel is little-endian. Say that we have a 32-bit integer stored in memory
 * with `p` being a `uint8` pointer to its location. In little-endian land,
 * `p[0]` is least significant byte and `p[3]` is its most significant byte.
 *
 * Conversely, in big-endian land, `p[0]` is its most significant byte and
 * `p[3]` is its least significant byte.
 *
 * ## MEMORY_WORD LAYOUT
 *
 * Little endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [lsb......s0.......msb] [lsb......s1.......msb]
 *   b16:   [l..s0...m] [l..s1...m] [l..s2...m] [l..s3...m]
 *
 * Big endian:
 *
 *   bytes: --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   b32:   [msb......s1.......lsb] [msb......s0.......lsb]
 *   b16:   [m..s3...l] [m..s2...l] [m..s1...l] [m...s0..l]
 *
 */

#ifdef WORDS_BIGENDIAN

typedef struct b32x2_be_t { int32_t s1, s0; } b32x2;
typedef struct b16x4_be_t { uint16_t s3, s2, s1, s0; } b16x4;

#else

typedef struct b32x2_le_t { int32_t s0, s1; } b32x2;
typedef struct b16x4_le_t { uint16_t s0, s1, s2, s3; } b16x4;

#endif /*WORDS_BIGENDIAN*/

typedef union {
    b32x2 b32;
    b16x4 b16;
    double gr;
    void *ptr;
} memory_word;

/* ## THE ORIGINAL SITUATION (archived for posterity)
 *
 * In XeTeX, a "quarterword" is 16 bits. Who knows why. A "halfword" is,
 * sensibly, 32 bits. A "memory word" is a full word: either four quarters or
 * two halves: i.e., 64 bits. The memory word union also has options for
 * doubles (called `gr`), `integer` which is an int32_t (called `cint`), and a
 * pointer (`ptr`).
 *
 * Original struct definition, LITTLE ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { int32_t LH, RH; } v;
 *       struct { short B1, B0; } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { uint16_t B3, B2, B1, B0; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *
 *       struct {
 *           int32_t junk;
 *           integer CINT;
 *       } u;
 *
 *       struct {
 *           four_quarters QQQQ;
 *       } v;
 *   } memory_word;
 *
 *   #  define cint u.CINT
 *   #  define qqqq v.QQQQ
 *
 * Original memory layout, LITTLE ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:                             [lsb...............msb]
 *   hh.u:     [l..B1...m] [l..B0...m]
 *   hh.v:     [lsb......LH.......msb] [lsb......RH.......msb]
 *   quarters: [l..B3...m] [l..B2...m] [l..B1...m] [l..B0...m]
 *
 * Original struct definition, BIG ENDIAN (condensed):
 *
 *   typedef union {
 *       struct { int32_t RH, LH; } v;
 *       struct {
 *           int32_t junk;
 *           short B0, B1;
 *       } u;
 *   } two_halves;
 *
 *   typedef struct {
 *       struct { uint16_t B0, B1, B2, B3; } u;
 *   } four_quarters;
 *
 *   typedef union {
 *       two_halves hh;
 *       four_quarters qqqq;
 *   } memory_word;
 *
 * Original memory layout, BIG ENDIAN:
 *
 *   bytes:    --0-- --1-- --2-- --3-- --4-- --5-- --6-- --7--
 *   cint:     [msb...............lsb]
 *   hh.u:                             [m..B0...l] [m..B1...l]
 *   hh.v:     [msb......RH.......lsb] [msb......LH.......lsb]
 *   quarters: [m..B0...l] [m..B1...l] [m..B2...l] [m...B3..l]
 *
 * Several things to note that apply to both endiannesses:
 *
 *   1. The different B0 and B1 instances do not line up.
 *   2. `cint` is isomorphic to `hh.v.RH`
 *   3. `hh.u.B0` is isomorphic to `qqqq.u.B2`
 *   4. `hh.u.B1` is isomorphic to `qqqq.u.B3`.
 *   5. The `four_quarters` field `u` serves no discernable purpose.
 *
 * CONVERTING TO THE NEW SYSTEM
 *
 * - `w.cint` => `w.b32.s1`
 * - `w.qqqq.u.B<n>` => `w.b16.s{{3 - <n>}}` !!!!!!!!!!!
 * - similar for `<quarterword_variable>.u.B<n>` => `<quarterword_variable>.s{{3 - <n>}}` !!!
 * - `w.hh.u.B0` => `w.b16.s1`
 * - `w.hh.u.B1` => `w.b16.s0`
 * - `w.hh.v.RH` => `w.b32.s1`
 * - `w.hh.v.LH` => `w.b32.s0`
 * - `four_quarters` => `b16x4`
 * - `two_halves` => `b32x2`
 *
 */


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

void getmd5sum(integer s, bool file);
void get_date_and_time (integer *, integer *, integer *, integer *);

str_number make_full_name_string(void);
char *gettexstring(str_number);
bool is_new_source(str_number, int);
pool_pointer make_src_special(str_number, int);
void remember_source_info(str_number, int);

/* variables! */

/* All the following variables are defined in xetexini.c */
extern memory_word *the_eqtb;
#define CACHE_THE_EQTB register memory_word *eqtb = the_eqtb

extern integer bad;
extern UTF8_code *name_of_file;
extern UTF16_code *name_of_file16;
extern integer name_length;
extern integer name_length16;
extern UnicodeScalar *buffer;
extern integer first;
extern integer last;
extern integer max_buf_stack;
extern bool in_initex_mode;
extern integer error_line;
extern integer half_error_line;
extern integer max_print_line;
extern integer max_strings;
extern integer strings_free;
extern integer string_vacancies;
extern integer pool_size;
extern integer pool_free;
extern integer font_mem_size;
extern integer font_max;
extern integer font_k;
extern integer hyph_size;
extern integer trie_size;
extern integer buf_size;
extern integer stack_size;
extern integer max_in_open;
extern integer param_size;
extern integer nest_size;
extern integer save_size;
extern integer dvi_buf_size;
extern integer expand_depth;
extern int file_line_error_style_p;
extern int halt_on_error_p;
extern bool quoted_filename;
extern bool insert_src_special_auto;
extern bool insert_src_special_every_par;
extern bool insert_src_special_every_math;
extern bool insert_src_special_every_vbox;
extern packed_UTF16_code *str_pool;
extern pool_pointer *str_start;
extern pool_pointer pool_ptr;
extern str_number str_ptr;
extern pool_pointer init_pool_ptr;
extern str_number init_str_ptr;
extern rust_output_handle_t rust_stdout;
extern rust_output_handle_t log_file;
extern selector_t selector;
extern unsigned char dig[23];
extern integer tally;
extern integer term_offset;
extern integer file_offset;
extern UTF16_code trick_buf[256];
extern integer trick_count;
extern integer first_count;
extern bool doing_special;
extern UTF16_code *native_text;
extern integer native_text_size;
extern integer native_len;
extern integer save_native_len;
extern unsigned char interaction;
extern bool deletions_allowed;
extern bool set_box_allowed;
extern tt_history_t history;
extern signed char error_count;
extern const char* help_line[6];
extern unsigned char help_ptr;
extern bool use_err_help;
extern bool arith_error;
extern scaled tex_remainder;
extern int32_t temp_ptr;
extern memory_word *zmem;
extern int32_t lo_mem_max;
extern int32_t hi_mem_min;
extern integer var_used, dyn_used;
extern int32_t avail;
extern int32_t mem_end;
extern int32_t rover;
extern int32_t last_leftmost_char;
extern int32_t last_rightmost_char;
extern int32_t hlist_stack[513];
extern short hlist_stack_level;
extern int32_t first_p;
extern int32_t global_prev_p;
extern integer font_in_short_display;
extern integer depth_threshold;
extern integer breadth_max;
extern list_state_record *nest;
extern integer nest_ptr;
extern integer max_nest_stack;
extern list_state_record cur_list;
extern short shown_mode;
extern unsigned char old_setting;
extern b32x2 *hash;
extern b32x2 *yhash;
extern int32_t hash_used;
extern int32_t hash_extra;
extern int32_t hash_top;
extern int32_t eqtb_top;
extern int32_t hash_high;
extern bool no_new_control_sequence;
extern integer cs_count;
extern b32x2 prim[501];
extern int32_t prim_used;
extern memory_word prim_eqtb[501];
extern memory_word *save_stack;
extern integer save_ptr;
extern integer max_save_stack;
extern uint16_t cur_level;
extern group_code cur_group;
extern integer cur_boundary;
extern integer mag_set;
extern eight_bits cur_cmd;
extern int32_t cur_chr;
extern int32_t cur_cs;
extern int32_t cur_tok;
extern input_state_t *input_stack;
extern integer input_ptr;
extern integer max_in_stack;
extern input_state_t cur_input;
extern integer in_open;
extern integer open_parens;
extern UFILE **input_file;
extern integer line;
extern integer *line_stack;
extern str_number *source_filename_stack;
extern str_number *full_source_filename_stack;
extern unsigned char scanner_status;
extern int32_t warning_index;
extern int32_t def_ref;
extern int32_t *param_stack;
extern integer param_ptr;
extern integer max_param_stack;
extern integer align_state;
extern integer base_ptr;
extern int32_t par_loc;
extern int32_t par_token;
extern bool force_eof;
extern integer expand_depth_count;
extern bool is_in_csname;
extern int32_t cur_mark[5];
extern unsigned char long_state;
extern int32_t pstack[9];
extern integer cur_val;
extern integer cur_val1;
extern unsigned char cur_val_level;
extern small_number radix;
extern glue_ord cur_order;
extern UFILE *read_file[16];
extern unsigned char read_open[17];
extern int32_t cond_ptr;
extern unsigned char if_limit;
extern small_number cur_if;
extern integer if_line;
extern integer skip_line;
extern str_number cur_name;
extern str_number cur_area;
extern str_number cur_ext;
extern pool_pointer area_delimiter;
extern pool_pointer ext_delimiter;
extern UTF16_code file_name_quote_char;
extern integer format_default_length;
extern char *TEX_format_default;
extern bool name_in_progress;
extern str_number job_name;
extern bool log_opened;
extern const char* output_file_extension;
extern rust_output_handle_t dvi_file;
extern str_number output_file_name;
extern str_number texmf_log_name;
extern memory_word *font_info;
extern font_index fmem_ptr;
extern internal_font_number font_ptr;
extern b16x4 *font_check;
extern scaled *font_size;
extern scaled *font_dsize;
extern font_index *font_params;
extern str_number *font_name;
extern str_number *font_area;
extern UTF16_code *font_bc;
extern UTF16_code *font_ec;
extern int32_t *font_glue;
extern bool *font_used;
extern integer *hyphen_char;
extern integer *skew_char;
extern font_index *bchar_label;
extern nine_bits *font_bchar;
extern nine_bits *font_false_bchar;
extern void **font_layout_engine;
extern void **font_mapping;
extern char *font_flags;
extern scaled *font_letter_space;
extern void *loaded_font_mapping;
extern char loaded_font_flags;
extern scaled loaded_font_letter_space;
extern scaled loaded_font_design_size;
extern UTF16_code *mapped_text;
extern char *xdv_buffer;
extern integer *char_base;
extern integer *width_base;
extern integer *height_base;
extern integer *depth_base;
extern integer *italic_base;
extern integer *lig_kern_base;
extern integer *kern_base;
extern integer *exten_base;
extern integer *param_base;
extern b16x4 null_character;
extern integer total_pages;
extern scaled max_v;
extern scaled max_h;
extern integer max_push;
extern integer last_bop;
extern integer dead_cycles;
extern bool doing_leaders;
extern uint16_t c;
extern internal_font_number f;
extern scaled rule_ht, rule_dp, rule_wd;
extern int32_t g;
extern integer lq, lr;
extern eight_bits *dvi_buf;
extern integer half_buf;
extern integer dvi_limit;
extern integer dvi_ptr;
extern integer dvi_offset;
extern integer dvi_gone;
extern int32_t down_ptr, right_ptr;
extern scaled dvi_h, dvi_v;
extern scaled cur_h, cur_v;
extern internal_font_number dvi_f;
extern integer cur_s;
extern scaled total_stretch[4], total_shrink[4];
extern integer last_badness;
extern int32_t adjust_tail;
extern int32_t pre_adjust_tail;
extern integer pack_begin_line;
extern b32x2 empty;
extern b16x4 null_delimiter;
extern int32_t cur_mlist;
extern small_number cur_style;
extern integer cur_size;
extern scaled cur_mu;
extern bool mlist_penalties;
extern internal_font_number cur_f;
extern integer cur_c;
extern b16x4 cur_i;
extern int32_t cur_align;
extern int32_t cur_span;
extern int32_t cur_loop;
extern int32_t align_ptr;
extern int32_t cur_head, cur_tail;
extern int32_t cur_pre_head, cur_pre_tail;
extern int32_t just_box;
extern int32_t passive;
extern int32_t printed_node;
extern int32_t pass_number;
extern scaled active_width[7];
extern scaled cur_active_width[7];
extern scaled background[7];
extern scaled break_width[7];
extern bool no_shrink_error_yet;
extern int32_t cur_p;
extern bool second_pass;
extern bool final_pass;
extern integer threshold;
extern integer minimal_demerits[4];
extern integer minimum_demerits;
extern int32_t best_place[4];
extern int32_t best_pl_line[4];
extern scaled disc_width;
extern int32_t easy_line;
extern int32_t last_special_line;
extern scaled first_width;
extern scaled second_width;
extern scaled first_indent;
extern scaled second_indent;
extern int32_t best_bet;
extern integer fewest_demerits;
extern int32_t best_line;
extern integer actual_looseness;
extern integer line_diff;
extern integer hc[4099];
extern small_number hn;
extern int32_t ha, hb;
extern internal_font_number hf;
extern integer hu[4097];
extern integer hyf_char;
extern unsigned char cur_lang, init_cur_lang;
extern integer l_hyf, r_hyf, init_l_hyf, init_r_hyf;
extern int32_t hyf_bchar;
extern integer max_hyph_char;
extern unsigned char hyf[4097];
extern int32_t init_list;
extern bool init_lig;
extern bool init_lft;
extern small_number hyphen_passed;
extern int32_t cur_l, cur_r;
extern int32_t cur_q;
extern int32_t lig_stack;
extern bool ligature_present;
extern bool lft_hit, rt_hit;
extern trie_pointer *trie_trl;
extern trie_pointer *trie_tro;
extern uint16_t *trie_trc;
extern small_number hyf_distance[trie_op_size + 1];
extern small_number hyf_num[trie_op_size + 1];
extern trie_opcode hyf_next[trie_op_size + 1];
extern integer op_start[256];
extern str_number *hyph_word;
extern int32_t *hyph_list;
extern hyph_pointer *hyph_link;
extern integer hyph_count;
extern integer hyph_next;
extern trie_opcode trie_used[256];
extern unsigned char trie_op_lang[trie_op_size + 1];
extern trie_opcode trie_op_val[trie_op_size + 1];
extern integer trie_op_ptr;
extern trie_opcode max_op_used;
extern packed_UTF16_code *trie_c;
extern trie_opcode *trie_o;
extern trie_pointer *trie_l;
extern trie_pointer *trie_r;
extern trie_pointer trie_ptr;
extern trie_pointer *trie_hash;
extern bool *trie_taken;
extern trie_pointer trie_min[65536];
extern trie_pointer trie_max;
extern bool trie_not_ready;
extern scaled best_height_plus_depth;
extern int32_t page_tail;
extern unsigned char page_contents;
extern scaled page_max_depth;
extern int32_t best_page_break;
extern integer least_page_cost;
extern scaled best_size;
extern scaled page_so_far[8];
extern int32_t last_glue;
extern integer last_penalty;
extern scaled last_kern;
extern integer last_node_type;
extern integer insert_penalties;
extern bool output_active;
extern internal_font_number main_f;
extern b16x4 main_i;
extern b16x4 main_j;
extern font_index main_k;
extern int32_t main_p;
extern int32_t main_pp, main_ppp;
extern int32_t main_h;
extern bool is_hyph;
extern integer space_class;
extern integer prev_class;
extern integer main_s;
extern int32_t bchar;
extern int32_t false_bchar;
extern bool cancel_boundary;
extern bool ins_disc;
extern int32_t cur_box;
extern int32_t after_token;
extern bool long_help_seen;
extern str_number format_ident;
extern rust_output_handle_t write_file[16];
extern bool write_open[18];
extern int32_t write_loc;
extern scaled cur_page_width;
extern scaled cur_page_height;
extern scaled cur_h_offset;
extern scaled cur_v_offset;
extern integer pdf_last_x_pos;
extern integer pdf_last_y_pos;
extern bool *eof_seen;
extern int32_t LR_ptr;
extern integer LR_problems;
extern small_number cur_dir;
extern int32_t pseudo_files;
extern save_pointer *grp_stack;
extern int32_t *if_stack;
extern int32_t max_reg_num;
extern const char* max_reg_help_line;
extern int32_t sa_root[8];
extern int32_t cur_ptr;
extern memory_word sa_null;
extern int32_t sa_chain;
extern uint16_t sa_level;
extern int32_t last_line_fill;
extern bool do_last_line_fit;
extern small_number active_node_size;
extern scaled fill_width[3];
extern scaled best_pl_short[4];
extern scaled best_pl_glue[4];
extern trie_pointer hyph_start;
extern trie_pointer hyph_index;
extern int32_t disc_ptr[4];
extern pool_pointer edit_name_start;
extern bool stop_at_space;
extern unsigned char k, l;
extern integer native_font_type_flag;
extern bool xtx_ligature_present;
extern scaled delta;
extern int synctex_enabled;
extern bool used_tectonic_coda_tokens;
extern bool semantic_pagination_enabled;

/*:1683*/

/* It looks like these arrays are set up so that they can be safely indexed
 * with negative indices. The underlying arrays used to be named "zzzaa" and
 * "zzzbb". */

extern uint16_t _xeq_level_array[1114731];
#define xeq_level (_xeq_level_array - 8938740)

extern integer _trie_op_hash_array[trie_op_size - neg_trie_op_size + 1];
#define trie_op_hash (_trie_op_hash_array - (int) neg_trie_op_size)

/* the former xetexcoerce.h: */

void print_ln(void);
void print_raw_char(UTF16_code s, bool incr_offset);
void print_char(integer s);
void print(integer s);
void print_cstr(const char* s);
void print_nl(str_number s);
void print_nl_cstr(const char* s);
void print_esc(str_number s);
void print_esc_cstr(const char* s);
void print_int(integer n);
void print_cs(integer p);
void sprint_cs(int32_t p);
void print_file_name(integer n, integer a, integer e);
void print_size(integer s);
void print_write_whatsit(const char* s, int32_t p);
void print_native_word(int32_t p);
void print_sa_num(int32_t q);
void print_file_line(void);
void print_two(integer n);
void print_hex(integer n);
void print_roman_int(integer n);
void print_scaled(scaled s);

void error(void);
NORETURN void fatal_error(const char* s);
NORETURN void overflow(const char* s, integer n);
NORETURN void confusion(const char* s);
NORETURN void pdf_error(const char* t, const char* p);

integer length(str_number s);
str_number make_string(void);
void append_str(str_number s);
bool str_eq_buf(str_number s, integer k);
bool str_eq_str(str_number s, str_number t);
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
void check_mem(bool print_locs);
void search_mem(int32_t p);
int32_t prev_rightmost(int32_t s, int32_t e);
scaled round_xn_over_d(scaled x, integer n, integer d);
void short_display(integer p);
void print_font_and_char(integer p);
void print_mark(integer p);
void print_rule_dimen(scaled d);
void print_glue(scaled d, integer order, const char* s);
void print_spec(integer p, const char* s);
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
void end_diagnostic(bool blank_line);
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
void print_group(bool e);
void group_trace(bool e);
bool pseudo_input(void);
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
void find_sa_element(small_number t, int32_t n, bool w);
void expand(void);
void get_x_token(void);
void x_token(void);
void scan_left_brace(void);
void scan_optional_equals(void);
bool scan_keyword(const char* s);
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
integer effective_char(bool err_p, internal_font_number f, uint16_t c);
void scan_font_ident(void);
void find_font_dimen(bool writing);
void scan_something_internal(small_number level, bool negative);
void scan_int(void);
void xetex_scan_dimen(bool mu, bool inf, bool shortcut, bool requires_units);
void scan_dimen(bool mu, bool inf, bool shortcut);
void scan_decimal(void);
void scan_glue(small_number level);
integer add_or_sub(integer x, integer y, integer max_answer, bool negative);
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
int32_t scan_toks(bool macro_def, bool xpand);
void read_toks(integer n, int32_t r, int32_t j);
void pass_text(void);
void change_if_limit(small_number l, int32_t p);
void conditional(void);
void begin_name(void);
bool more_name(UTF16_code c);
void end_name(void);
void pack_file_name(str_number n, str_number a, str_number e);
str_number make_name_string(void);
void scan_file_name(void);
void pack_job_name(const char*);
void open_log_file(void);
void start_input(const char *primary_input_name);
b16x4 effective_char_info(internal_font_number f, uint16_t c);
void char_warning(internal_font_number f, integer c);
int32_t new_native_word_node(internal_font_number f, integer n);
int32_t new_native_character(internal_font_number f, UnicodeScalar c);
void font_feature_warning(const void *featureNameP, integer featLen, const void *settingNameP, integer setLen);
void font_mapping_warning(const void *mappingNameP, integer mappingNameLen, integer warningType);
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
void scan_spec(group_code c, bool three_codes);
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
int32_t build_opentype_assembly(internal_font_number f, void *a, scaled s, bool horiz);
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
bool fin_col(void);
void fin_row(void);
void fin_align(void);
void align_peek(void);
int32_t finite_shrink(int32_t p);
void push_node(int32_t p);
int32_t pop_node(void);
int32_t find_protchar_left(int32_t l, bool d);
int32_t find_protchar_right(int32_t l, int32_t r);
scaled total_pw(int32_t q, int32_t p);
void try_break(integer pi, small_number break_type);
void post_line_break(bool d);
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
void line_break(bool d);
bool eTeX_enabled(bool b, uint16_t j, int32_t k);
void show_save_groups(void);
int32_t prune_page_top(int32_t p, bool s);
int32_t vert_break(int32_t p, scaled h, scaled d);
bool do_marks(small_number a, small_number l, int32_t q);
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
bool privileged(void);
bool its_all_over(void);
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
void new_graf(bool indented);
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
void scan_delimiter(int32_t p, bool r);
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
void load_picture(bool is_pdf);
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

/* Tectonic related functions */
tt_history_t tt_run_engine(char *dump_name, char *input_file_name);


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
