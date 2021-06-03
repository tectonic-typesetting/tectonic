/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEX_XETEXD_H
#define TECTONIC_XETEX_XETEXD_H

#include "xetex-core.h"
#include "xetex-XeTeXOTMath.h"
#include "teckit-Common.h"
#include "xetex-ext.h"
#include "tectonic_bridge_core.h"
#include "xetex-constants.h"

BEGIN_EXTERN_C

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

/*11:*/
#define MIN_TRIE_OP 0
#define TRIE_OP_SIZE 35111L

/*18: */
typedef unsigned short UTF16_code;
typedef unsigned char UTF8_code;
typedef int32_t UnicodeScalar;
typedef unsigned char eight_bits;
typedef int32_t pool_pointer;
typedef int32_t str_number;
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
 *           int32_t CINT;
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

/* Symbolic accessors for various TeX data structures. I would loooove to turn these
 * into actual structs, but the path to doing that is not currently clear. Making
 * field references symbolic seems like a decent start. Sadly I don't see how to do
 * this conversion besides painstakingly annotating things.
 */

#define LLIST_link(p) mem[p].b32.s1
#define LLIST_info(p) mem[p].b32.s0

#define NODE_type(p) mem[p].b16.s1 /* half of LLIST_info(p) */
#define NODE_subtype(p) mem[p].b16.s0 /* the other half of LLIST_info(p) */

#define BOX_lr_mode(p) mem[p].b16.s0 /* subtype; records L/R direction mode */
#define BOX_width(p) mem[(p) + 1].b32.s1 /* a scaled; 1 <=> WEB const `width_offset` */
#define BOX_depth(p) mem[(p) + 2].b32.s1 /* a scaled; 2 <=> WEB const `depth_offset` */
#define BOX_height(p) mem[(p) + 3].b32.s1 /* a scaled; 3 <=> WEB const `height_offset` */
#define BOX_shift_amount(p) mem[(p) + 4].b32.s1 /* a scaled */
#define BOX_list_ptr(p) mem[(p) + 5].b32.s1 /* aka `link` of p+5 */
#define BOX_glue_sign(p) mem[(p) + 5].b16.s1 /* aka `type` of p+5 */
#define BOX_glue_order(p) mem[(p) + 5].b16.s0 /* aka `subtype` of p+5 */
#define BOX_glue_set(p) mem[(p) + 6].gr /* the glue ratio */

#define ACTIVE_NODE_fitness(p) mem[p].b16.s0 /* aka "subtype" of a node */
#define ACTIVE_NODE_break_node(p) mem[(p) + 1].b32.s1 /* aka "rlink" in double-linked list */
#define ACTIVE_NODE_line_number(p) mem[(p) + 1].b32.s0 /* aka "llink" in doubly-linked list */
#define ACTIVE_NODE_total_demerits(p) mem[(p) + 2].b32.s1 /* was originally the `mem[x+2].int` field */
#define ACTIVE_NODE_shortfall(p) mem[(p) + 3].b32.s1 /* a scaled; "active_short" in the WEB */
#define ACTIVE_NODE_glue(p) mem[(p) + 4].b32.s1 /* a scaled */

#define CHAR_NODE_font(p) mem[p].b16.s1 /* aka "type" of a node */
#define CHAR_NODE_character(p) mem[p].b16.s0 /* aka "subtype" of a node */

#define DELTA_NODE_dwidth(p) mem[(p) + 1].b32.s1 /* the "natural width" difference */
#define DELTA_NODE_dstretch0(p) mem[(p) + 2].b32.s1 /* the stretch difference in points */
#define DELTA_NODE_dstretch1(p) mem[(p) + 3].b32.s1 /* the stretch difference in fil */
#define DELTA_NODE_dstretch2(p) mem[(p) + 4].b32.s1 /* the stretch difference in fill */
#define DELTA_NODE_dstretch3(p) mem[(p) + 5].b32.s1 /* the stretch difference in fill */
#define DELTA_NODE_dshrink(p) mem[(p) + 6].b32.s1 /* the shrink difference */

#define DISCRETIONARY_NODE_replace_count(p) mem[p].b16.s0 /* aka "subtype" of a node */
#define DISCRETIONARY_NODE_pre_break(p) mem[(p) + 1].b32.s0 /* aka "llink" in doubly-linked list */
#define DISCRETIONARY_NODE_post_break(p) mem[(p) + 1].b32.s1 /* aka "rlink" in double-linked list */

#define EDGE_NODE_edge_dist(p) mem[(p) + 2].b32.s1 /* "new left_edge position relative to cur_h" */

#define GLUE_NODE_glue_ptr(p) mem[(p) + 1].b32.s0 /* aka "llink" in doubly-linked list */
#define GLUE_NODE_leader_ptr(p) mem[(p) + 1].b32.s1 /* aka "rlink" in double-linked list */

#define INSERTION_NODE_float_cost(p) mem[(p) + 1].b32.s1 /* "the floating_penalty to be used" */
#define INSERTION_NODE_split_top_ptr(p) mem[(p) + 4].b32.s1 /* a glue pointer */
#define INSERTION_NODE_ins_ptr(p) mem[(p) + 4].b32.s0 /* a pointer to a vlist */

#define LANGUAGE_NODE_what_lang(p) mem[(p) + 1].b32.s1 /* language number, 0..255 */
#define LANGUAGE_NODE_what_lhm(p) mem[(p) + 1].b16.s1 /* "minimum left fragment, range 1..63" */
#define LANGUAGE_NODE_what_rhm(p) mem[(p) + 1].b16.s0 /* "minimum right fragment, range 1..63" */

#define LIGATURE_NODE_lig_font(p) mem[(p) + 1].b16.s1 /* WEB: font(lig_char(p)) */
#define LIGATURE_NODE_lig_char(p) mem[(p) + 1].b16.s0 /* WEB: character(lig_char(p)) */
#define LIGATURE_NODE_lig_ptr(p) mem[(p) + 1].b32.s1 /* WEB: link(lig_char(p)) */

#define MARK_NODE_ptr(p) mem[(p) + 1].b32.s1 /* "head of the token list for the mark" */
#define MARK_NODE_class(p) mem[(p) + 1].b32.s0 /* "the mark class" */

/* To check: do these really only apply to MATH_NODEs? */
#define MATH_NODE_lr_dir(p) (NODE_subtype(p) / R_CODE)
#define MATH_NODE_end_lr_type(p) (L_CODE * (NODE_subtype(p) / L_CODE) + END_M_CODE)

#define NATIVE_NODE_size(p) mem[(p) + 4].b16.s3
#define NATIVE_NODE_font(p) mem[(p) + 4].b16.s2
#define NATIVE_NODE_length(p) mem[(p) + 4].b16.s1 /* number of UTF16 items in the text */
#define NATIVE_NODE_glyph(p) mem[(p) + 4].b16.s1 /* ... or the glyph number, if subtype==GLYPH_NODE */
#define NATIVE_NODE_glyph_count(p) mem[(p) + 4].b16.s0
#define NATIVE_NODE_glyph_info_ptr(p) mem[(p) + 5].ptr
#define NATIVE_NODE_text(p) ((unsigned short *) &mem[(p) + NATIVE_NODE_SIZE])

#define PAGE_INS_NODE_broken_ptr(p) mem[(p) + 1].b32.s1 /* "an insertion for this class will break here if anywhere" */
#define PAGE_INS_NODE_broken_ins(p) mem[(p) + 1].b32.s0 /* "this insertion might break at broken_ptr" */
#define PAGE_INS_NODE_last_ins_ptr(p) mem[(p) + 2].b32.s1 /* "the most recent insertion for this subtype" */
#define PAGE_INS_NODE_best_ins_ptr(p) mem[(p) + 2].b32.s0 /* "the optimum most recent insertion" */

#define PASSIVE_NODE_prev_break(p) mem[(p) + 1].b32.s0 /* aka "llink" in doubly-linked list */
#define PASSIVE_NODE_next_break(p) PASSIVE_NODE_prev_break(p) /* siggggghhhhh */
#define PASSIVE_NODE_cur_break(p) mem[(p) + 1].b32.s1 /* aka "rlink" in double-linked list */
#define PASSIVE_NODE_serial(p) mem[p].b32.s0 /* aka "info" */

#define PENALTY_NODE_penalty(p) mem[(p) + 1].b32.s1 /* was originally the `mem[x+1].int` field */

#define PIC_NODE_path_len(p) mem[(p) + 4].b16.s1 /* number of bytes in the path item */
#define PIC_NODE_path(p) ((unsigned char *) &mem[(p) + PIC_NODE_SIZE])
#define PIC_NODE_total_size(p) (PIC_NODE_SIZE + (PIC_NODE_path_len(p) + sizeof(memory_word) - 1) / sizeof(memory_word))

#define WRITE_NODE_tokens(p) mem[(p) + 1].b32.s1 /* "reference count of token list to write" */

/* Synctex hacks various nodes to add an extra word at the end to store its
 * information, hence the need to know the node size to get the synctex
 * info. */
#define SYNCTEX_tag(p, nodesize) mem[(p) + nodesize - SYNCTEX_FIELD_SIZE].b32.s0
#define SYNCTEX_line(p, nodesize) mem[(p) + nodesize - SYNCTEX_FIELD_SIZE].b32.s1

#define GLUE_SPEC_ref_count(p) mem[p].b32.s1 /* aka "link" of a link-list node */
#define GLUE_SPEC_stretch_order(p) mem[p].b16.s1 /* aka "type" of a node */
#define GLUE_SPEC_shrink_order(p) mem[p].b16.s0 /* aka "subtype" of a node */
#define GLUE_SPEC_stretch(p) mem[(p) + 2].b32.s1 /* a scaled */
#define GLUE_SPEC_shrink(p) mem[(p) + 3].b32.s1 /* a scaled */

#define FONT_CHARACTER_INFO(f, c) font_info[char_base[f] + (c)].b16
#define FONT_CHARINFO_WIDTH(f, info) font_info[width_base[f] + (info).s3].b32.s1
#define FONT_CHARINFO_HEIGHT(f, info) font_info[height_base[f] + (info).s2 / 16].b32.s1
#define FONT_CHARINFO_DEPTH(f, info) font_info[depth_base[f] + (info).s2 % 16].b32.s1
#define FONT_CHARINFO_ITALCORR(f, info) font_info[italic_base[f] + (info).s1 / 4].b32.s1
#define FONT_CHARACTER_WIDTH(f, c) FONT_CHARINFO_WIDTH(f, FONT_CHARACTER_INFO(f, c))

#define TOKEN_LIST_ref_count(p) mem[p].b32.s0

/* e-TeX sparse arrays for large-numebered registers, etc. */
#define ETEX_SA_ref(p) mem[(p) + 1].b32.s0
#define ETEX_SA_ptr(p) mem[(p) + 1].b32.s1
#define ETEX_SA_num(p) ETEX_SA_ptr(p)

/* e-TeX extended marks stuff ... not sure where to put these */
#define ETEX_MARK_sa_top_mark(p) mem[(p) + 1].b32.s0 /* \topmarks<n> */
#define ETEX_MARK_sa_first_mark(p) mem[(p) + 1].b32.s1 /* \firstmarks<n> */
#define ETEX_MARK_sa_bot_mark(p) mem[(p) + 2].b32.s0 /* \botmarks<n> */
#define ETEX_MARK_sa_split_first_mark(p) mem[(p) + 2].b32.s1 /* \splitfirstmarks<n> */
#define ETEX_MARK_sa_split_bot_mark(p) mem[(p) + 3].b32.s0 /* \splitbotmarks<n> */

typedef unsigned char glue_ord; /* enum: normal .. filll */
typedef unsigned char group_code;
typedef int32_t internal_font_number;
typedef int32_t font_index;
typedef int32_t nine_bits; /* range: 0 .. 0x1FF */
typedef int32_t trie_pointer;
typedef unsigned short trie_opcode;
typedef unsigned short hyph_pointer;
typedef int32_t save_pointer;

typedef struct {
    short mode; /* which mode we are: horz, vert, etc. */
    int32_t head; /* pointer to head of list being built */
    int32_t tail; /* pointer to tail of list being built */
    int32_t eTeX_aux; /* LR_save or LR_box or delim_ptr */
    int32_t prev_graf; /* number of lines that have already been put into the current vlist */
    int32_t mode_line; /* source line number at which this level was entered */
    memory_word aux; /* prev_depth or space_factor/clang or incompleat_noad */
} list_state_record;

typedef struct {
    uint16_t state; /* tokenizer state: mid_line, skip_blanks, new_line */
    uint16_t index; /* index of this level of input in input_file array */
    int32_t start; /* position of beginning of current line in `buffer` */
    int32_t loc; /* position of next character to read in `buffer` */
    int32_t limit; /* position of end of line in `buffer` */
    int32_t name; /* string number: name of current file or magic value for terminal, etc. */
    int32_t synctex_tag;
} input_state_t;

typedef enum {
    HISTORY_SPOTLESS = 0,
    HISTORY_WARNING_ISSUED = 1,
    HISTORY_ERROR_ISSUED = 2,
    HISTORY_FATAL_ERROR = 3
} tt_history_t;

/* Functions originating in texmfmp.c */

void getmd5sum(int32_t s, bool file);

void init_start_time(time_t source_date_epoch);
void get_date_and_time (time_t source_date_epoch, int32_t *minutes, int32_t *day, int32_t *month, int32_t *year);
void get_seconds_and_micros (int32_t *seconds,  int32_t *micros);

void getcreationdate(void);
void getfilemoddate(int32_t s);
void getfilesize(int32_t s);
void getfiledump(int32_t s, int offset, int length);

char *gettexstring(str_number);
bool is_new_source(str_number, int);
pool_pointer make_src_special(str_number, int);
void remember_source_info(str_number, int);

/* Needed here for UFILE */
#include "xetex-io.h"

/* variables! */

/* All the following variables are defined in xetexini.c */
extern bool shell_escape_enabled;
extern memory_word *eqtb;
extern int32_t bad;
extern char *name_of_file;
extern UTF16_code *name_of_file16;
extern int32_t name_length;
extern int32_t name_length16;
extern UnicodeScalar *buffer;
extern int32_t first;
extern int32_t last;
extern int32_t max_buf_stack;
extern bool in_initex_mode;
extern int32_t error_line;
extern int32_t half_error_line;
extern int32_t max_print_line;
extern int32_t max_strings;
extern int32_t strings_free;
extern int32_t string_vacancies;
extern int32_t pool_size;
extern int32_t pool_free;
extern int32_t font_mem_size;
extern int32_t font_max;
extern int32_t hyph_size;
extern int32_t trie_size;
extern int32_t buf_size;
extern int32_t stack_size;
extern int32_t max_in_open;
extern int32_t param_size;
extern int32_t nest_size;
extern int32_t save_size;
extern int32_t expand_depth;
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
extern int32_t tally;
extern int32_t term_offset;
extern int32_t file_offset;
extern UTF16_code trick_buf[256];
extern int32_t trick_count;
extern int32_t first_count;
extern bool doing_special;
extern UTF16_code *native_text;
extern int32_t native_text_size;
extern int32_t native_len;
extern int32_t save_native_len;
extern unsigned char interaction;
extern bool deletions_allowed;
extern bool set_box_allowed;
extern tt_history_t history;
extern signed char error_count;
extern const char* help_line[6];
extern unsigned char help_ptr;
extern bool use_err_help;
extern bool arith_error;
extern scaled_t tex_remainder;
extern int32_t randoms[55];
extern unsigned char j_random;
extern scaled_t random_seed;
extern int32_t two_to_the[31];
extern int32_t spec_log[29];
extern int32_t temp_ptr;
extern memory_word *mem;
extern int32_t lo_mem_max;
extern int32_t hi_mem_min;
extern int32_t var_used, dyn_used;
extern int32_t avail;
extern int32_t mem_end;
extern int32_t rover;
extern int32_t last_leftmost_char;
extern int32_t last_rightmost_char;
extern int32_t hlist_stack[513];
extern short hlist_stack_level;
extern int32_t first_p;
extern int32_t global_prev_p;
extern int32_t font_in_short_display;
extern int32_t depth_threshold;
extern int32_t breadth_max;
extern list_state_record *nest;
extern int32_t nest_ptr;
extern int32_t max_nest_stack;
extern list_state_record cur_list;
extern short shown_mode;
extern unsigned char old_setting;
extern b32x2 *hash;
extern int32_t hash_used;
extern int32_t hash_extra;
extern int32_t hash_top;
extern int32_t eqtb_top;
extern int32_t hash_high;
extern bool no_new_control_sequence;
extern int32_t cs_count;
extern b32x2 prim[501];
extern int32_t prim_used;
extern memory_word *save_stack;
extern int32_t save_ptr;
extern int32_t max_save_stack;
extern uint16_t cur_level;
extern group_code cur_group;
extern int32_t cur_boundary;
extern int32_t mag_set;
extern eight_bits cur_cmd;
extern int32_t cur_chr;
extern int32_t cur_cs;
extern int32_t cur_tok;
extern input_state_t *input_stack;
extern int32_t input_ptr;
extern int32_t max_in_stack;
extern input_state_t cur_input;
extern int32_t in_open;
extern int32_t open_parens;
extern UFILE **input_file;
extern int32_t line;
extern int32_t *line_stack;
extern str_number *source_filename_stack;
extern str_number *full_source_filename_stack;
extern unsigned char scanner_status;
extern int32_t warning_index;
extern int32_t def_ref;
extern int32_t *param_stack;
extern int32_t param_ptr;
extern int32_t max_param_stack;
extern int32_t align_state;
extern int32_t base_ptr;
extern int32_t par_loc;
extern int32_t par_token;
extern bool force_eof;
extern int32_t expand_depth_count;
extern bool is_in_csname;
extern int32_t cur_mark[5];
extern unsigned char long_state;
extern int32_t pstack[9];
extern int32_t cur_val;
extern int32_t cur_val1;
extern unsigned char cur_val_level;
extern small_number radix;
extern glue_ord cur_order;
extern UFILE *read_file[16];
extern unsigned char read_open[17];
extern int32_t cond_ptr;
extern unsigned char if_limit;
extern small_number cur_if;
extern int32_t if_line;
extern int32_t skip_line;
extern str_number cur_name;
extern str_number cur_area;
extern str_number cur_ext;
extern pool_pointer area_delimiter;
extern pool_pointer ext_delimiter;
extern UTF16_code file_name_quote_char;
extern int32_t format_default_length;
extern char *TEX_format_default;
extern bool name_in_progress;
extern str_number job_name;
extern bool log_opened;
extern const char* output_file_extension;
extern str_number texmf_log_name;
extern memory_word *font_info;
extern font_index fmem_ptr;
extern internal_font_number font_ptr;
extern b16x4 *font_check;
extern scaled_t *font_size;
extern scaled_t *font_dsize;
extern font_index *font_params;
extern str_number *font_name;
extern str_number *font_area;
extern UTF16_code *font_bc;
extern UTF16_code *font_ec;
extern int32_t *font_glue;
extern bool *font_used;
extern int32_t *hyphen_char;
extern int32_t *skew_char;
extern font_index *bchar_label;
extern nine_bits *font_bchar;
extern nine_bits *font_false_bchar;
extern void **font_layout_engine;
extern void **font_mapping;
extern char *font_flags;
extern scaled_t *font_letter_space;
extern void *loaded_font_mapping;
extern char loaded_font_flags;
extern scaled_t loaded_font_letter_space;
extern UTF16_code *mapped_text;
extern char *xdv_buffer;
extern int32_t *char_base;
extern int32_t *width_base;
extern int32_t *height_base;
extern int32_t *depth_base;
extern int32_t *italic_base;
extern int32_t *lig_kern_base;
extern int32_t *kern_base;
extern int32_t *exten_base;
extern int32_t *param_base;
extern b16x4 null_character;
extern int32_t total_pages;
extern scaled_t max_v;
extern scaled_t max_h;
extern int32_t max_push;
extern int32_t last_bop;
extern int32_t dead_cycles;
extern bool doing_leaders;
extern scaled_t rule_ht, rule_dp, rule_wd;
extern scaled_t cur_h, cur_v; /* should be internal to shipout, but accessed by synctex */
extern int32_t epochseconds;
extern int32_t microseconds;
extern scaled_t total_stretch[4], total_shrink[4];
extern int32_t last_badness;
extern int32_t adjust_tail;
extern int32_t pre_adjust_tail;
extern int32_t pack_begin_line;
extern b32x2 empty;
extern internal_font_number cur_f;
extern int32_t cur_c;
extern b16x4 cur_i;
extern int32_t cur_align;
extern int32_t cur_span;
extern int32_t cur_loop;
extern int32_t align_ptr;
extern int32_t cur_head, cur_tail;
extern int32_t cur_pre_head, cur_pre_tail;
extern int32_t just_box;
extern scaled_t active_width[7];
extern int32_t hc[4099];
extern internal_font_number hf;
extern int32_t hu[4097];
extern unsigned char cur_lang;
extern int32_t max_hyph_char;
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
extern small_number hyf_distance[TRIE_OP_SIZE + 1];
extern small_number hyf_num[TRIE_OP_SIZE + 1];
extern trie_opcode hyf_next[TRIE_OP_SIZE + 1];
extern int32_t op_start[256];
extern str_number *hyph_word;
extern int32_t *hyph_list;
extern hyph_pointer *hyph_link;
extern int32_t hyph_count;
extern int32_t hyph_next;
extern trie_opcode trie_used[256];
extern unsigned char trie_op_lang[TRIE_OP_SIZE + 1];
extern trie_opcode trie_op_val[TRIE_OP_SIZE + 1];
extern int32_t trie_op_ptr;
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
extern scaled_t best_height_plus_depth;
extern int32_t page_tail;
extern unsigned char page_contents;
extern scaled_t page_so_far[8];
extern int32_t last_glue;
extern int32_t last_penalty;
extern scaled_t last_kern;
extern int32_t last_node_type;
extern int32_t insert_penalties;
extern bool output_active;
extern internal_font_number main_f;
extern b16x4 main_i;
extern b16x4 main_j;
extern font_index main_k;
extern int32_t main_p;
extern int32_t main_pp, main_ppp;
extern int32_t main_h;
extern bool is_hyph;
extern int32_t space_class;
extern int32_t prev_class;
extern int32_t main_s;
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
extern scaled_t cur_page_width;
extern scaled_t cur_page_height;
extern scaled_t cur_h_offset;
extern scaled_t cur_v_offset;
extern int32_t pdf_last_x_pos;
extern int32_t pdf_last_y_pos;
extern bool *eof_seen;
extern int32_t LR_ptr;
extern int32_t LR_problems;
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
extern trie_pointer hyph_start;
extern trie_pointer hyph_index;
extern int32_t disc_ptr[4];
extern pool_pointer edit_name_start;
extern bool stop_at_space;
extern int32_t native_font_type_flag;
extern bool xtx_ligature_present;
extern scaled_t delta;
extern int synctex_enabled;
extern bool used_tectonic_coda_tokens;
extern bool semantic_pagination_enabled;
extern bool gave_char_warning_help;

/*:1683*/

/* It looks like these arrays are set up so that they can be safely indexed
 * with negative indices. The underlying arrays used to be named "zzzaa" and
 * "zzzbb". */

extern uint16_t _xeq_level_array[EQTB_SIZE - INT_BASE + 1];
#define XEQ_LEVEL(i) _xeq_level_array[(i) - INT_BASE]

/* the former xetexcoerce.h: */

int32_t badness(scaled_t t, scaled_t s);
void print_word(memory_word w);
void show_token_list(int32_t p, int32_t q, int32_t l);
void runaway(void);
int32_t get_avail(void);
void flush_list(int32_t p);
int32_t get_node(int32_t s);
void free_node(int32_t p, int32_t s);
int32_t new_null_box(void);
int32_t new_rule(void);
int32_t new_ligature(internal_font_number f, uint16_t c, int32_t q);
int32_t new_lig_item(uint16_t c);
int32_t new_disc(void);
void copy_native_glyph_info(int32_t src, int32_t dest);
int32_t new_math(scaled_t w, small_number s);
int32_t new_spec(int32_t p);
int32_t new_param_glue(small_number n);
int32_t new_glue(int32_t q);
int32_t new_skip_param(small_number n);
int32_t new_kern(scaled_t w);
int32_t new_penalty(int32_t m);
void check_mem(bool print_locs);
void search_mem(int32_t p);
int32_t prev_rightmost(int32_t s, int32_t e);
int32_t get_microinterval(void);
scaled_t round_xn_over_d(scaled_t x, int32_t n, int32_t d);
void short_display(int32_t p);
void print_font_and_char(int32_t p);
void print_mark(int32_t p);
void print_rule_dimen(scaled_t d);
void print_glue(scaled_t d, int32_t order, const char* s);
void print_spec(int32_t p, const char* s);
void print_fam_and_char(int32_t p);
void print_delimiter(int32_t p);
void print_subsidiary_data(int32_t p, UTF16_code c);
void print_style(int32_t c);
void print_skip_param(int32_t n);
void show_node_list(int32_t p);
void show_box(int32_t p);
void short_display_n(int32_t p, int32_t m);
void delete_token_ref(int32_t p);
void delete_glue_ref(int32_t p);
void flush_node_list(int32_t p);
int32_t copy_node_list(int32_t p);
void print_mode(int32_t m);
void print_in_mode(int32_t m);
void push_nest(void);
void pop_nest(void);
void show_activities(void);
void print_param(int32_t n);
void begin_diagnostic(void);
void end_diagnostic(bool blank_line);
void print_length_param(int32_t n);
void print_cmd_chr(uint16_t cmd, int32_t chr_code);
void not_aat_font_error(int32_t cmd, int32_t c, int32_t f);
void not_aat_gr_font_error(int32_t cmd, int32_t c, int32_t f);
void not_ot_font_error(int32_t cmd, int32_t c, int32_t f);
void not_native_font_error(int32_t cmd, int32_t c, int32_t f);
void show_eqtb(int32_t n);
int32_t id_lookup(int32_t j, int32_t l);
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
void sa_w_def(int32_t p, int32_t w);
void gsa_def(int32_t p, int32_t e);
void gsa_w_def(int32_t p, int32_t w);
void sa_restore(void);
void new_save_level(group_code c);
void eq_destroy(memory_word w);
void eq_save(int32_t p, uint16_t l);
void eq_define(int32_t p, uint16_t t, int32_t e);
void eq_word_define(int32_t p, int32_t w);
void geq_define(int32_t p, uint16_t t, int32_t e);
void geq_word_define(int32_t p, int32_t w);
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
int32_t effective_char(bool err_p, internal_font_number f, uint16_t c);
void scan_font_ident(void);
void find_font_dimen(bool writing);
void scan_something_internal(small_number level, bool negative);
void scan_int(void);
void xetex_scan_dimen(bool mu, bool inf, bool shortcut, bool requires_units);
void scan_dimen(bool mu, bool inf, bool shortcut);
void scan_decimal(void);
void scan_glue(small_number level);
int32_t add_or_sub(int32_t x, int32_t y, int32_t max_answer, bool negative);
int32_t quotient(int32_t n, int32_t d);
int32_t fract(int32_t x, int32_t n, int32_t d, int32_t max_answer);
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
void read_toks(int32_t n, int32_t r, int32_t j);
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
void char_warning(internal_font_number f, int32_t c);
int32_t new_native_word_node(internal_font_number f, int32_t n);
int32_t new_native_character(internal_font_number f, UnicodeScalar c);
void font_feature_warning(const void *featureNameP, int32_t featLen, const void *settingNameP, int32_t setLen);
void font_mapping_warning(const void *mappingNameP, int32_t mappingNameLen, int32_t warningType);
void graphite_warning(void);
internal_font_number load_native_font(int32_t u, str_number nom, str_number aire, scaled_t s);
void do_locale_linebreaks(int32_t s, int32_t len);
void bad_utf8_warning(void);
int32_t get_input_normalization_state(void);
int32_t get_tracing_fonts_state(void);
internal_font_number read_font_info(int32_t u, str_number nom, str_number aire, scaled_t s);
int32_t new_character(internal_font_number f, UTF16_code c);
void out_what(int32_t p);
int32_t new_edge(small_number s, scaled_t w);
void scan_spec(group_code c, bool three_codes);
scaled_t char_pw(int32_t p, small_number side);
int32_t new_margin_kern(scaled_t w, int32_t p, small_number side);
int32_t hpack(int32_t p, scaled_t w, small_number m);
int32_t vpackage(int32_t p, scaled_t h, small_number m, scaled_t l);
void append_to_vlist(int32_t b);
int32_t new_noad(void);
int32_t new_style(small_number s);
int32_t new_choice(void);
void show_info(void);
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
int32_t max_hyphenatable_length(void);
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
int32_t vert_break(int32_t p, scaled_t h, scaled_t d);
bool do_marks(small_number a, small_number l, int32_t q);
int32_t vsplit(int32_t n, scaled_t h);
void print_totals(void);
void box_error(eight_bits n);
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
void box_end(int32_t box_context);
void begin_box(int32_t box_context);
void scan_box(int32_t box_context);
void package(small_number c);
small_number norm_min(int32_t h);
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
void scan_math(int32_t p);
void set_math_char(int32_t c);
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

/* xetex-errors */

void error(void);
NORETURN void fatal_error(const char* s);
NORETURN void overflow(const char* s, int32_t n);
NORETURN void confusion(const char* s);
NORETURN void pdf_error(const char* t, const char* p);

/* xetex-math */

void initialize_math_variables(void);
void init_math(void);
void after_math(void);
void start_eq_no(void);
void math_limit_switch(void);
void math_radical(void);
void math_ac(void);
void append_choices(void);
int32_t fin_mlist(int32_t p);
void build_choices(void);
void sub_sup(void);
void math_fraction(void);
void math_left_right(void);
void flush_math(void);

/* xetex-output */

// Duplicate messages printed to log/terminal into a warning diagnostic buffer,
// until a call capture_to_diagnostic(0). A standard usage of this is
//
//     ttbc_diagnostic_t *warning = diagnostic_begin_capture_warning_here();
//
//     ... XeTeX prints some errors using print_* functions ...
//
//     capture_to_diagnostic(NULL);
//
// The current file and line number information are prefixed to the captured
// output.
//
// NOTE: the only reason there isn't also an _error_ version of this function is
// that we haven't yet wired up anything that uses it.
ttbc_diagnostic_t *diagnostic_begin_capture_warning_here(void);

// A lower-level API to begin or end the capture of messages into the diagnostic
// buffer. You can start capture by obtaining a diagnostic_t and passing it to
// this function -- however, the other functions in this API generally do this
// for you. Complete capture by passing NULL. Either way, if a capture is in
// progress when this function is called, it will be completed and reported.
void capture_to_diagnostic(ttbc_diagnostic_t *diagnostic);

// A replacement for xetex print_file_line+print_nl_ctr blocks. e.g. Replace
//
//     if (file_line_error_style_p)
//         print_file_line();
//     else
//         print_nl_cstr("! ");
//     print_cstr("Cannot use ");
//
// with
//
//     ttbc_diagnostic_t *errmsg = error_here_with_diagnostic("Cannot use ");
//
// This function calls capture_to_diagnostic(errmsg) to begin diagnostic
// capture. You must call capture_to_diagnostic(NULL) to mark the capture as
// complete.
ttbc_diagnostic_t *error_here_with_diagnostic(const char* message);

void print_ln(void);
void print_raw_char(UTF16_code s, bool incr_offset);
void print_char(int32_t s);
void print(int32_t s);
void print_cstr(const char* s);
void print_nl(str_number s);
void print_nl_cstr(const char* s);
void print_esc(str_number s);
void print_esc_cstr(const char* s);
void print_int(int32_t n);
void print_cs(int32_t p);
void sprint_cs(int32_t p);
void print_file_name(int32_t n, int32_t a, int32_t e);
void print_size(int32_t s);
void print_write_whatsit(const char* s, int32_t p);
void print_native_word(int32_t p);
void print_sa_num(int32_t q);
void print_file_line(void);
void print_two(int32_t n);
void print_hex(int32_t n);
void print_roman_int(int32_t n);
void print_current_string(void);
void print_scaled(scaled_t s);

/* xetex-pagebuilder */

void initialize_pagebuilder_variables(void);
void build_page(void);

/* xetex-scaledmath */

int32_t tex_round(double);
int32_t half(int32_t x);
scaled_t mult_and_add(int32_t n, scaled_t x, scaled_t y, scaled_t max_answer);
scaled_t x_over_n(scaled_t x, int32_t n);
scaled_t xn_over_d(scaled_t x, int32_t n, int32_t d);
void init_randoms(int32_t seed);
int32_t unif_rand(int32_t x);
int32_t norm_rand(void);

/* xetex-shipout */

void initialize_shipout_variables(void);
void deinitialize_shipout_variables(void);
void ship_out(int32_t p);
void finalize_dvi_file(void);

/* Inlines */

static inline bool
is_char_node(const int32_t p) {
    return p >= hi_mem_min;
}

static inline bool
is_non_discardable_node(const int32_t p) {
    return NODE_type(p) < MATH_NODE;
}

static inline void
print_c_string(const char *str) {
    /* Strings printed this way will end up in the .log as well
     * as the terminal output. */
    while (*str)
        print_char(*str++);
}

static inline pool_pointer
cur_length(void) {
    /*41: The length of the current string in the pool */
    return pool_ptr - str_start[str_ptr - TOO_BIG_CHAR];
}


/* Tectonic related functions */
void tt_cleanup(void);
tt_history_t tt_run_engine(const char *dump_name, const char *input_file_name, time_t build_date);


/* formerly xetex.h: */
/* additional declarations we want to slip in for xetex */

/* p is native_word node; g is XeTeX_use_glyph_metrics flag */
#define set_native_metrics(p,g)               measure_native_node(&(mem[p]), g)
#define set_native_glyph_metrics(p,g)         measure_native_glyph(&(mem[p]), g)
#define set_justified_native_glyphs(p)        store_justified_native_glyphs(&(mem[p]))
#define get_native_italic_correction(p)       real_get_native_italic_correction(&(mem[p]))
#define get_native_glyph_italic_correction(p) real_get_native_glyph_italic_correction(&(mem[p]))
#define get_native_glyph(p,i)                 real_get_native_glyph(&(mem[p]), i)
#define make_xdv_glyph_array_data(p)          makeXDVGlyphArrayData(&(mem[p]))
#define get_native_word_cp(p,s)               real_get_native_word_cp(&(mem[p]), s)

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

END_EXTERN_C

#include "xetex-stringpool.h"

#endif /* TECTONIC_XETEX_XETEXD_H */
