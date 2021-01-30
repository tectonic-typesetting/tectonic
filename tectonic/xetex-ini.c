/* xetex-ini.c: WEB initialization code translated to C
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-synctex.h"
#include "dpx-pdfobj.h" /* pdf_files_{init,close} */
#include "core-bindgen.h" /* FORMAT_SERIAL */

/* All the following variables are declared in xetex-xetexd.h */
bool shell_escape_enabled = false;
memory_word *eqtb;
int32_t bad;
char *name_of_file;
UTF16_code *name_of_file16;
int32_t name_length;
int32_t name_length16;
UnicodeScalar *buffer;
int32_t first;
int32_t last;
int32_t max_buf_stack;
bool in_initex_mode;
int32_t error_line;
int32_t half_error_line;
int32_t max_print_line;
int32_t max_strings;
int32_t strings_free;
int32_t string_vacancies;
int32_t pool_size;
int32_t pool_free;
int32_t font_mem_size;
int32_t font_max;
int32_t hyph_size;
int32_t trie_size;
int32_t buf_size;
int32_t stack_size;
int32_t max_in_open;
int32_t param_size;
int32_t nest_size;
int32_t save_size;
int32_t expand_depth;
int file_line_error_style_p;
int halt_on_error_p;
bool quoted_filename;
bool insert_src_special_auto;
bool insert_src_special_every_par;
bool insert_src_special_every_math;
bool insert_src_special_every_vbox;
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
int32_t tally;
int32_t term_offset;
int32_t file_offset;
UTF16_code trick_buf[256];
int32_t trick_count;
int32_t first_count;
bool doing_special;
UTF16_code *native_text;
int32_t native_text_size;
int32_t native_len;
int32_t save_native_len;
unsigned char interaction;
bool deletions_allowed;
bool set_box_allowed;
tt_history_t history;
signed char error_count;
const char* help_line[6];
unsigned char help_ptr;
bool use_err_help;
bool arith_error;
scaled_t tex_remainder;
int32_t randoms[55];
unsigned char j_random;
scaled_t random_seed;
int32_t two_to_the[31];
int32_t spec_log[29];
int32_t temp_ptr;
memory_word *mem;
int32_t lo_mem_max;
int32_t hi_mem_min;
int32_t var_used, dyn_used;
int32_t avail;
int32_t mem_end;
int32_t rover;
int32_t last_leftmost_char;
int32_t last_rightmost_char;
int32_t hlist_stack[513];
short hlist_stack_level;
int32_t first_p;
int32_t global_prev_p;
int32_t font_in_short_display;
int32_t depth_threshold;
int32_t breadth_max;
list_state_record *nest;
int32_t nest_ptr;
int32_t max_nest_stack;
list_state_record cur_list;
short shown_mode;
unsigned char old_setting;
b32x2 *hash;
int32_t hash_used;
int32_t hash_extra;
int32_t hash_top;
int32_t eqtb_top;
int32_t hash_high;
bool no_new_control_sequence;
int32_t cs_count;
b32x2 prim[501];
int32_t prim_used;
memory_word *save_stack;
int32_t save_ptr;
int32_t max_save_stack;
uint16_t cur_level;
group_code cur_group;
int32_t cur_boundary;
int32_t mag_set;
eight_bits cur_cmd;
int32_t cur_chr;
int32_t cur_cs;
int32_t cur_tok;
input_state_t *input_stack;
int32_t input_ptr;
int32_t max_in_stack;
input_state_t cur_input;
int32_t in_open;
int32_t open_parens;
UFILE **input_file;
int32_t line;
int32_t *line_stack;
str_number *source_filename_stack;
str_number *full_source_filename_stack;
unsigned char scanner_status;
int32_t warning_index;
int32_t def_ref;
int32_t *param_stack;
int32_t param_ptr;
int32_t max_param_stack;
int32_t align_state;
int32_t base_ptr;
int32_t par_loc;
int32_t par_token;
bool force_eof;
int32_t expand_depth_count;
bool is_in_csname;
int32_t cur_mark[5];
unsigned char long_state;
int32_t pstack[9];
int32_t cur_val;
int32_t cur_val1;
unsigned char cur_val_level;
small_number radix;
glue_ord cur_order;
UFILE *read_file[16];
unsigned char read_open[17];
int32_t cond_ptr;
unsigned char if_limit;
small_number cur_if;
int32_t if_line;
int32_t skip_line;
str_number cur_name;
str_number cur_area;
str_number cur_ext;
pool_pointer area_delimiter;
pool_pointer ext_delimiter;
UTF16_code file_name_quote_char;
int32_t format_default_length;
char *TEX_format_default;
bool name_in_progress;
str_number job_name;
bool log_opened;
const char* output_file_extension;
str_number texmf_log_name;
memory_word *font_info;
font_index fmem_ptr;
internal_font_number font_ptr;
b16x4 *font_check;
scaled_t *font_size;
scaled_t *font_dsize;
font_index *font_params;
str_number *font_name;
str_number *font_area;
UTF16_code *font_bc;
UTF16_code *font_ec;
int32_t *font_glue;
bool *font_used;
int32_t *hyphen_char;
int32_t *skew_char;
font_index *bchar_label;
nine_bits *font_bchar;
nine_bits *font_false_bchar;
void **font_layout_engine;
void **font_mapping;
char *font_flags;
scaled_t *font_letter_space;
void *loaded_font_mapping;
char loaded_font_flags;
scaled_t loaded_font_letter_space;
UTF16_code *mapped_text;
char *xdv_buffer;
int32_t *char_base;
int32_t *width_base;
int32_t *height_base;
int32_t *depth_base;
int32_t *italic_base;
int32_t *lig_kern_base;
int32_t *kern_base;
int32_t *exten_base;
int32_t *param_base;
b16x4 null_character;
int32_t total_pages;
scaled_t max_v;
scaled_t max_h;
int32_t max_push;
int32_t last_bop;
int32_t dead_cycles;
bool doing_leaders;
scaled_t rule_ht, rule_dp, rule_wd;
scaled_t cur_h, cur_v;
int32_t epochseconds;
int32_t microseconds;
scaled_t total_stretch[4], total_shrink[4];
int32_t last_badness;
int32_t adjust_tail;
int32_t pre_adjust_tail;
int32_t pack_begin_line;
b32x2 empty;
internal_font_number cur_f;
int32_t cur_c;
b16x4 cur_i;
int32_t cur_align;
int32_t cur_span;
int32_t cur_loop;
int32_t align_ptr;
int32_t cur_head, cur_tail;
int32_t cur_pre_head, cur_pre_tail;
int32_t just_box;
scaled_t active_width[7];
int32_t hc[4099];
internal_font_number hf;
int32_t hu[4097];
unsigned char cur_lang;
int32_t max_hyph_char;
unsigned char hyf[4097];
int32_t init_list;
bool init_lig;
bool init_lft;
small_number hyphen_passed;
int32_t cur_l, cur_r;
int32_t cur_q;
int32_t lig_stack;
bool ligature_present;
bool lft_hit, rt_hit;
trie_pointer *trie_trl;
trie_pointer *trie_tro;
uint16_t *trie_trc;
small_number hyf_distance[TRIE_OP_SIZE + 1];
small_number hyf_num[TRIE_OP_SIZE + 1];
trie_opcode hyf_next[TRIE_OP_SIZE + 1];
int32_t op_start[256];
str_number *hyph_word;
int32_t *hyph_list;
hyph_pointer *hyph_link;
int32_t hyph_count;
int32_t hyph_next;
trie_opcode trie_used[256];
unsigned char trie_op_lang[TRIE_OP_SIZE + 1];
trie_opcode trie_op_val[TRIE_OP_SIZE + 1];
int32_t trie_op_ptr;
trie_opcode max_op_used;
packed_UTF16_code *trie_c;
trie_opcode *trie_o;
trie_pointer *trie_l;
trie_pointer *trie_r;
trie_pointer trie_ptr;
trie_pointer *trie_hash;
bool *trie_taken;
trie_pointer trie_min[65536];
trie_pointer trie_max;
bool trie_not_ready;
scaled_t best_height_plus_depth;
internal_font_number main_f;
b16x4 main_i;
b16x4 main_j;
font_index main_k;
int32_t main_p;
int32_t main_pp, main_ppp;
int32_t main_h;
bool is_hyph;
int32_t space_class;
int32_t prev_class;
int32_t main_s;
int32_t bchar;
int32_t false_bchar;
bool cancel_boundary;
bool ins_disc;
int32_t cur_box;
int32_t after_token;
bool long_help_seen;
str_number format_ident;
rust_output_handle_t write_file[16];
bool write_open[18];
int32_t write_loc;
scaled_t cur_page_width;
scaled_t cur_page_height;
scaled_t cur_h_offset;
scaled_t cur_v_offset;
int32_t pdf_last_x_pos;
int32_t pdf_last_y_pos;
bool *eof_seen;
int32_t LR_ptr;
int32_t LR_problems;
small_number cur_dir;
int32_t pseudo_files;
save_pointer *grp_stack;
int32_t *if_stack;
int32_t max_reg_num;
const char* max_reg_help_line;
int32_t sa_root[8];
int32_t cur_ptr;
memory_word sa_null;
int32_t sa_chain;
uint16_t sa_level;
trie_pointer hyph_start;
trie_pointer hyph_index;
int32_t disc_ptr[4];
pool_pointer edit_name_start;
bool stop_at_space;
int32_t native_font_type_flag;
bool xtx_ligature_present;
scaled_t delta;
int synctex_enabled;
bool used_tectonic_coda_tokens;
bool semantic_pagination_enabled;
bool gave_char_warning_help;

/* These ought to live in xetex-pagebuilder.c but are shared a lot: */
int32_t page_tail;
unsigned char page_contents;
scaled_t page_so_far[8];
int32_t last_glue;
int32_t last_penalty;
scaled_t last_kern;
int32_t last_node_type;
int32_t insert_penalties;
bool output_active;

uint16_t _xeq_level_array[EQTB_SIZE - INT_BASE + 1];

#define NEG_TRIE_OP_SIZE -35111L
#define MAX_TRIE_OP 65535L

static int32_t _trie_op_hash_array[TRIE_OP_SIZE - NEG_TRIE_OP_SIZE + 1];
#define TRIE_OP_HASH(i) _trie_op_hash_array[(i) - NEG_TRIE_OP_SIZE]

static b32x2 *yhash;

#define FORMAT_HEADER_MAGIC 0x54544E43 /* "TTNC" in ASCII */
#define FORMAT_FOOTER_MAGIC 0x0000029A

/* Read and write dump files.  As distributed, these files are
   architecture dependent; specifically, BigEndian and LittleEndian
   architectures produce different files.  These routines always output
   BigEndian files.  This still does not guarantee them to be
   architecture-independent, because it is possible to make a format
   that dumps a glue ratio, i.e., a floating-point number.  Fortunately,
   none of the standard formats do that.  */

#if !defined (WORDS_BIGENDIAN)

/* This macro is always invoked as a statement.  It assumes a variable
   `temp'.  */

#define SWAP(x, y) temp = (x); (x) = (y); (y) = temp


/* Make the NITEMS items pointed at by P, each of size SIZE, be the
   opposite-endianness of whatever they are now.  */

static void
swap_items (char *p, size_t nitems, size_t size)
{
    char temp;

    switch (size) {
    case 16:
        while (nitems--) {
            SWAP (p[0], p[15]);
            SWAP (p[1], p[14]);
            SWAP (p[2], p[13]);
            SWAP (p[3], p[12]);
            SWAP (p[4], p[11]);
            SWAP (p[5], p[10]);
            SWAP (p[6], p[9]);
            SWAP (p[7], p[8]);
            p += size;
        }
        break;
    case 8:
        while (nitems--) {
            SWAP (p[0], p[7]);
            SWAP (p[1], p[6]);
            SWAP (p[2], p[5]);
            SWAP (p[3], p[4]);
            p += size;
        }
        break;
    case 4:
        while (nitems--) {
            SWAP (p[0], p[3]);
            SWAP (p[1], p[2]);
            p += size;
        }
        break;
    case 2:
        while (nitems--) {
            SWAP (p[0], p[1]);
            p += size;
        }
        break;
    case 1:
        break; /* Nothing to do. */
    default:
        _tt_abort("can't swap a %"PRIuZ"-byte item for (un)dumping", size);
    }
}
#else /* not WORDS_BIGENDIAN */
#define swap_items(a,b,c) do {} while(0)
#endif


/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
   The pointer to the stuff to write is P, and we write to the file
   OUT_FILE.  */

static void
do_dump (char *p, size_t item_size, size_t nitems, rust_output_handle_t out_file)
{
    swap_items (p, nitems, item_size);

    ssize_t r = ttstub_output_write (out_file, p, item_size * nitems);
    if (r < 0 || (size_t) r != item_size * nitems)
        _tt_abort ("could not write %"PRIuZ" %"PRIuZ"-byte item(s) to %s",
                   nitems, item_size, name_of_file);

    /* Have to restore the old contents of memory, since some of it might
       get used again.  */
    swap_items (p, nitems, item_size);
}


/* Here is the dual of the writing routine.  */

static void
do_undump (char *p, size_t item_size, size_t nitems, rust_input_handle_t in_file)
{
    ssize_t r = ttstub_input_read (in_file, p, item_size * nitems);
    if (r < 0 || (size_t) r != item_size * nitems)
        _tt_abort("could not undump %"PRIuZ" %"PRIuZ"-byte item(s) from %s",
                  nitems, item_size, name_of_file);

    swap_items (p, nitems, item_size);
}


#define dump_things(base, len) \
    do_dump ((char *) &(base), sizeof (base), (size_t) (len), fmt_out)
#define undump_things(base, len) \
    do_undump ((char *) &(base), sizeof (base), (size_t) (len), fmt_in)

/* Like do_undump, but check each value against LOW and HIGH.  The
   slowdown isn't significant, and this improves the chances of
   detecting incompatible format files.  In fact, Knuth himself noted
   this problem with Web2c some years ago, so it seems worth fixing.  We
   can't make this a subroutine because then we lose the type of BASE.  */
#define undump_checked_things(low, high, base, len)                     \
    do {                                                                \
        int i;                                                     \
        undump_things (base, len);                                      \
        for (i = 0; i < (len); i++) {                                   \
            if ((&(base))[i] < (low) || (&(base))[i] > (high)) {        \
                _tt_abort ("item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                           " <%" PRIdPTR " or >%" PRIdPTR,              \
                           i, (uintptr_t) (&(base))[i], (uintptr_t) &(base), \
                           (uintptr_t) low, (uintptr_t) high);          \
            }                                                           \
        }                                                               \
    } while (0)

/* Like undump_checked_things, but only check the upper value. We use
   this when the base type is unsigned, and thus all the values will be
   greater than zero by definition.  */
#define undump_upper_check_things(high, base, len)                      \
    do {                                                                \
        int i;                                                     \
        undump_things (base, len);                                      \
        for (i = 0; i < (len); i++) {                                   \
            if ((&(base))[i] > (high)) {                                \
                _tt_abort ("Item %u (=%" PRIdPTR ") of .fmt array at %" PRIxPTR \
                           " >%" PRIdPTR,                               \
                           i, (uintptr_t) (&(base))[i], (uintptr_t) &(base), \
                           (uintptr_t) high);                           \
            }                                                           \
        }                                                               \
    } while (0)


/* Since dump_things is a macro with a sizeof(), these all work: */
#define dump_b64(x) dump_things(x, 1)
#define dump_b32(x) dump_things(x, 1)
#define undump_b64(x) undump_things(x, 1)
#define undump_b32(x) undump_things(x, 1)

/* `dump_int' is called with constant integers, so we put them into a
   variable first.  */
#define dump_int(x)            \
    do {                       \
        int32_t x_val = (x);   \
        dump_things(x_val, 1); \
    } while (0)

#define undump_int(x) undump_things(x, 1)


#define hash_offset 514
#define sup_max_strings 2097151L /* magic constant, origin unclear */
#define sup_font_mem_size 147483647L /* magic constant, origin unclear */
#define sup_pool_size 40000000L
#define sup_string_vacancies (sup_pool_size - 23000)
#define sup_hash_extra sup_max_strings
#define inf_expand_depth 10
#define sup_expand_depth 10000000L


/*:134*//*135: */

static void
sort_avail(void)
{
    int32_t p, q, r;
    int32_t old_rover;

    p = get_node(0x40000000);
    p = mem[rover + 1].b32.s1;
    mem[rover + 1].b32.s1 = MAX_HALFWORD;
    old_rover = rover;

    /*136: */

    while (p != old_rover) {
        if (p < rover) {
            q = p;
            p = mem[q + 1].b32.s1;
            mem[q + 1].b32.s1 = rover;
            rover = q;
        } else {
            q = rover;
            while (mem[q + 1].b32.s1 < p)
                q = mem[q + 1].b32.s1;
            r = mem[p + 1].b32.s1;
            mem[p + 1].b32.s1 = mem[q + 1].b32.s1;
            mem[q + 1].b32.s1 = p;
            p = r;
        }
    }

    p = rover;

    while (mem[p + 1].b32.s1 != MAX_HALFWORD) {
        mem[mem[p + 1].b32.s1 + 1].b32.s0 = p;
        p = mem[p + 1].b32.s1;
    }

    mem[p + 1].b32.s1 = rover;
    mem[rover + 1].b32.s0 = p;
}

/*:271*//*276: */

static void
primitive(const char* ident, uint16_t c, int32_t o)
{
    int32_t prim_val;
    int len = strlen(ident);
    if (len > 1) {
        str_number s = maketexstring(ident);

        if (first + len > buf_size + 1)
            overflow("buffer size", buf_size);

        for (int i = 0; i < len; i++)
            buffer[first + i] = ident[i];

        cur_val = id_lookup(first, len);
        str_ptr--;
        pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
        hash[cur_val].s1 = s;
        prim_val = prim_lookup(s);
    } else {
        cur_val = ident[0] + SINGLE_BASE;
        prim_val = prim_lookup(ident[0]);
    }

    eqtb[cur_val].b16.s0 = LEVEL_ONE;
    eqtb[cur_val].b16.s1 = c;
    eqtb[cur_val].b32.s1 = o;
    eqtb[PRIM_EQTB_BASE + prim_val].b16.s0 = LEVEL_ONE;
    eqtb[PRIM_EQTB_BASE + prim_val].b16.s1 = c;
    eqtb[PRIM_EQTB_BASE + prim_val].b32.s1 = o;
}

/*:925*//*977: */

trie_opcode new_trie_op(small_number d, small_number n, trie_opcode v)
{
    int32_t h;
    trie_opcode u;
    int32_t l;
    h = abs(n + 313 * d + 361 * v + 1009 * cur_lang) % (TRIE_OP_SIZE - NEG_TRIE_OP_SIZE) + NEG_TRIE_OP_SIZE;
    while (true) {

        l = TRIE_OP_HASH(h);
        if (l == 0) {
            if (trie_op_ptr == TRIE_OP_SIZE)
                overflow("pattern memory ops", TRIE_OP_SIZE);
            u = trie_used[cur_lang];
            if (u == MAX_TRIE_OP)
                overflow("pattern memory ops per language",
                         MAX_TRIE_OP - MIN_TRIE_OP);
            trie_op_ptr++;
            u++;
            trie_used[cur_lang] = u;
            if (u > max_op_used)
                max_op_used = u;
            hyf_distance[trie_op_ptr] = d;
            hyf_num[trie_op_ptr] = n;
            hyf_next[trie_op_ptr] = v;
            trie_op_lang[trie_op_ptr] = cur_lang;
            TRIE_OP_HASH(h) = trie_op_ptr;
            trie_op_val[trie_op_ptr] = u;
            return u;
        }
        if ((hyf_distance[l] == d) && (hyf_num[l] == n) && (hyf_next[l] == v) && (trie_op_lang[l] == cur_lang)) {
            return trie_op_val[l];
        }
        if (h > -(int32_t) TRIE_OP_SIZE)
            h--;
        else
            h = TRIE_OP_SIZE;
    }
}

trie_pointer trie_node(trie_pointer p)
{
    trie_pointer h;
    trie_pointer q;
    h = abs(trie_c[p] + 1009 * trie_o[p] + 2718 * trie_l[p] + 3142 * trie_r[p]) % trie_size;
    while (true) {

        q = trie_hash[h];
        if (q == 0) {
            trie_hash[h] = p;
            return p;
        }
        if ((trie_c[q] == trie_c[p]) && (trie_o[q] == trie_o[p]) && (trie_l[q] == trie_l[p])
            && (trie_r[q] == trie_r[p])) {
            return q;
        }
        if (h > 0)
            h--;
        else
            h = trie_size;
    }
}

trie_pointer compress_trie(trie_pointer p)
{
    if (p == 0)
        return 0;
    else {

        trie_l[p] = compress_trie(trie_l[p]);
        trie_r[p] = compress_trie(trie_r[p]);
        return trie_node(p);
    }
}

void first_fit(trie_pointer p)
{
    trie_pointer h;
    trie_pointer z;
    trie_pointer q;
    UTF16_code c;
    trie_pointer l, r;
    int32_t /*too_big_char */ ll;
    c = trie_c[p];
    z = trie_min[c];
    while (true) {

        h = z - c;
        if (trie_max < h + max_hyph_char) {
            if (trie_size <= h + max_hyph_char)
                overflow("pattern memory", trie_size);
            do {
                trie_max++;
                trie_taken[trie_max] = false;
                trie_trl[trie_max] = trie_max + 1;
                trie_tro[trie_max] = trie_max - 1;
            } while (!(trie_max == h + max_hyph_char));
        }
        if (trie_taken[h])
            goto not_found;
        q = trie_r[p];
        while (q > 0) {

            if (trie_trl[h + trie_c[q]] == 0)
                goto not_found;
            q = trie_r[q];
        }
        goto found;
    not_found:                        /*not_found */ z = trie_trl[z];
    }
found:                        /*found *//*991: */ trie_taken[h] = true;
    trie_hash[p] = h;
    q = p;
    do {
        z = h + trie_c[q];
        l = trie_tro[z];
        r = trie_trl[z];
        trie_tro[r] = l;
        trie_trl[l] = r;
        trie_trl[z] = 0;
        if (l < max_hyph_char) {
            if (z < max_hyph_char)
                ll = z;
            else
                ll = max_hyph_char;
            do {
                trie_min[l] = r;
                l++;
            } while (!(l == ll));
        }
        q = trie_r[q];
    } while (!(q == 0 /*:991 */ ));
}

void trie_pack(trie_pointer p)
{
    trie_pointer q;
    do {
        q = trie_l[p];
        if ((q > 0) && (trie_hash[q] == 0)) {
            first_fit(q);
            trie_pack(q);
        }
        p = trie_r[p];
    } while (!(p == 0));
}

void trie_fix(trie_pointer p)
{
    trie_pointer q;
    UTF16_code c;
    trie_pointer z;
    z = trie_hash[p];
    do {
        q = trie_l[p];
        c = trie_c[p];
        trie_trl[z + c] = trie_hash[q];
        trie_trc[z + c] = c;
        trie_tro[z + c] = trie_o[p];
        if (q > 0)
            trie_fix(q);
        p = trie_r[p];
    } while (!(p == 0));
}

static void
new_patterns(void)
{
    short /*hyphenatable_length_limit 1 */ k, l;
    bool digit_sensed;
    trie_opcode v;
    trie_pointer p, q;
    bool first_child;
    UTF16_code c;

    if (trie_not_ready) {
        if (INTPAR(language) <= 0)
            cur_lang = 0;
        else if (INTPAR(language) > BIGGEST_LANG)
            cur_lang = 0;
        else
            cur_lang = INTPAR(language);

        scan_left_brace();
        k = 0;
        hyf[0] = 0;
        digit_sensed = false;

        while (true) {
            get_x_token();

            switch (cur_cmd) {
            case LETTER:
            case OTHER_CHAR:
                if (digit_sensed || cur_chr < '0'  || cur_chr > '9' ) {
                    if (cur_chr == '.' ) {
                        cur_chr = 0;
                    } else {
                        cur_chr = LC_CODE(cur_chr);

                        if (cur_chr == 0) {
                            error_here_with_diagnostic("Nonletter");
                            capture_to_diagnostic(NULL);
                            help_ptr = 1;
                            help_line[0] = "(See Appendix H.)";
                            error();
                        }
                    }

                    if (cur_chr > max_hyph_char)
                        max_hyph_char = cur_chr;

                    if (k < max_hyphenatable_length()) {
                        k++;
                        hc[k] = cur_chr;
                        hyf[k] = 0;
                        digit_sensed = false;
                    }
                } else if (k < max_hyphenatable_length()) {
                    hyf[k] = cur_chr - 48;
                    digit_sensed = true;
                }
                break;

            case SPACER:
            case RIGHT_BRACE:
                if (k > 0) { /*998:*/
                    if (hc[1] == 0)
                        hyf[0] = 0;
                    if (hc[k] == 0)
                        hyf[k] = 0;

                    l = k;
                    v = MIN_TRIE_OP;

                    while (true) {
                        if (hyf[l] != 0)
                            v = new_trie_op(k - l, hyf[l], v);
                        if (l > 0)
                            l--;
                        else
                            break;
                    }

                    q = 0;
                    hc[0] = cur_lang;

                    while (l <= k) {
                        c = hc[l];
                        l++;
                        p = trie_l[q];
                        first_child = true;

                        while (p > 0 && c > trie_c[p]) {
                            q = p;
                            p = trie_r[q];
                            first_child = false;
                        }

                        if (p == 0 || c < trie_c[p]) { /*999:*/
                            if (trie_ptr == trie_size)
                                overflow("pattern memory", trie_size);
                            trie_ptr++;
                            trie_r[trie_ptr] = p;
                            p = trie_ptr;
                            trie_l[p] = 0;
                            if (first_child)
                                trie_l[q] = p;
                            else
                                trie_r[q] = p;
                            trie_c[p] = c;
                            trie_o[p] = MIN_TRIE_OP;
                        }

                        q = p;
                    }

                    if (trie_o[q] != MIN_TRIE_OP) {
                        error_here_with_diagnostic("Duplicate pattern");
                        capture_to_diagnostic(NULL);
                        help_ptr = 1;
                        help_line[0] = "(See Appendix H.)";
                        error();
                    }

                    trie_o[q] = v;
                }

                if (cur_cmd == RIGHT_BRACE)
                    goto done;

                k = 0;
                hyf[0] = 0;
                digit_sensed = false;
                break;

            default:
                error_here_with_diagnostic("Bad ");
                print_esc_cstr("patterns");
                capture_to_diagnostic(NULL);
                help_ptr = 1;
                help_line[0] = "(See Appendix H.)";
                error();
                break;
            }
        }

    done: /*:996*/
        if (INTPAR(saving_hyphs) > 0) { /*1643:*/
            c = cur_lang;
            first_child = false;
            p = 0;

            do {
                q = p;
                p = trie_r[q];
            } while (!(p == 0 || c <= trie_c[p]));

            if (p == 0 || c < trie_c[p]) { /*999:*/
                if (trie_ptr == trie_size)
                    overflow("pattern memory", trie_size);
                trie_ptr++;
                trie_r[trie_ptr] = p;
                p = trie_ptr;
                trie_l[p] = 0;
                if (first_child)
                    trie_l[q] = p;
                else
                    trie_r[q] = p;
                trie_c[p] = c;
                trie_o[p] = MIN_TRIE_OP;
            }

            q = p;
            p = trie_l[q];
            first_child = true;

            for (c = 0; c <= 255; c++) {
                if (LC_CODE(c) > 0 || (c == 255 && first_child)) {
                    if (p == 0) { /*999:*/
                        if (trie_ptr == trie_size)
                            overflow("pattern memory", trie_size);
                        trie_ptr++;
                        trie_r[trie_ptr] = p;
                        p = trie_ptr;
                        trie_l[p] = 0;
                        if (first_child)
                            trie_l[q] = p;
                        else
                            trie_r[q] = p;
                        trie_c[p] = c;
                        trie_o[p] = MIN_TRIE_OP;
                    } else {
                        trie_c[p] = c;
                    }

                    trie_o[p] = LC_CODE(c);
                    q = p;
                    p = trie_r[q];
                    first_child = false;
                }
            }

            if (first_child)
                trie_l[q] = 0;
            else
                trie_r[q] = 0; /*:1644*/
        }
    } else {
        error_here_with_diagnostic("Too late for ");
        print_esc_cstr("patterns");
        capture_to_diagnostic(NULL);
        help_ptr = 1;
        help_line[0] = "All patterns must be given before typesetting begins.";
        error();

        mem[GARBAGE].b32.s1 = scan_toks(false, false);
        flush_list(def_ref);
    }
}

void init_trie(void)
{
    trie_pointer p;
    int32_t j, k, t;
    trie_pointer r, s;
    max_hyph_char++;
    op_start[0] = -(int32_t) MIN_TRIE_OP;
    {
        register int32_t for_end;
        j = 1;
        for_end = BIGGEST_LANG;
        if (j <= for_end)
            do
                op_start[j] = op_start[j - 1] + trie_used[j - 1];
            while (j++ < for_end);
    }
    {
        register int32_t for_end;
        j = 1;
        for_end = trie_op_ptr;
        if (j <= for_end)
            do
                TRIE_OP_HASH(j) = op_start[trie_op_lang[j]] + trie_op_val[j];
            while (j++ < for_end);
    }
    {
        register int32_t for_end;
        j = 1;
        for_end = trie_op_ptr;
        if (j <= for_end)
            do
                while (TRIE_OP_HASH(j) > j) {

                    k = TRIE_OP_HASH(j);
                    t = hyf_distance[k];
                    hyf_distance[k] = hyf_distance[j];
                    hyf_distance[j] = t;
                    t = hyf_num[k];
                    hyf_num[k] = hyf_num[j];
                    hyf_num[j] = t;
                    t = hyf_next[k];
                    hyf_next[k] = hyf_next[j];
                    hyf_next[j] = t;
                    TRIE_OP_HASH(j) = TRIE_OP_HASH(k);
                    TRIE_OP_HASH(k) = k;
                }
            while (j++ < for_end);
    }
    {
        register int32_t for_end;
        p = 0;
        for_end = trie_size;
        if (p <= for_end)
            do
                trie_hash[p] = 0;
            while (p++ < for_end);
    }
    trie_r[0] = compress_trie(trie_r[0]);
    trie_l[0] = compress_trie(trie_l[0]);
    {
        register int32_t for_end;
        p = 0;
        for_end = trie_ptr;
        if (p <= for_end)
            do
                trie_hash[p] = 0;
            while (p++ < for_end);
    }
    {
        register int32_t for_end;
        p = 0;
        for_end = BIGGEST_CHAR;
        if (p <= for_end)
            do
                trie_min[p] = p + 1;
            while (p++ < for_end);
    }
    trie_trl[0] = 1;
    trie_max = 0 /*:987 */ ;
    if (trie_l[0] != 0) {
        first_fit(trie_l[0]);
        trie_pack(trie_l[0]);
    }
    if (trie_r[0] != 0) {       /*1645: */
        if (trie_l[0] == 0) {
            register int32_t for_end;
            p = 0;
            for_end = 255;
            if (p <= for_end)
                do
                    trie_min[p] = p + 2;
                while (p++ < for_end);
        }
        first_fit(trie_r[0]);
        trie_pack(trie_r[0]);
        hyph_start = trie_hash[trie_r[0]];
    }
    if (trie_max == 0) {
        {
            register int32_t for_end;
            r = 0;
            for_end = max_hyph_char;
            if (r <= for_end)
                do {
                    trie_trl[r] = 0;
                    trie_tro[r] = MIN_TRIE_OP;
                    trie_trc[r] = 0;
                }
                while (r++ < for_end);
        }
        trie_max = max_hyph_char;
    } else {

        if (trie_r[0] > 0)
            trie_fix(trie_r[0]);
        if (trie_l[0] > 0)
            trie_fix(trie_l[0]);
        r = 0;
        do {
            s = trie_trl[r];
            {
                trie_trl[r] = 0;
                trie_tro[r] = MIN_TRIE_OP;
                trie_trc[r] = 0;
            }
            r = s;
        } while (!(r > trie_max));
    }
    trie_trc[0] = '?' ;
    trie_not_ready = false;
}

/*:1001*/

static void
new_hyph_exceptions(void)
{
    short /*hyphenatable_length_limit 1 */ n;
    short /*hyphenatable_length_limit 1 */ j;
    hyph_pointer h;
    str_number k;
    int32_t p;
    int32_t q;
    str_number s;
    pool_pointer u, v;

    scan_left_brace();

    if (INTPAR(language) <= 0)
        cur_lang = 0;
    else if (INTPAR(language) > BIGGEST_LANG)
        cur_lang = 0;
    else
        cur_lang = INTPAR(language);

    if (trie_not_ready) {
        hyph_index = 0;
        goto not_found1;
    }

    if (trie_trc[hyph_start + cur_lang] != cur_lang)
        hyph_index = 0;
    else
        hyph_index = trie_trl[hyph_start + cur_lang];

not_found1: /*970:*/
    n = 0;
    p = TEX_NULL;

    while (true) {
        get_x_token();

    reswitch:
        switch (cur_cmd) {
        case LETTER:
        case OTHER_CHAR:
        case CHAR_GIVEN:
            if (cur_chr == '-' ) { /*973:*/
                if (n < max_hyphenatable_length()) {
                    q = get_avail();
                    mem[q].b32.s1 = p;
                    mem[q].b32.s0 = n;
                    p = q;
                }
            } else {
                if (hyph_index == 0 || cur_chr > 255)
                    hc[0] = LC_CODE(cur_chr);
                else if (trie_trc[hyph_index + cur_chr] != cur_chr)
                    hc[0] = 0;
                else
                    hc[0] = trie_tro[hyph_index + cur_chr];

                if (hc[0] == 0) {
                    error_here_with_diagnostic("Not a letter");
                    capture_to_diagnostic(NULL);
                    help_ptr = 2;
                    help_line[1] = "Letters in \\hyphenation words must have \\lccode>0.";
                    help_line[0] = "Proceed; I'll ignore the character I just read.";
                    error();
                } else if (n < max_hyphenatable_length()) {
                    n++;

                    if (hc[0] < 65536L) {
                        hc[n] = hc[0];
                    } else {
                        hc[n] = (hc[0] - 65536L) / 1024 + 55296L;
                        n++;
                        hc[n] = hc[0] % 1024 + 56320L;
                    }
                }
            }
            break;

        case CHAR_NUM:
            scan_char_num();
            cur_chr = cur_val;
            cur_cmd = CHAR_GIVEN;
            goto reswitch;
            break;

        case SPACER:
        case RIGHT_BRACE:
            if (n > 1) { /*974:*/
                n++;
                hc[n] = cur_lang;
                if (pool_ptr + n > pool_size)
                    overflow("pool size", pool_size - init_pool_ptr);
                h = 0;

                for (j = 1; j <= n; j++) {
                    h = (h + h + hc[j]) % HYPH_PRIME;
                    str_pool[pool_ptr] = hc[j];
                    pool_ptr++;
                }

                s = make_string();

                if (hyph_next <= HYPH_PRIME) {
                    while (hyph_next > 0 && hyph_word[hyph_next - 1] > 0)
                        hyph_next--;
                }

                if (hyph_count == hyph_size || hyph_next == 0)
                    overflow("exception dictionary", hyph_size);

                hyph_count++;

                while (hyph_word[h] != 0) {
                    k = hyph_word[h];
                    if (length(k) != length(s))
                        goto not_found;

                    u = str_start[(k) - 65536L];
                    v = str_start[(s) - 65536L];

                    do {
                        if (str_pool[u] != str_pool[v])
                            goto not_found;
                        u++;
                        v++;
                    } while (u != str_start[(k + 1) - 65536L]);

                    str_ptr--;
                    pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
                    s = hyph_word[h];
                    hyph_count--;
                    goto found;

                not_found: /*:976*/
                    if (hyph_link[h] == 0) {
                        hyph_link[h] = hyph_next;
                        if (hyph_next >= hyph_size)
                            hyph_next = HYPH_PRIME;
                        if (hyph_next > HYPH_PRIME)
                            hyph_next++;
                    }
                    h = hyph_link[h] - 1;
                }

            found:
                hyph_word[h] = s;
                hyph_list[h] = p; /*:975*/
            }

            if (cur_cmd == RIGHT_BRACE)
                return;

            n = 0;
            p = TEX_NULL;
            break;

        default:
            error_here_with_diagnostic("Improper ");
            print_esc_cstr("hyphenation");
            print_cstr(" will be flushed");
            capture_to_diagnostic(NULL);
            help_ptr = 2;
            help_line[1] = "Hyphenation exceptions must contain only letters";
            help_line[0] = "and hyphens. But continue; I'll forgive and forget.";
            error();
            break;
        }
    }
}


void
prefixed_command(void)
{
    small_number a;
    internal_font_number f;
    int32_t j;
    font_index k;
    int32_t p, q;
    int32_t n;
    bool e;

    a = 0;

    while (cur_cmd == PREFIX) {
        if (!odd(a / cur_chr))
            a = a + cur_chr;

        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        if (cur_cmd <= MAX_NON_PREFIXED_COMMAND) { /*1247:*/
            error_here_with_diagnostic("You can't use a prefix with `");
            print_cmd_chr(cur_cmd, cur_chr);
            print_char('\'');
            capture_to_diagnostic(NULL);
            help_ptr = 1;
            help_line[0] = "I'll pretend you didn't say \\long or \\outer or \\global or \\protected.";
            back_error();
            return;
        }

        if (INTPAR(tracing_commands) > 2) {
            show_cur_cmd_chr();
        }
    }

    if (a >= 8) {
        j = PROTECTED_TOKEN;
        a = a - 8;
    } else {
        j = 0;
    }

    if (cur_cmd != DEF && (a % 4 != 0 || j != 0)) {
        error_here_with_diagnostic("You can't use `");
        print_esc_cstr("long");
        print_cstr("' or `");
        print_esc_cstr("outer");
        print_cstr("' or `");
        print_esc_cstr("protected");
        print_cstr("' with `");
        print_cmd_chr(cur_cmd, cur_chr);
        print_char('\'');
        capture_to_diagnostic(NULL);
        help_ptr = 1;
        help_line[0] = "I'll pretend you didn't say \\long or \\outer or \\protected here.";
        error();
    }

    if (INTPAR(global_defs) != 0) {
        if (INTPAR(global_defs) < 0) {
            if (a >= 4)
                a = a - 4;
        } else {
            if (a < 4)
                a = a + 4;
        }
    }

    switch (cur_cmd) { /*1252:*/
    case SET_FONT:
        if (a >= 4)
            geq_define(CUR_FONT_LOC, DATA, cur_chr);
        else
            eq_define(CUR_FONT_LOC, DATA, cur_chr);
        break;

    case DEF:
        if (odd(cur_chr) && a < 4 && INTPAR(global_defs) >= 0)
            a = a + 4;

        e = (cur_chr >= 2);
        get_r_token();
        p = cur_cs;
        q = scan_toks(true, e);

        if (j != 0) {
            q = get_avail();
            mem[q].b32.s0 = j;
            mem[q].b32.s1 = mem[def_ref].b32.s1;
            mem[def_ref].b32.s1 = q;
        }

        if (a >= 4)
            geq_define(p, CALL + (a % 4), def_ref);
        else
            eq_define(p, CALL + (a % 4), def_ref);
        break;

    case LET:
        n = cur_chr;
        get_r_token();
        p = cur_cs;

        if (n == NORMAL) {
            do {
                get_token();
            } while (cur_cmd == SPACER);

            if (cur_tok == (OTHER_TOKEN + '=' )) {
                get_token();
                if (cur_cmd == SPACER)
                    get_token();
            }
        } else {
            get_token();
            q = cur_tok;
            get_token();
            back_input();
            cur_tok = q;
            back_input();
        }

        if (cur_cmd >= CALL) {
            mem[cur_chr].b32.s0++;
        } else if (cur_cmd == REGISTER || cur_cmd == TOKS_REGISTER) {
            if (cur_chr < 0 || cur_chr > 19) /* 19 = lo_mem_stat_max, I think */
                mem[cur_chr + 1].b32.s0++;
        }

        if (a >= 4)
            geq_define(p, cur_cmd, cur_chr);
        else
            eq_define(p, cur_cmd, cur_chr);
        break;

    case SHORTHAND_DEF:
        if (cur_chr == CHAR_SUB_DEF_CODE) {
            scan_char_num();
            p = CHAR_SUB_CODE_BASE + cur_val;
            scan_optional_equals();
            scan_char_num();
            n = cur_val;
            scan_char_num();
            if (INTPAR(tracing_char_sub_def) > 0) {
                begin_diagnostic();
                print_nl_cstr("New character substitution: ");
                print(p - CHAR_SUB_CODE_BASE);
                print_cstr(" = ");
                print(n);
                print_char(' ');
                print(cur_val);
                end_diagnostic(false);
            }

            n = n * 256 + cur_val;

            if (a >= 4)
                geq_define(p, DATA, n);
            else
                eq_define(p, DATA, n);

            if ((p - CHAR_SUB_CODE_BASE) < INTPAR(char_sub_def_min)) {
                if (a >= 4)
                    geq_word_define(INT_BASE + INT_PAR__char_sub_def_min, p - CHAR_SUB_CODE_BASE);
                else
                    eq_word_define(INT_BASE + INT_PAR__char_sub_def_min, p - CHAR_SUB_CODE_BASE);
            }

            if ((p - CHAR_SUB_CODE_BASE) > INTPAR(char_sub_def_max)) {
                if (a >= 4)
                    geq_word_define(INT_BASE + INT_PAR__char_sub_def_max, p - CHAR_SUB_CODE_BASE);
                else
                    eq_word_define(INT_BASE + INT_PAR__char_sub_def_max, p - CHAR_SUB_CODE_BASE);
            }
        } else {
            n = cur_chr;
            get_r_token();
            p = cur_cs;

            if (a >= 4)
                geq_define(p, RELAX, TOO_BIG_USV);
            else
                eq_define(p, RELAX, TOO_BIG_USV);

            scan_optional_equals();

            switch (n) {
            case CHAR_DEF_CODE:
                scan_usv_num();
                if (a >= 4)
                    geq_define(p, CHAR_GIVEN, cur_val);
                else
                    eq_define(p, CHAR_GIVEN, cur_val);
                break;

            case MATH_CHAR_DEF_CODE:
                scan_fifteen_bit_int();
                if (a >= 4)
                    geq_define(p, MATH_GIVEN, cur_val);
                else
                    eq_define(p, MATH_GIVEN, cur_val);
                break;

            case XETEX_MATH_CHAR_NUM_DEF_CODE:
                scan_xetex_math_char_int();
                if (a >= 4)
                    geq_define(p, XETEX_MATH_GIVEN, cur_val);
                else
                    eq_define(p, XETEX_MATH_GIVEN, cur_val);
                break;

            case XETEX_MATH_CHAR_DEF_CODE:
                scan_math_class_int();
                n = set_class(cur_val);
                scan_math_fam_int();
                n = n + set_family(cur_val);
                scan_usv_num();
                n = n + cur_val;
                if (a >= 4)
                    geq_define(p, XETEX_MATH_GIVEN, n);
                else
                    eq_define(p, XETEX_MATH_GIVEN, n);
                break;

            default:
                scan_register_num();

                if (cur_val > 255) {
                    j = n - 2;

                    if (j > MU_VAL)
                        j = TOK_VAL;

                    find_sa_element(j, cur_val, true);
                    mem[cur_ptr + 1].b32.s0++;

                    if (j == TOK_VAL)
                        j = TOKS_REGISTER;
                    else
                        j = REGISTER;

                    if (a >= 4)
                        geq_define(p, j, cur_ptr);
                    else
                        eq_define(p, j, cur_ptr);
                } else {
                    switch (n) {
                    case COUNT_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_INT, COUNT_BASE + cur_val);
                        break;
                    case DIMEN_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_DIMEN, SCALED_BASE + cur_val);
                        break;
                    case SKIP_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_GLUE, SKIP_BASE + cur_val);
                        break;
                    case MU_SKIP_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_MU_GLUE, MU_SKIP_BASE + cur_val);
                        break;
                    case TOKS_DEF_CODE:
                        if (a >= 4)
                            geq_define(p, ASSIGN_TOKS, TOKS_BASE + cur_val);
                        else
                            eq_define(p, ASSIGN_TOKS, TOKS_BASE + cur_val);
                        break;
                    }
                }
                break;
            }
        }
        break;

    case READ_TO_CS:
        j = cur_chr;
        scan_int();
        n = cur_val;
        if (!scan_keyword("to")) {
            error_here_with_diagnostic("Missing `to' inserted");
            capture_to_diagnostic(NULL);
            help_ptr = 2;
            help_line[1] = "You should have said `\\read<number> to \\cs'.";
            help_line[0] = "I'm going to look for the \\cs now.";
            error();
        }

        get_r_token();
        p = cur_cs;
        read_toks(n, p, j);

        if (a >= 4)
            geq_define(p, CALL, cur_val);
        else
            eq_define(p, CALL, cur_val);
        break;

    case TOKS_REGISTER:
    case ASSIGN_TOKS:
        q = cur_cs;
        e = false;

        if (cur_cmd == TOKS_REGISTER) {
            if (cur_chr == 0) {
                scan_register_num();
                if (cur_val > 255) {
                    find_sa_element(TOK_VAL, cur_val, true);
                    cur_chr = cur_ptr;
                    e = true;
                } else {
                    cur_chr = TOKS_BASE + cur_val;
                }
            } else {
                e = true;
            }
        } else if (cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char) {
            scan_char_class_not_ignored();
            cur_ptr = cur_val;
            scan_char_class_not_ignored();
            find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, true);
            cur_chr = cur_ptr;
            e = true;
        }

        p = cur_chr;
        scan_optional_equals();

        do {
            get_x_token();
        } while (cur_cmd == SPACER || cur_cmd == RELAX);

        if (cur_cmd != LEFT_BRACE) { /*1262:*/
            if (cur_cmd == TOKS_REGISTER || cur_cmd == ASSIGN_TOKS) {
                if (cur_cmd == TOKS_REGISTER) {
                    if (cur_chr == 0) {
                        scan_register_num();
                        if (cur_val < 256) {
                            q = TOKS_REG(cur_val);
                        } else {
                            find_sa_element(TOK_VAL, cur_val, false);
                            if (cur_ptr == TEX_NULL)
                                q = TEX_NULL;
                            else
                                q = mem[cur_ptr + 1].b32.s1;
                        }
                    } else {
                        q = mem[cur_chr + 1].b32.s1;
                    }
                } else if (cur_chr == LOCAL_BASE + LOCAL__xetex_inter_char) {
                    scan_char_class_not_ignored();
                    cur_ptr = cur_val;
                    scan_char_class_not_ignored();
                    find_sa_element(INTER_CHAR_VAL, cur_ptr * CHAR_CLASS_LIMIT + cur_val, false);
                    if (cur_ptr == TEX_NULL)
                        q = TEX_NULL;
                    else
                        q = mem[cur_ptr + 1].b32.s1;
                } else {
                    q = eqtb[cur_chr].b32.s1;
                }

                if (q == TEX_NULL) {
                    if (e) {
                        if (a >= 4)
                            gsa_def(p, TEX_NULL);
                        else
                            sa_def(p, TEX_NULL);
                    } else if (a >= 4) {
                        geq_define(p, UNDEFINED_CS, TEX_NULL);
                    } else {
                        eq_define(p, UNDEFINED_CS, TEX_NULL);
                    }
                } else {
                    mem[q].b32.s0++;
                    if (e) {
                        if (a >= 4)
                            gsa_def(p, q);
                        else
                            sa_def(p, q);
                    } else if (a >= 4) {
                        geq_define(p, CALL, q);
                    } else {
                        eq_define(p, CALL, q);
                    }
                }

                goto done;
            }
        }

        back_input();
        cur_cs = q;
        q = scan_toks(false, false);

        if (mem[def_ref].b32.s1 == TEX_NULL) {
            if (e) {
                if (a >= 4)
                    gsa_def(p, TEX_NULL);
                else
                    sa_def(p, TEX_NULL);
            } else if (a >= 4) {
                geq_define(p, UNDEFINED_CS, TEX_NULL);
            } else {
                eq_define(p, UNDEFINED_CS, TEX_NULL);
            }

            mem[def_ref].b32.s1 = avail;
            avail = def_ref;
        } else {
            if (p == LOCAL_BASE + LOCAL__output_routine && !e) {
                mem[q].b32.s1 = get_avail();
                q = LLIST_link(q);
                mem[q].b32.s0 = (RIGHT_BRACE_TOKEN + 125);
                q = get_avail();
                mem[q].b32.s0 = (LEFT_BRACE_TOKEN + 123);
                mem[q].b32.s1 = mem[def_ref].b32.s1;
                mem[def_ref].b32.s1 = q;
            }

            if (e) {
                if (a >= 4)
                    gsa_def(p, def_ref);
                else
                    sa_def(p, def_ref);
            } else if (a >= 4) {
                geq_define(p, CALL, def_ref);
            } else {
                eq_define(p, CALL, def_ref);
            }
        }

        break;

    case ASSIGN_INT:
        p = cur_chr;
        scan_optional_equals();
        scan_int();
        if (a >= 4)
            geq_word_define(p, cur_val);
        else
            eq_word_define(p, cur_val);
        break;

    case ASSIGN_DIMEN:
        p = cur_chr;
        scan_optional_equals();
        scan_dimen(false, false, false);
        if (a >= 4)
            geq_word_define(p, cur_val);
        else
            eq_word_define(p, cur_val);
        break;

    case ASSIGN_GLUE:
    case ASSIGN_MU_GLUE:
        p = cur_chr;
        n = cur_cmd;
        scan_optional_equals();
        if (n == ASSIGN_MU_GLUE)
            scan_glue(MU_VAL);
        else
            scan_glue(GLUE_VAL);
        trap_zero_glue();
        if (a >= 4)
            geq_define(p, GLUE_REF, cur_val);
        else
            eq_define(p, GLUE_REF, cur_val);
        break;

    case XETEX_DEF_CODE:
        if (cur_chr == SF_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            n = SF_CODE(cur_val) % 65536L;
            scan_optional_equals();
            scan_char_class();
            if (a >= 4)
                geq_define(p, DATA, cur_val * 65536L + n);
            else
                eq_define(p, DATA, cur_val * 65536L + n);
        } else if (cur_chr == MATH_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_xetex_math_char_int();
            if (a >= 4)
                geq_define(p, DATA, cur_val);
            else
                eq_define(p, DATA, cur_val);
        } else if (cur_chr == MATH_CODE_BASE + 1) {
            p = cur_chr - 1;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_math_class_int();
            n = set_class(cur_val);
            scan_math_fam_int();
            n = n + set_family(cur_val);
            scan_usv_num();
            n = n + cur_val;
            if (a >= 4)
                geq_define(p, DATA, n);
            else
                eq_define(p, DATA, n);
        } else if (cur_chr == DEL_CODE_BASE) {
            p = cur_chr;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            scan_int();
            if (a >= 4)
                geq_word_define(p, cur_val);
            else
                eq_word_define(p, cur_val);
        } else {
            p = cur_chr - 1;
            scan_usv_num();
            p = p + cur_val;
            scan_optional_equals();
            n = 0x40000000; /* "extended delimiter code flag" */
            scan_math_fam_int();
            n = n + cur_val * 0x200000; /* "extended delimiter code family */
            scan_usv_num();
            n = n + cur_val;
            if (a >= 4)
                geq_word_define(p, n);
            else
                eq_word_define(p, n);
        }
        break;

    case DEF_CODE:
        if (cur_chr == CAT_CODE_BASE)
            n = MAX_CHAR_CODE;
        else if (cur_chr == MATH_CODE_BASE)
            n = 0x8000;
        else if (cur_chr == SF_CODE_BASE)
            n = 0x7FFF;
        else if (cur_chr == DEL_CODE_BASE)
            n = 0xFFFFFF;
        else
            n = BIGGEST_USV; /*:1268 */

        p = cur_chr;
        scan_usv_num();
        p = p + cur_val;
        scan_optional_equals();
        scan_int();

        if ((cur_val < 0 && p < DEL_CODE_BASE) || cur_val > n) {
            error_here_with_diagnostic("Invalid code (");
            print_int(cur_val);
            if (p < DEL_CODE_BASE)
                print_cstr("), should be in the range 0..");
            else
                print_cstr("), should be at most ");
            print_int(n);
            capture_to_diagnostic(NULL);
            help_ptr = 1;
            help_line[0] = "I'm going to use 0 instead of that illegal code value.";
            error();
            cur_val = 0;
        }

        if (p < MATH_CODE_BASE) {
            if (p >= SF_CODE_BASE) {
                n = eqtb[p].b32.s1 / 65536L;
                if (a >= 4)
                    geq_define(p, DATA, n * 65536L + cur_val);
                else
                    eq_define(p, DATA, n * 65536L + cur_val);
            } else if (a >= 4) {
                geq_define(p, DATA, cur_val);
            } else {
                eq_define(p, DATA, cur_val);
            }
        } else if (p < DEL_CODE_BASE) {
            if (cur_val == 32768L)
                cur_val = ACTIVE_MATH_CHAR;
            else
                cur_val = set_class(cur_val / 4096) + set_family((cur_val % 4096) / 256) + (cur_val % 256);

            if (a >= 4)
                geq_define(p, DATA, cur_val);
            else
                eq_define(p, DATA, cur_val);
        } else if (a >= 4) {
            geq_word_define(p, cur_val);
        } else {
            eq_word_define(p, cur_val);
        }
        break;

    case DEF_FAMILY:
        p = cur_chr;
        scan_math_fam_int();
        p = p + cur_val;
        scan_optional_equals();
        scan_font_ident();
        if (a >= 4)
            geq_define(p, DATA, cur_val);
        else
            eq_define(p, DATA, cur_val);
        break;

    case REGISTER:
    case ADVANCE:
    case MULTIPLY:
    case DIVIDE:
        do_register_command(a);
        break;

    case SET_BOX:
        scan_register_num();
        if (a >= 4)
            n = GLOBAL_BOX_FLAG + cur_val;
        else
            n = BOX_FLAG + cur_val;

        scan_optional_equals();

        if (set_box_allowed) {
            scan_box(n);
        } else {
            error_here_with_diagnostic("Improper ");
            print_esc_cstr("setbox");
            capture_to_diagnostic(NULL);
            help_ptr = 2;
            help_line[1] = "Sorry, \\setbox is not allowed after \\halign in a display,";
            help_line[0] = "or between \\accent and an accented character.";
            error();
        }
        break;

    case SET_AUX:
        alter_aux();
        break;

    case SET_PREV_GRAF:
        alter_prev_graf();
        break;

    case SET_PAGE_DIMEN:
        alter_page_so_far();
        break;

    case SET_PAGE_INT:
        alter_integer();
        break;

    case SET_BOX_DIMEN:
        alter_box_dimen();
        break;

    case SET_SHAPE:
        q = cur_chr;
        scan_optional_equals();
        scan_int();
        n = cur_val;

        if (n <= 0) {
            p = TEX_NULL;
        } else if (q > LOCAL_BASE + LOCAL__par_shape) {
            n = (cur_val / 2) + 1;
            p = get_node(2 * n + 1);
            mem[p].b32.s0 = n;
            n = cur_val;
            mem[p + 1].b32.s1 = n;

            for (j = p + 2; j <= p + n + 1; j++) {
                scan_int();
                mem[j].b32.s1 = cur_val;
            }

            if (!odd(n))
                mem[p + n + 2].b32.s1 = 0;
        } else {
            p = get_node(2 * n + 1);
            mem[p].b32.s0 = n;

            for (j = 1; j <= n; j++) {
                scan_dimen(false, false, false);
                mem[p + 2 * j - 1].b32.s1 = cur_val;
                scan_dimen(false, false, false);
                mem[p + 2 * j].b32.s1 = cur_val;
            }
        }

        if (a >= 4)
            geq_define(q, SHAPE_REF, p);
        else
            eq_define(q, SHAPE_REF, p);
        break;

    case HYPH_DATA:
        if (cur_chr == 1) {
            if (in_initex_mode) {
                new_patterns();
                goto done;
            }

            error_here_with_diagnostic("Patterns can be loaded only by INITEX");
            capture_to_diagnostic(NULL);
            help_ptr = 0;
            error();

            do {
                get_token();
            } while (cur_cmd != RIGHT_BRACE);

            return;
        } else {
            new_hyph_exceptions();
            goto done;
        }
        break;

    case ASSIGN_FONT_DIMEN:
        find_font_dimen(true);
        k = cur_val;
        scan_optional_equals();
        scan_dimen(false, false, false);
        font_info[k].b32.s1 = cur_val;
        break;

    case ASSIGN_FONT_INT:
        n = cur_chr;
        scan_font_ident();
        f = cur_val;

        if (n < 2) {
            scan_optional_equals();
            scan_int();
            if (n == 0)
                hyphen_char[f] = cur_val;
            else
                skew_char[f] = cur_val;
        } else {
            if (font_area[f] == AAT_FONT_FLAG || font_area[f] == OTGR_FONT_FLAG)
                scan_glyph_number(f);
            else
                scan_char_num();

            p = cur_val;
            scan_optional_equals();
            scan_int();
            switch (n) {
            case LP_CODE_BASE:
                set_cp_code(f, p, LEFT_SIDE, cur_val);
                break;
            case RP_CODE_BASE:
                set_cp_code(f, p, RIGHT_SIDE, cur_val);
                break;
            }
        }
        break;

    case DEF_FONT:
        new_font(a);
        break;

    case SET_INTERACTION:
        new_interaction();
        break;

    default:
        confusion("prefix");
        break;
    }

done: /*1304:*/
    if (after_token != 0) {
        cur_tok = after_token;
        back_input();
        after_token = 0;
    }
}
/*:1328*/


/*1337:*/
static void
store_fmt_file(void)
{
    int32_t j, k, l;
    int32_t p, q;
    int32_t x;
    rust_output_handle_t fmt_out;

    if (save_ptr != 0) {
        error_here_with_diagnostic("You can't dump inside a group");
        capture_to_diagnostic(NULL);
        help_ptr = 1;
        help_line[0] = "`{...\\dump}' is a no-no.";

        if (interaction == ERROR_STOP_MODE)
            interaction = SCROLL_MODE;
        if (log_opened)
            error();

        history = HISTORY_FATAL_ERROR;
        close_files_and_terminate();
        ttstub_output_flush (rust_stdout);
        _tt_abort("\\dump inside a group");
    }

    selector = SELECTOR_NEW_STRING;
    print_cstr(" (preloaded format=");
    print(job_name);
    print_char(' ');
    print_int(INTPAR(year));
    print_char('.');
    print_int(INTPAR(month));
    print_char('.');
    print_int(INTPAR(day));
    print_char(')');

    if (interaction == BATCH_MODE)
        selector = SELECTOR_LOG_ONLY;
    else
        selector = SELECTOR_TERM_AND_LOG;

    if (pool_ptr + 1 > pool_size)
        overflow("pool size", pool_size - init_pool_ptr);

    format_ident = make_string();
    pack_job_name(".fmt");

    fmt_out = ttstub_output_open (name_of_file, 0);
    if (fmt_out == NULL)
        _tt_abort ("cannot open format output file \"%s\"", name_of_file);

    print_nl_cstr("Beginning to dump on file ");
    print(make_name_string());

    str_ptr--;
    pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];

    print_nl_cstr("");
    print(format_ident);

    /* Header */

    dump_int(FORMAT_HEADER_MAGIC);
    dump_int(FORMAT_SERIAL);
    dump_int(hash_high);

    while (pseudo_files != TEX_NULL)
        pseudo_close(); /* TODO: can we move this farther up in this function? */

    dump_int(MEM_TOP);
    dump_int(EQTB_SIZE);
    dump_int(HASH_PRIME);
    dump_int(HYPH_PRIME);

    /* string pool */

    dump_int(pool_ptr);
    dump_int(str_ptr);
    dump_things(str_start[0], str_ptr - TOO_BIG_CHAR + 1);
    dump_things(str_pool[0], pool_ptr);

    print_ln();
    print_int(str_ptr);
    print_cstr(" strings of total length ");
    print_int(pool_ptr);

    /* "memory locations" */

    sort_avail();
    var_used = 0;
    dump_int(lo_mem_max);
    dump_int(rover);

    for (k = INT_VAL; k <= INTER_CHAR_VAL; k++)
        dump_int(sa_root[k]);

    p = 0;
    q = rover;
    x = 0;
    do {
        dump_things(mem[p], q + 2 - p);
        x = x + q + 2 - p;
        var_used = var_used + q - p;
        p = q + mem[q].b32.s0;
        q = mem[q + 1].b32.s1;
    } while (q != rover);

    var_used = var_used + lo_mem_max - p;
    dyn_used = mem_end + 1 - hi_mem_min;
    dump_things(mem[p], lo_mem_max + 1 - p);

    x = x + lo_mem_max + 1 - p;
    dump_int(hi_mem_min);
    dump_int(avail);
    dump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);

    x = x + mem_end + 1 - hi_mem_min;
    p = avail;
    while (p != TEX_NULL) {
        dyn_used--;
        p = LLIST_link(p);
    }

    dump_int(var_used);
    dump_int(dyn_used);

    print_ln();
    print_int(x);
    print_cstr(" memory locations dumped; current usage is ");
    print_int(var_used);
    print_char('&');
    print_int(dyn_used);

    /* equivalents table / primitive */

    k = ACTIVE_BASE;

    do {
        j = k;

        while (j < INT_BASE - 1) {
            if (eqtb[j].b32.s1 == eqtb[j + 1].b32.s1 &&
                eqtb[j].b16.s1 == eqtb[j + 1].b16.s1 &&
                eqtb[j].b16.s0 == eqtb[j + 1].b16.s0)
                goto found1;
            j++;
        }

        l = INT_BASE;
        goto done1;

    found1:
        j++;
        l = j;

        while (j < INT_BASE - 1) {
            if (eqtb[j].b32.s1 != eqtb[j + 1].b32.s1 ||
                eqtb[j].b16.s1 != eqtb[j + 1].b16.s1 ||
                eqtb[j].b16.s0 != eqtb[j + 1].b16.s0)
                goto done1;
            j++;
        }
    done1:

        dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (k != INT_BASE); /*:1350*/

    do {
        j = k;

        while (j < EQTB_SIZE) {
            if (eqtb[j].b32.s1 == eqtb[j + 1].b32.s1)
                goto found2;
            j++;
        }

        l = EQTB_SIZE + 1;
        goto done2;

    found2:
        j++;
        l = j;

        while (j < EQTB_SIZE) {
            if (eqtb[j].b32.s1 != eqtb[j + 1].b32.s1)
                goto done2;
            j++;
        }

    done2:
        dump_int(l - k);
        dump_things(eqtb[k], l - k);
        k = j + 1;
        dump_int(k - l);
    } while (k <= EQTB_SIZE);

    if (hash_high > 0)
        dump_things(eqtb[EQTB_SIZE + 1], hash_high);

    dump_int(par_loc);
    dump_int(write_loc);

    for (p = 0; p <= PRIM_SIZE; p++)
        dump_b32(prim[p]);

    /* control sequences */

    dump_int(hash_used);
    cs_count = (FROZEN_CONTROL_SEQUENCE - 1) - hash_used + hash_high;

    for (p = HASH_BASE; p <= hash_used; p++) {
        if (hash[p].s1 != 0) {
            dump_int(p);
            dump_b32(hash[p]);
            cs_count++;
        }
    }

    dump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);
    if (hash_high > 0)
        dump_things(hash[EQTB_SIZE + 1], hash_high);

    dump_int(cs_count);

    print_ln();
    print_int(cs_count);
    print_cstr(" multiletter control sequences");

    /* fonts */

    dump_int(fmem_ptr);
    dump_things(font_info[0], fmem_ptr);
    dump_int(font_ptr);
    dump_things(font_check[FONT_BASE], font_ptr + 1);
    dump_things(font_size[FONT_BASE], font_ptr + 1);
    dump_things(font_dsize[FONT_BASE], font_ptr + 1);
    dump_things(font_params[FONT_BASE], font_ptr + 1);
    dump_things(hyphen_char[FONT_BASE], font_ptr + 1);
    dump_things(skew_char[FONT_BASE], font_ptr + 1);
    dump_things(font_name[FONT_BASE], font_ptr + 1);
    dump_things(font_area[FONT_BASE], font_ptr + 1);
    dump_things(font_bc[FONT_BASE], font_ptr + 1);
    dump_things(font_ec[FONT_BASE], font_ptr + 1);
    dump_things(char_base[FONT_BASE], font_ptr + 1);
    dump_things(width_base[FONT_BASE], font_ptr + 1);
    dump_things(height_base[FONT_BASE], font_ptr + 1);
    dump_things(depth_base[FONT_BASE], font_ptr + 1);
    dump_things(italic_base[FONT_BASE], font_ptr + 1);
    dump_things(lig_kern_base[FONT_BASE], font_ptr + 1);
    dump_things(kern_base[FONT_BASE], font_ptr + 1);
    dump_things(exten_base[FONT_BASE], font_ptr + 1);
    dump_things(param_base[FONT_BASE], font_ptr + 1);
    dump_things(font_glue[FONT_BASE], font_ptr + 1);
    dump_things(bchar_label[FONT_BASE], font_ptr + 1);
    dump_things(font_bchar[FONT_BASE], font_ptr + 1);
    dump_things(font_false_bchar[FONT_BASE], font_ptr + 1);

    for (k = FONT_BASE; k <= font_ptr; k++) {
        print_nl_cstr("\\font");
        print_esc(hash[FONT_ID_BASE + k].s1);
        print_char('=');

        if (font_area[k] == AAT_FONT_FLAG || font_area[k] == OTGR_FONT_FLAG || font_mapping[k] != NULL) {
            print_file_name(font_name[k], EMPTY_STRING, EMPTY_STRING);

            error_here_with_diagnostic("Can't \\dump a format with native fonts or font-mappings");
            capture_to_diagnostic(NULL);

            help_ptr = 3;
            help_line[2] = "You really, really don't want to do this.";
            help_line[1] = "It won't work, and only confuses me.";
            help_line[0] = "(Load them at runtime, not as part of the format file.)";
            error();
        } else {
            print_file_name(font_name[k], font_area[k], EMPTY_STRING);
        }

        if (font_size[k] != font_dsize[k]) {
            print_cstr(" at ");
            print_scaled(font_size[k]);
            print_cstr("pt");
        }
    }

    print_ln();
    print_int(fmem_ptr - 7);
    print_cstr(" words of font info for ");
    print_int(font_ptr - 0);
    if (font_ptr != FONT_BASE + 1)
        print_cstr(" preloaded fonts");
    else
        print_cstr(" preloaded font");

    /* hyphenation info */

    dump_int(hyph_count);
    if (hyph_next <= HYPH_PRIME)
        hyph_next = hyph_size;
    dump_int(hyph_next);

    for (k = 0; k <= hyph_size; k++) {
        if (hyph_word[k] != 0) {
            dump_int(k + 65536L * hyph_link[k]);
            dump_int(hyph_word[k]);
            dump_int(hyph_list[k]);
        }
    }

    print_ln();
    print_int(hyph_count);
    if (hyph_count != 1)
        print_cstr(" hyphenation exceptions");
    else
        print_cstr(" hyphenation exception");

    if (trie_not_ready)
        init_trie();

    dump_int(trie_max);
    dump_int(hyph_start);
    dump_things(trie_trl[0], trie_max + 1);
    dump_things(trie_tro[0], trie_max + 1);
    dump_things(trie_trc[0], trie_max + 1);
    dump_int(max_hyph_char);
    dump_int(trie_op_ptr);
    dump_things(hyf_distance[1], trie_op_ptr);
    dump_things(hyf_num[1], trie_op_ptr);
    dump_things(hyf_next[1], trie_op_ptr);

    print_nl_cstr("Hyphenation trie of length ");
    print_int(trie_max);
    print_cstr(" has ");
    print_int(trie_op_ptr);
    if (trie_op_ptr != 1)
        print_cstr(" ops");
    else
        print_cstr(" op");
    print_cstr(" out of ");
    print_int(TRIE_OP_SIZE);

    for (k = BIGGEST_LANG; k >= 0; k--) {
        if (trie_used[k] > 0) {
            print_nl_cstr("  ");
            print_int(trie_used[k]);
            print_cstr(" for language ");
            print_int(k);
            dump_int(k);
            dump_int(trie_used[k]);
        }
    }

    /* footer */

    dump_int(FORMAT_FOOTER_MAGIC);

    INTPAR(tracing_stats) = 0; /*:1361*/
    ttstub_output_close(fmt_out);
}


static void
pack_buffered_name(small_number n, int32_t a, int32_t b)
{
    free(name_of_file);
    name_of_file = xmalloc_array(UTF8_code, format_default_length + 1);

    strcpy(name_of_file, TEX_format_default);
    name_length = strlen(name_of_file);
}


static bool
load_fmt_file(void)
{
    int32_t j, k;
    int32_t p, q;
    int32_t x;
    rust_input_handle_t fmt_in;

    j = cur_input.loc;

    /* This is where a first line starting with "&" used to
     * trigger code that would change the format file. */

    pack_buffered_name(format_default_length - 4, 1, 0);

    fmt_in = ttstub_input_open(name_of_file, TTBC_FILE_FORMAT_FORMAT, 0);
    if (fmt_in == NULL)
        _tt_abort("cannot open the format file \"%s\"", name_of_file);

    cur_input.loc = j;

    if (in_initex_mode) {
        free(font_info);
        free(str_pool);
        free(str_start);
        free(yhash);
        free(eqtb);
        free(mem);
        mem = NULL;
    }

    /* start reading the header */

    undump_int(x);
    if (x != FORMAT_HEADER_MAGIC)
        goto bad_fmt;

    undump_int(x);
    if (x != FORMAT_SERIAL)
        _tt_abort("format file \"%s\" is of the wrong version: expected %d, found %d",
                  name_of_file, FORMAT_SERIAL, x);

    /* hash table parameters */

    undump_int(hash_high);
    if (hash_high < 0 || hash_high > sup_hash_extra)
        goto bad_fmt;
    if (hash_extra < hash_high)
        hash_extra = hash_high;

    eqtb_top = EQTB_SIZE + hash_extra;
    if (hash_extra == 0)
        hash_top = UNDEFINED_CONTROL_SEQUENCE;
    else
        hash_top = eqtb_top;

    yhash = xmalloc_array(b32x2, 1 + hash_top - hash_offset);
    hash = yhash - hash_offset;
    hash[HASH_BASE].s0 = 0;
    hash[HASH_BASE].s1 = 0;

    for (x = HASH_BASE + 1; x <= hash_top; x++)
        hash[x] = hash[HASH_BASE];

    eqtb = xmalloc_array(memory_word, eqtb_top + 1);
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b16.s1 = UNDEFINED_CS;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b32.s1 = TEX_NULL;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b16.s0 = LEVEL_ZERO;

    for (x = EQTB_SIZE + 1; x <= eqtb_top; x++)
        eqtb[x] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    max_reg_num = 32767;
    max_reg_help_line = "A register number must be between 0 and 32767.";

    /* "memory locations" */

    undump_int(x);
    if (x != MEM_TOP)
        goto bad_fmt;

    cur_list.head = CONTRIB_HEAD;
    cur_list.tail = CONTRIB_HEAD;
    page_tail = PAGE_HEAD;
    mem = xmalloc_array(memory_word, MEM_TOP + 1);

    undump_int(x);
    if (x != EQTB_SIZE)
        goto bad_fmt;

    undump_int(x);
    if (x != HASH_PRIME)
        goto bad_fmt;

    undump_int(x);
    if (x != HYPH_PRIME)
        goto bad_fmt;

    /* string pool */

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > sup_pool_size - pool_free)
        _tt_abort ("must increase string_pool_size");
    pool_ptr = x;

    if (pool_size < pool_ptr + pool_free)
        pool_size = pool_ptr + pool_free;

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > sup_max_strings - strings_free)
        _tt_abort ("must increase sup_strings");
    str_ptr = x;

    if (max_strings < str_ptr + strings_free)
        max_strings = str_ptr + strings_free;

    str_start = xmalloc_array(pool_pointer, max_strings);
    undump_checked_things(0, pool_ptr, str_start[0], str_ptr - TOO_BIG_CHAR + 1);
    str_pool = xmalloc_array(packed_UTF16_code, pool_size);

    undump_things(str_pool[0], pool_ptr);

    init_str_ptr = str_ptr;
    init_pool_ptr = pool_ptr; /*:1345 */

    /* "By sorting the list of available spaces in the variable-size portion
     * of |mem|, we are usually able to get by without having to dump very
     * much of the dynamic memory." */

    undump_int(x);
    if (x < 1019 || x > MEM_TOP - HI_MEM_STAT_USAGE)
        goto bad_fmt;
    else
        lo_mem_max = x;

    undump_int(x);
    if (x < 20 || x > lo_mem_max)
        goto bad_fmt;
    else
        rover = x;

    for (k = INT_VAL; k <= INTER_CHAR_VAL; k++) {
        undump_int(x);
        if (x < MIN_HALFWORD || x > lo_mem_max)
            goto bad_fmt;
        else
            sa_root[k] = x;
    }

    p = 0;
    q = rover;

    do {
        undump_things(mem[p], q + 2 - p);
        p = q + mem[q].b32.s0;
        if (p > lo_mem_max || (q >= mem[q + 1].b32.s1 && mem[q + 1].b32.s1 != rover))
            goto bad_fmt;
        q = mem[q + 1].b32.s1;
    } while (q != rover);

    undump_things(mem[p], lo_mem_max + 1 - p);

    undump_int(x);
    if (x < lo_mem_max + 1 || x > PRE_ADJUST_HEAD)
        goto bad_fmt;
    else
        hi_mem_min = x;

    undump_int(x);
    if (x < MIN_HALFWORD || x > MEM_TOP)
        goto bad_fmt;
    else
        avail = x;

    mem_end = MEM_TOP;

    undump_things(mem[hi_mem_min], mem_end + 1 - hi_mem_min);
    undump_int(var_used);
    undump_int(dyn_used);

    /* equivalents table / primitives
     *
     * "The table of equivalents usually contains repeated information, so we
     * dump it in compressed form: The sequence of $n + 2$ values
     * $(n, x_1, \ldots, x_n, m)$ in the format file represents $n + m$ consecutive
     * entries of |eqtb|, with |m| extra copies of $x_n$, namely
     * $(x_1, \ldots, x_n, x_n, \ldots, x_n)$"
     */

    k = ACTIVE_BASE;

    do {
        undump_int(x);
        if (x < 1 || k + x > EQTB_SIZE + 1)
            goto bad_fmt;

        undump_things(eqtb[k], x);
        k = k + x;

        undump_int(x);
        if (x < 0 || k + x > EQTB_SIZE + 1)
            goto bad_fmt;

        for (j = k; j <= k + x - 1; j++)
            eqtb[j] = eqtb[k - 1];

        k = k + x;
    } while (k <= EQTB_SIZE);

    if (hash_high > 0)
        undump_things(eqtb[EQTB_SIZE + 1], hash_high);

    undump_int(x);
    if (x < HASH_BASE || x > hash_top)
        goto bad_fmt;
    else
        par_loc = x;

    par_token = CS_TOKEN_FLAG + par_loc;

    undump_int(x);
    if (x < HASH_BASE || x > hash_top)
        goto bad_fmt;
    else
        write_loc = x;

    /* control sequence names
     *
     * "A different scheme is used to compress the hash table, since its lower
     * region is usually sparse. When |text(p) != 0| for |p <= hash_used|, we
     * output two words, |p| and |hash[p]|. The hash table is, of course,
     * densely packed for |p >= hash_used|, so the remaining entries are
     * output in a block."
     */

    for (p = 0; p <= PRIM_SIZE; p++)
        undump_b32(prim[p]);

    undump_int(x);
    if (x < HASH_BASE || x > FROZEN_CONTROL_SEQUENCE)
        goto bad_fmt;
    else
        hash_used = x;

    p = HASH_BASE - 1;

    do {
        undump_int(x);
        if (x < p + 1 || x > hash_used)
            goto bad_fmt;
        else
            p = x;
        undump_b32(hash[p]);
    } while (p != hash_used);

    undump_things(hash[hash_used + 1], (UNDEFINED_CONTROL_SEQUENCE - 1) - hash_used);

    if (hash_high > 0)
        undump_things(hash[EQTB_SIZE + 1], hash_high);

    undump_int(cs_count);

    /* font info */

    undump_int(x);
    if (x < 7)
        goto bad_fmt;
    if (x > sup_font_mem_size)
        _tt_abort ("must increase font_mem_size");

    fmem_ptr = x;
    if (fmem_ptr > font_mem_size)
        font_mem_size = fmem_ptr;

    font_info = xmalloc_array(memory_word, font_mem_size);
    undump_things(font_info[0], fmem_ptr);

    undump_int(x);
    if (x < FONT_BASE)
        goto bad_fmt;
    if (x > FONT_BASE + MAX_FONT_MAX)
        _tt_abort ("must increase font_max");

    font_ptr = x;

    font_mapping = xmalloc_array(void *, font_max);
    font_layout_engine = xcalloc_array(void *, font_max);
    font_flags = xmalloc_array(char, font_max);
    font_letter_space = xmalloc_array(scaled_t, font_max);
    font_check = xmalloc_array(b16x4, font_max);
    font_size = xmalloc_array(scaled_t, font_max);
    font_dsize = xmalloc_array(scaled_t, font_max);
    font_params = xmalloc_array(font_index, font_max);
    font_name = xmalloc_array(str_number, font_max);
    font_area = xmalloc_array(str_number, font_max);
    font_bc = xmalloc_array(UTF16_code, font_max);
    font_ec = xmalloc_array(UTF16_code, font_max);
    font_glue = xmalloc_array(int32_t, font_max);
    hyphen_char = xmalloc_array(int32_t, font_max);
    skew_char = xmalloc_array(int32_t, font_max);
    bchar_label = xmalloc_array(font_index, font_max);
    font_bchar = xmalloc_array(nine_bits, font_max);
    font_false_bchar = xmalloc_array(nine_bits, font_max);
    char_base = xmalloc_array(int32_t, font_max);
    width_base = xmalloc_array(int32_t, font_max);
    height_base = xmalloc_array(int32_t, font_max);
    depth_base = xmalloc_array(int32_t, font_max);
    italic_base = xmalloc_array(int32_t, font_max);
    lig_kern_base = xmalloc_array(int32_t, font_max);
    kern_base = xmalloc_array(int32_t, font_max);
    exten_base = xmalloc_array(int32_t, font_max);
    param_base = xmalloc_array(int32_t, font_max);

    for (k = FONT_BASE; k <= font_ptr; k++)
        font_mapping[k] = 0;

    undump_things(font_check[FONT_BASE], font_ptr + 1);
    undump_things(font_size[FONT_BASE], font_ptr + 1);
    undump_things(font_dsize[FONT_BASE], font_ptr + 1);
    undump_checked_things(MIN_HALFWORD, MAX_HALFWORD, font_params[FONT_BASE], font_ptr + 1);
    undump_things(hyphen_char[FONT_BASE], font_ptr + 1);
    undump_things(skew_char[FONT_BASE], font_ptr + 1);
    undump_upper_check_things(str_ptr, font_name[FONT_BASE], font_ptr + 1);
    undump_upper_check_things(str_ptr, font_area[FONT_BASE], font_ptr + 1);
    undump_things(font_bc[FONT_BASE], font_ptr + 1);
    undump_things(font_ec[FONT_BASE], font_ptr + 1);
    undump_things(char_base[FONT_BASE], font_ptr + 1);
    undump_things(width_base[FONT_BASE], font_ptr + 1);
    undump_things(height_base[FONT_BASE], font_ptr + 1);
    undump_things(depth_base[FONT_BASE], font_ptr + 1);
    undump_things(italic_base[FONT_BASE], font_ptr + 1);
    undump_things(lig_kern_base[FONT_BASE], font_ptr + 1);
    undump_things(kern_base[FONT_BASE], font_ptr + 1);
    undump_things(exten_base[FONT_BASE], font_ptr + 1);
    undump_things(param_base[FONT_BASE], font_ptr + 1);
    undump_checked_things(MIN_HALFWORD, lo_mem_max, font_glue[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, fmem_ptr - 1, bchar_label[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, TOO_BIG_CHAR, font_bchar[FONT_BASE], font_ptr + 1);
    undump_checked_things(0, TOO_BIG_CHAR, font_false_bchar[FONT_BASE], font_ptr + 1);

    /* hyphenations */

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > hyph_size)
        _tt_abort ("must increase hyph_size");
    hyph_count = x;

    undump_int(x);
    if (x < HYPH_PRIME)
        goto bad_fmt;
    if (x > hyph_size)
        _tt_abort ("must increase hyph_size");
    hyph_next = x;

    j = 0;

    for (k = 1; k <= hyph_count; k++) {
        undump_int(j);
        if (j < 0)
            goto bad_fmt;
        if (j > 65535L) {
            hyph_next = j / 65536L;
            j = j - hyph_next * 65536L;
        } else {
            hyph_next = 0;
        }

        if (j >= hyph_size || hyph_next > hyph_size)
            goto bad_fmt;

        hyph_link[j] = hyph_next;

        undump_int(x);
        if (x < 0 || x > str_ptr)
            goto bad_fmt;
        else
            hyph_word[j] = x;

        undump_int(x);
        if (x < MIN_HALFWORD || x > MAX_HALFWORD)
            goto bad_fmt;
        else
            hyph_list[j] = x;
    }

    j++;
    if (j < HYPH_PRIME)
        j = HYPH_PRIME;

    hyph_next = j;
    if (hyph_next >= hyph_size)
        hyph_next = HYPH_PRIME;
    else if (hyph_next >= HYPH_PRIME)
        hyph_next++;

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > trie_size)
        _tt_abort ("must increase trie_size");

    j = x;
    trie_max = j;

    undump_int(x);
    if (x < 0 || x > j)
        goto bad_fmt;
    else
        hyph_start = x;

    if (!trie_trl)
        trie_trl = xmalloc_array(trie_pointer, j + 1);
    undump_things(trie_trl[0], j + 1);

    if (!trie_tro)
        trie_tro = xmalloc_array(trie_pointer, j + 1);
    undump_things(trie_tro[0], j + 1);

    if (!trie_trc)
        trie_trc = xmalloc_array(uint16_t, j + 1);
    undump_things(trie_trc[0], j + 1);

    undump_int(max_hyph_char);

    undump_int(x);
    if (x < 0)
        goto bad_fmt;
    if (x > TRIE_OP_SIZE)
        _tt_abort ("must increase TRIE_OP_SIZE");

    j = x;
    trie_op_ptr = j;

    undump_things(hyf_distance[1], j);
    undump_things(hyf_num[1], j);
    undump_upper_check_things(MAX_TRIE_OP, hyf_next[1], j);

    for (k = 0; k <= BIGGEST_LANG; k++)
        trie_used[k] = 0;

    k = BIGGEST_LANG + 1;

    while (j > 0) {
        undump_int(x);
        if (x < 0 || x > k - 1)
            goto bad_fmt;
        else
            k = x;

        undump_int(x);
        if (x < 1 || x > j)
            goto bad_fmt;

        trie_used[k] = x;
        j = j - x;
        op_start[k] = j;
    }

    trie_not_ready = false;

    /* trailer */

    undump_int(x);
    if (x != FORMAT_FOOTER_MAGIC)
        goto bad_fmt;

    ttstub_input_close (fmt_in);
    return true;

bad_fmt:
    _tt_abort ("fatal format file error");
}


static void
final_cleanup(void)
{
    small_number c;

    c = cur_chr;
    if (job_name == 0)
        open_log_file();
    while (input_ptr > 0)
        if (cur_input.state == TOKEN_LIST)
            end_token_list();
        else
            end_file_reading();
    while (open_parens > 0) {

        print_cstr(" )");
        open_parens--;
    }
    if (cur_level > LEVEL_ONE) {
        print_nl('(' );
        print_esc_cstr("end occurred ");
        print_cstr("inside a group at level ");
        print_int(cur_level - 1);
        print_char(')');
        show_save_groups();
    }
    while (cond_ptr != TEX_NULL) {

        print_nl('(' );
        print_esc_cstr("end occurred ");
        print_cstr("when ");
        print_cmd_chr(IF_TEST, cur_if);
        if (if_line != 0) {
            print_cstr(" on line ");
            print_int(if_line);
        }
        print_cstr(" was incomplete)");
        if_line = mem[cond_ptr + 1].b32.s1;
        cur_if = mem[cond_ptr].b16.s0;
        temp_ptr = cond_ptr;
        cond_ptr = LLIST_link(cond_ptr);
        free_node(temp_ptr, IF_NODE_SIZE);
    }

    if (history != HISTORY_SPOTLESS) {
        if ((history == HISTORY_WARNING_ISSUED || (interaction < ERROR_STOP_MODE))) {

            if (selector == SELECTOR_TERM_AND_LOG) {
                selector = SELECTOR_TERM_ONLY;
                print_nl_cstr("(see the transcript file for additional information)");
                selector = SELECTOR_TERM_AND_LOG;
            }
        }
    }
    if (c == 1) {
        if (in_initex_mode) {
            {
                register int32_t for_end;
                c = TOP_MARK_CODE;
                for_end = SPLIT_BOT_MARK_CODE;
                if (c <= for_end)
                    do
                        if (cur_mark[c] != TEX_NULL)
                            delete_token_ref(cur_mark[c]);
                    while (c++ < for_end) ;
            }
            if (sa_root[MARK_VAL] != TEX_NULL) {

                if (do_marks(3, 0, sa_root[MARK_VAL]))
                    sa_root[MARK_VAL] = TEX_NULL;
            }
            {
                register int32_t for_end;
                c = LAST_BOX_CODE;
                for_end = VSPLIT_CODE;
                if (c <= for_end)
                    do
                        flush_node_list(disc_ptr[c]);
                    while (c++ < for_end);
            }
            if (last_glue != MAX_HALFWORD)
                delete_glue_ref(last_glue);
            store_fmt_file();
            return;
        }
        print_nl_cstr("(\\dump is performed only by INITEX)");
        return;
    }
}


/* Engine initialization */

static UFILE stdin_ufile;

static void
init_io(void)
{
    /* This is largely vestigial at this point */
    stdin_ufile.handle = NULL;
    stdin_ufile.savedChar = -1;
    stdin_ufile.skipNextLF = 0;
    stdin_ufile.encodingMode = UTF8;
    stdin_ufile.conversionData = 0;
    input_file[0] = &stdin_ufile;

    buffer[first] = 0;
    last = first;
    cur_input.loc = first;
    cur_input.limit = last;
    first = last + 1;
}


static void
initialize_more_variables(void)
{
    int32_t k;
    hyph_pointer z;

    doing_special = false;
    native_text_size = 128;
    native_text = xmalloc(native_text_size * sizeof(UTF16_code));

    interaction = ERROR_STOP_MODE;

    deletions_allowed = true;
    set_box_allowed = true;
    error_count = 0;
    help_ptr = 0;
    use_err_help = false;

    two_to_the[0] = 1;
    for (k = 1; k <= 30; k++)
        two_to_the[k] = 2 * two_to_the[k - 1];

    spec_log[1] = 93032640L;
    spec_log[2] = 38612034L;
    spec_log[3] = 17922280L;
    spec_log[4] = 8662214L;
    spec_log[5] = 4261238L;
    spec_log[6] = 2113709L;
    spec_log[7] = 1052693L;
    spec_log[8] = 525315L;
    spec_log[9] = 262400L;
    spec_log[10] = 131136L;
    spec_log[11] = 65552L;
    spec_log[12] = 32772L;
    spec_log[13] = 16385;
    for (k = 14; k <= 27; k++)
        spec_log[k] = two_to_the[27 - k];
    spec_log[28] = 1;

    nest_ptr = 0;
    max_nest_stack = 0;
    cur_list.mode = VMODE;
    cur_list.head = CONTRIB_HEAD;
    cur_list.tail = CONTRIB_HEAD;
    cur_list.eTeX_aux = TEX_NULL;
    cur_list.aux.b32.s1 = IGNORE_DEPTH;
    cur_list.mode_line = 0;
    cur_list.prev_graf = 0;
    shown_mode = 0;
    page_contents = EMPTY;
    page_tail = PAGE_HEAD;
    last_glue = MAX_HALFWORD;
    last_penalty = 0;
    last_kern = 0;
    last_node_type = -1;
    page_so_far[7] = 0;

    for (k = INT_BASE; k <= EQTB_SIZE; k++)
        XEQ_LEVEL(k) = LEVEL_ONE;

    no_new_control_sequence = true;
    prim[0].s0 = 0;
    prim[0].s1 = 0;

    for (k = 1; k <= PRIM_SIZE; k++)
        prim[k] = prim[0];

    save_ptr = 0;
    cur_level = LEVEL_ONE;
    cur_group = BOTTOM_LEVEL;
    cur_boundary = 0;
    max_save_stack = 0;
    mag_set = 0;
    expand_depth_count = 0;
    is_in_csname = false;
    cur_mark[TOP_MARK_CODE] = TEX_NULL;
    cur_mark[FIRST_MARK_CODE] = TEX_NULL;
    cur_mark[BOT_MARK_CODE] = TEX_NULL;
    cur_mark[SPLIT_FIRST_MARK_CODE] = TEX_NULL;
    cur_mark[SPLIT_BOT_MARK_CODE] = TEX_NULL;
    cur_val = 0;
    cur_val_level = INT_VAL;
    radix = 0;
    cur_order = NORMAL;

    for (k = 0; k <= 16; k++)
        read_open[k] = CLOSED;

    cond_ptr = TEX_NULL;
    if_limit = NORMAL;
    cur_if = 0;
    if_line = 0;
    null_character.s3 = 0;
    null_character.s2 = 0;
    null_character.s1 = 0;
    null_character.s0 = 0;
    total_pages = 0;
    max_v = 0;
    max_h = 0;
    max_push = 0;
    last_bop = -1;
    doing_leaders = false;
    dead_cycles = 0;
    adjust_tail = TEX_NULL;
    last_badness = 0;
    pre_adjust_tail = TEX_NULL;
    pack_begin_line = 0;
    empty.s1 = EMPTY;
    empty.s0 = TEX_NULL;
    align_ptr = TEX_NULL;
    cur_align = TEX_NULL;
    cur_span = TEX_NULL;
    cur_loop = TEX_NULL;
    cur_head = TEX_NULL;
    cur_tail = TEX_NULL;
    cur_pre_head = TEX_NULL;
    cur_pre_tail = TEX_NULL;
    cur_f = 0;
    max_hyph_char = TOO_BIG_LANG;

    for (z = 0; z <= hyph_size; z++) {
        hyph_word[z] = 0;
        hyph_list[z] = TEX_NULL;
        hyph_link[z] = 0;
    }

    hyph_count = 0;
    hyph_next = (HYPH_PRIME + 1);
    if (hyph_next > hyph_size)
        hyph_next = HYPH_PRIME;

    output_active = false;
    insert_penalties = 0;
    ligature_present = false;
    cancel_boundary = false;
    lft_hit = false;
    rt_hit = false;
    ins_disc = false;
    after_token = 0;
    long_help_seen = false;
    format_ident = 0;

    for (k = 0; k <= 17; k++)
        write_open[k] = false;

    LR_ptr = TEX_NULL;
    LR_problems = 0;
    cur_dir = LEFT_TO_RIGHT;
    pseudo_files = TEX_NULL;
    sa_root[MARK_VAL] = TEX_NULL;
    sa_null.b32.s0 = TEX_NULL;
    sa_null.b32.s1 = TEX_NULL;
    sa_chain = TEX_NULL;
    sa_level = LEVEL_ZERO;
    disc_ptr[LAST_BOX_CODE] = TEX_NULL;
    disc_ptr[VSPLIT_CODE] = TEX_NULL;
    edit_name_start = 0;
    stop_at_space = true;
}

static void
initialize_more_initex_variables(void)
{
    int32_t i, k;

    for (k = 1; k <= 19; k++)
        mem[k].b32.s1 = 0;

    for (k = 0; k <= 19; k += 4) {
        mem[k].b32.s1 = TEX_NULL + 1;
        mem[k].b16.s1 = NORMAL;
        mem[k].b16.s0 = NORMAL;
    }

    mem[6].b32.s1 = 65536L;
    mem[4].b16.s1 = FIL;
    mem[10].b32.s1 = 65536L;
    mem[8].b16.s1 = FILL;
    mem[14].b32.s1 = 65536L;
    mem[12].b16.s1 = FIL;
    mem[15].b32.s1 = 65536L;
    mem[12].b16.s0 = FIL;
    mem[18].b32.s1 = -65536L;
    mem[16].b16.s1 = FIL;
    rover = 20;
    mem[rover].b32.s1 = MAX_HALFWORD;
    mem[rover].b32.s0 = 1000;
    mem[rover + 1].b32.s0 = rover;
    mem[rover + 1].b32.s1 = rover;
    lo_mem_max = rover + 1000;
    mem[lo_mem_max].b32.s1 = TEX_NULL;
    mem[lo_mem_max].b32.s0 = TEX_NULL;

    for (k = PRE_ADJUST_HEAD; k <= MEM_TOP; k++)
        mem[k] = mem[lo_mem_max];

    mem[OMIT_TEMPLATE].b32.s0 = CS_TOKEN_FLAG + FROZEN_END_TEMPLATE;
    mem[END_SPAN].b32.s1 = UINT16_MAX + 1;
    mem[END_SPAN].b32.s0 = TEX_NULL;
    mem[ACTIVE_LIST].b16.s1 = HYPHENATED;
    mem[ACTIVE_LIST+1].b32.s0 = MAX_HALFWORD;
    mem[ACTIVE_LIST].b16.s0 = 0;
    mem[PAGE_INS_HEAD].b16.s0 = 255;
    mem[PAGE_INS_HEAD].b16.s1 = SPLIT_UP;
    mem[PAGE_INS_HEAD].b32.s1 = PAGE_INS_HEAD;
    NODE_type(PAGE_HEAD) = GLUE_NODE;
    mem[PAGE_HEAD].b16.s0 = NORMAL;
    avail = TEX_NULL;
    mem_end = MEM_TOP;
    hi_mem_min = PRE_ADJUST_HEAD;
    var_used = 20;
    dyn_used = HI_MEM_STAT_USAGE;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b16.s1 = UNDEFINED_CS;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b32.s1 = TEX_NULL;
    eqtb[UNDEFINED_CONTROL_SEQUENCE].b16.s0 = LEVEL_ZERO;

    for (k = ACTIVE_BASE; k <= eqtb_top; k++)
        eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    eqtb[GLUE_BASE].b32.s1 = 0;
    eqtb[GLUE_BASE].b16.s0 = LEVEL_ONE;
    eqtb[GLUE_BASE].b16.s1 = GLUE_REF;

    for (k = GLUE_BASE + 1; k <= LOCAL_BASE - 1; k++)
        eqtb[k] = eqtb[GLUE_BASE];

    mem[0].b32.s1 += 531;
    LOCAL(par_shape) = TEX_NULL;
    eqtb[LOCAL_BASE + LOCAL__par_shape].b16.s1 = SHAPE_REF;
    eqtb[LOCAL_BASE + LOCAL__par_shape].b16.s0 = LEVEL_ONE;

    for (k = ETEX_PEN_BASE; k <= ETEX_PENS - 1; k++)
        eqtb[k] = eqtb[LOCAL_BASE + LOCAL__par_shape];

    for (k = LOCAL_BASE + LOCAL__output_routine; k <= TOKS_BASE + NUMBER_REGS - 1; k++)
        eqtb[k] = eqtb[UNDEFINED_CONTROL_SEQUENCE];

    eqtb[BOX_BASE].b32.s1 = TEX_NULL;
    eqtb[BOX_BASE].b16.s1 = BOX_REF;
    eqtb[BOX_BASE].b16.s0 = LEVEL_ONE;

    for (k = BOX_BASE + 1; k <= BOX_BASE + NUMBER_REGS - 1; k++)
        eqtb[k] = eqtb[BOX_BASE];

    eqtb[CUR_FONT_LOC].b32.s1 = FONT_BASE;
    eqtb[CUR_FONT_LOC].b16.s1 = DATA;
    eqtb[CUR_FONT_LOC].b16.s0 = LEVEL_ONE;

    for (k = MATH_FONT_BASE; k <= MATH_FONT_BASE + NUMBER_MATH_FONTS - 1; k++)
        eqtb[k] = eqtb[CUR_FONT_LOC];

    eqtb[CAT_CODE_BASE].b32.s1 = 0;
    eqtb[CAT_CODE_BASE].b16.s1 = DATA;
    eqtb[CAT_CODE_BASE].b16.s0 = LEVEL_ONE;

    for (k = CAT_CODE_BASE + 1; k <= INT_BASE - 1; k++)
        eqtb[k] = eqtb[CAT_CODE_BASE];

    for (k = 0; k <= NUMBER_USVS - 1; k++) {
        CAT_CODE(k) = OTHER_CHAR;
        MATH_CODE(k) = k;
        SF_CODE(k) = 1000;
    }

    CAT_CODE(13) = CAR_RET;
    CAT_CODE(32) = SPACER;
    CAT_CODE(92) = ESCAPE;
    CAT_CODE(37) = COMMENT;
    CAT_CODE(127) = INVALID_CHAR;
    eqtb[CAT_CODE_BASE].b32.s1 = IGNORE;

    for (k = '0'; k <= '9'; k++)
        MATH_CODE(k) = k + set_class(VAR_FAM_CLASS);

    for (k = 'A'; k <= 'Z'; k++) {
        CAT_CODE(k) = LETTER;
        CAT_CODE(k + 32) = LETTER;
        MATH_CODE(k) = k + set_family(1) + set_class(VAR_FAM_CLASS);
        MATH_CODE(k + 32) = k + 32 + set_family(1) + set_class(VAR_FAM_CLASS);
        LC_CODE(k) = k + 32;
        LC_CODE(k + 32) = k + 32;
        UC_CODE(k) = k;
        UC_CODE(k + 32) = k;
        SF_CODE(k) = 999;
    }

    for (k = INT_BASE; k <= DEL_CODE_BASE - 1; k++)
        eqtb[k].b32.s1 = 0;

    INTPAR(char_sub_def_min) = 256;
    INTPAR(char_sub_def_max) = -1;
    INTPAR(mag) = 1000;
    INTPAR(tolerance) = 10000;
    INTPAR(hang_after) = 1;
    INTPAR(max_dead_cycles) = 25;
    INTPAR(escape_char) = '\\' ;
    INTPAR(end_line_char) = CARRIAGE_RETURN;

    for (k = 0; k <= NUMBER_USVS - 1; k++)
        DEL_CODE(k) = -1;

    DEL_CODE(46) = 0;

    for (k = DIMEN_BASE; k <= EQTB_SIZE; k++)
        eqtb[k].b32.s1 = 0;

    prim_used = PRIM_SIZE;
    hash_used = FROZEN_CONTROL_SEQUENCE;
    hash_high = 0;
    cs_count = 0;
    eqtb[FROZEN_DONT_EXPAND].b16.s1 = DONT_EXPAND;
    hash[FROZEN_DONT_EXPAND].s1 = maketexstring("notexpanded:");
    eqtb[FROZEN_PRIMITIVE].b16.s1 = IGNORE_SPACES;
    eqtb[FROZEN_PRIMITIVE].b32.s1 = 1;
    eqtb[FROZEN_PRIMITIVE].b16.s0 = LEVEL_ONE;
    hash[FROZEN_PRIMITIVE].s1 = maketexstring("primitive");

    for (k = -(int32_t) TRIE_OP_SIZE; k <= TRIE_OP_SIZE; k++)
        TRIE_OP_HASH(k) = 0;

    for (k = 0; k <= BIGGEST_LANG; k++)
        trie_used[k] = MIN_TRIE_OP;

    max_op_used = MIN_TRIE_OP;
    trie_op_ptr = 0;
    trie_not_ready = true;
    hash[FROZEN_PROTECTION].s1 = maketexstring("inaccessible");

    format_ident = maketexstring(" (INITEX)");

    hash[END_WRITE].s1 = maketexstring("endwrite");
    eqtb[END_WRITE].b16.s0 = LEVEL_ONE;
    eqtb[END_WRITE].b16.s1 = OUTER_CALL;
    eqtb[END_WRITE].b32.s1 = TEX_NULL;

    max_reg_num = 32767;
    max_reg_help_line = "A register number must be between 0 and 32767.";

    for (i = INT_VAL; i <= INTER_CHAR_VAL; i++)
        sa_root[i] = TEX_NULL;

    INTPAR(xetex_hyphenatable_length) = 63;
}


/*:1370*//*1371: */
static void
initialize_primitives(void)
{

    no_new_control_sequence = false;
    first = 0;

    primitive("lineskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__line_skip);
    primitive("baselineskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__baseline_skip);
    primitive("parskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__par_skip);
    primitive("abovedisplayskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__above_display_skip);
    primitive("belowdisplayskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__below_display_skip);
    primitive("abovedisplayshortskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__above_display_short_skip);
    primitive("belowdisplayshortskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__below_display_short_skip);
    primitive("leftskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__left_skip);
    primitive("rightskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__right_skip);
    primitive("topskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__top_skip);
    primitive("splittopskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__split_top_skip);
    primitive("tabskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__tab_skip);
    primitive("spaceskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__space_skip);
    primitive("xspaceskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__xspace_skip);
    primitive("parfillskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__par_fill_skip);
    primitive("XeTeXlinebreakskip", ASSIGN_GLUE, GLUE_BASE + GLUE_PAR__xetex_linebreak_skip);

    primitive("thinmuskip", ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__thin_mu_skip);
    primitive("medmuskip", ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__med_mu_skip);
    primitive("thickmuskip", ASSIGN_MU_GLUE, GLUE_BASE + GLUE_PAR__thick_mu_skip);

    primitive("output", ASSIGN_TOKS, LOCAL_BASE + LOCAL__output_routine);
    primitive("everypar", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_par);
    primitive("everymath", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_math);
    primitive("everydisplay", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_display);
    primitive("everyhbox", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_hbox);
    primitive("everyvbox", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_vbox);
    primitive("everyjob", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_job);
    primitive("everycr", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_cr);
    primitive("errhelp", ASSIGN_TOKS, LOCAL_BASE + LOCAL__err_help);
    primitive("everyeof", ASSIGN_TOKS, LOCAL_BASE + LOCAL__every_eof);
    primitive("XeTeXinterchartoks", ASSIGN_TOKS, LOCAL_BASE + LOCAL__xetex_inter_char);
    primitive("TectonicCodaTokens", ASSIGN_TOKS, LOCAL_BASE + LOCAL__TectonicCodaTokens);

    primitive("pretolerance", ASSIGN_INT, INT_BASE + INT_PAR__pretolerance);
    primitive("tolerance", ASSIGN_INT, INT_BASE + INT_PAR__tolerance);
    primitive("linepenalty", ASSIGN_INT, INT_BASE + INT_PAR__line_penalty);
    primitive("hyphenpenalty", ASSIGN_INT, INT_BASE + INT_PAR__hyphen_penalty);
    primitive("exhyphenpenalty", ASSIGN_INT, INT_BASE + INT_PAR__ex_hyphen_penalty);
    primitive("clubpenalty", ASSIGN_INT, INT_BASE + INT_PAR__club_penalty);
    primitive("widowpenalty", ASSIGN_INT, INT_BASE + INT_PAR__widow_penalty);
    primitive("displaywidowpenalty", ASSIGN_INT, INT_BASE + INT_PAR__display_widow_penalty);
    primitive("brokenpenalty", ASSIGN_INT, INT_BASE + INT_PAR__broken_penalty);
    primitive("binoppenalty", ASSIGN_INT, INT_BASE + INT_PAR__bin_op_penalty);
    primitive("relpenalty", ASSIGN_INT, INT_BASE + INT_PAR__rel_penalty);
    primitive("predisplaypenalty", ASSIGN_INT, INT_BASE + INT_PAR__pre_display_penalty);
    primitive("postdisplaypenalty", ASSIGN_INT, INT_BASE + INT_PAR__post_display_penalty);
    primitive("interlinepenalty", ASSIGN_INT, INT_BASE + INT_PAR__inter_line_penalty);
    primitive("doublehyphendemerits", ASSIGN_INT, INT_BASE + INT_PAR__double_hyphen_demerits);
    primitive("finalhyphendemerits", ASSIGN_INT, INT_BASE + INT_PAR__final_hyphen_demerits);
    primitive("adjdemerits", ASSIGN_INT, INT_BASE + INT_PAR__adj_demerits);
    primitive("mag", ASSIGN_INT, INT_BASE + INT_PAR__mag);
    primitive("delimiterfactor", ASSIGN_INT, INT_BASE + INT_PAR__delimiter_factor);
    primitive("looseness", ASSIGN_INT, INT_BASE + INT_PAR__looseness);
    primitive("time", ASSIGN_INT, INT_BASE + INT_PAR__time);
    primitive("day", ASSIGN_INT, INT_BASE + INT_PAR__day);
    primitive("month", ASSIGN_INT, INT_BASE + INT_PAR__month);
    primitive("year", ASSIGN_INT, INT_BASE + INT_PAR__year);
    primitive("showboxbreadth", ASSIGN_INT, INT_BASE + INT_PAR__show_box_breadth);
    primitive("showboxdepth", ASSIGN_INT, INT_BASE + INT_PAR__show_box_depth);
    primitive("hbadness", ASSIGN_INT, INT_BASE + INT_PAR__hbadness);
    primitive("vbadness", ASSIGN_INT, INT_BASE + INT_PAR__vbadness);
    primitive("pausing", ASSIGN_INT, INT_BASE + INT_PAR__pausing);
    primitive("tracingonline", ASSIGN_INT, INT_BASE + INT_PAR__tracing_online);
    primitive("tracingmacros", ASSIGN_INT, INT_BASE + INT_PAR__tracing_macros);
    primitive("tracingstats", ASSIGN_INT, INT_BASE + INT_PAR__tracing_stats);
    primitive("tracingparagraphs", ASSIGN_INT, INT_BASE + INT_PAR__tracing_paragraphs);
    primitive("tracingpages", ASSIGN_INT, INT_BASE + INT_PAR__tracing_pages);
    primitive("tracingoutput", ASSIGN_INT, INT_BASE + INT_PAR__tracing_output);
    primitive("tracinglostchars", ASSIGN_INT, INT_BASE + INT_PAR__tracing_lost_chars);
    primitive("tracingcommands", ASSIGN_INT, INT_BASE + INT_PAR__tracing_commands);
    primitive("tracingrestores", ASSIGN_INT, INT_BASE + INT_PAR__tracing_restores);
    primitive("uchyph", ASSIGN_INT, INT_BASE + INT_PAR__uc_hyph);
    primitive("outputpenalty", ASSIGN_INT, INT_BASE + INT_PAR__output_penalty);
    primitive("maxdeadcycles", ASSIGN_INT, INT_BASE + INT_PAR__max_dead_cycles);
    primitive("hangafter", ASSIGN_INT, INT_BASE + INT_PAR__hang_after);
    primitive("floatingpenalty", ASSIGN_INT, INT_BASE + INT_PAR__floating_penalty);
    primitive("globaldefs", ASSIGN_INT, INT_BASE + INT_PAR__global_defs);
    primitive("fam", ASSIGN_INT, INT_BASE + INT_PAR__cur_fam);
    primitive("escapechar", ASSIGN_INT, INT_BASE + INT_PAR__escape_char);
    primitive("defaulthyphenchar", ASSIGN_INT, INT_BASE + INT_PAR__default_hyphen_char);
    primitive("defaultskewchar", ASSIGN_INT, INT_BASE + INT_PAR__default_skew_char);
    primitive("endlinechar", ASSIGN_INT, INT_BASE + INT_PAR__end_line_char);
    primitive("newlinechar", ASSIGN_INT, INT_BASE + INT_PAR__new_line_char);
    primitive("language", ASSIGN_INT, INT_BASE + INT_PAR__language);
    primitive("lefthyphenmin", ASSIGN_INT, INT_BASE + INT_PAR__left_hyphen_min);
    primitive("righthyphenmin", ASSIGN_INT, INT_BASE + INT_PAR__right_hyphen_min);
    primitive("holdinginserts", ASSIGN_INT, INT_BASE + INT_PAR__holding_inserts);
    primitive("errorcontextlines", ASSIGN_INT, INT_BASE + INT_PAR__error_context_lines);

    primitive("XeTeXlinebreakpenalty", ASSIGN_INT, INT_BASE + 69);
    primitive("XeTeXprotrudechars", ASSIGN_INT, INT_BASE + 70);

    primitive("parindent", ASSIGN_DIMEN, DIMEN_BASE + 0);
    primitive("mathsurround", ASSIGN_DIMEN, DIMEN_BASE + 1);
    primitive("lineskiplimit", ASSIGN_DIMEN, DIMEN_BASE + 2);
    primitive("hsize", ASSIGN_DIMEN, DIMEN_BASE + 3);
    primitive("vsize", ASSIGN_DIMEN, DIMEN_BASE + 4);
    primitive("maxdepth", ASSIGN_DIMEN, DIMEN_BASE + 5);
    primitive("splitmaxdepth", ASSIGN_DIMEN, DIMEN_BASE + 6);
    primitive("boxmaxdepth", ASSIGN_DIMEN, DIMEN_BASE + 7);
    primitive("hfuzz", ASSIGN_DIMEN, DIMEN_BASE + 8);
    primitive("vfuzz", ASSIGN_DIMEN, DIMEN_BASE + 9);
    primitive("delimitershortfall", ASSIGN_DIMEN, DIMEN_BASE + 10);
    primitive("nulldelimiterspace", ASSIGN_DIMEN, DIMEN_BASE + 11);
    primitive("scriptspace", ASSIGN_DIMEN, DIMEN_BASE + 12);
    primitive("predisplaysize", ASSIGN_DIMEN, DIMEN_BASE + 13);
    primitive("displaywidth", ASSIGN_DIMEN, DIMEN_BASE + 14);
    primitive("displayindent", ASSIGN_DIMEN, DIMEN_BASE + 15);
    primitive("overfullrule", ASSIGN_DIMEN, DIMEN_BASE + 16);
    primitive("hangindent", ASSIGN_DIMEN, DIMEN_BASE + 17);
    primitive("hoffset", ASSIGN_DIMEN, DIMEN_BASE + 18);
    primitive("voffset", ASSIGN_DIMEN, DIMEN_BASE + 19);
    primitive("emergencystretch", ASSIGN_DIMEN, DIMEN_BASE + 20);
    primitive("pdfpagewidth", ASSIGN_DIMEN, DIMEN_BASE + 21);
    primitive("pdfpageheight", ASSIGN_DIMEN, DIMEN_BASE + 22);

    primitive(" ", EX_SPACE, 0);
    primitive("/", ITAL_CORR, 0);
    primitive("accent", ACCENT, 0);
    primitive("advance", ADVANCE, 0);
    primitive("afterassignment", AFTER_ASSIGNMENT, 0);
    primitive("aftergroup", AFTER_GROUP, 0);
    primitive("begingroup", BEGIN_GROUP, 0);
    primitive("char", CHAR_NUM, 0);
    primitive("csname", CS_NAME, 0);
    primitive("delimiter", DELIM_NUM, 0);
    primitive("XeTeXdelimiter", DELIM_NUM, 1);
    primitive("Udelimiter", DELIM_NUM, 1);
    primitive("divide", DIVIDE, 0);
    primitive("endcsname", END_CS_NAME, 0);
    primitive("endgroup", END_GROUP, 0);
    hash[FROZEN_END_GROUP].s1 = maketexstring("endgroup");
    eqtb[FROZEN_END_GROUP] = eqtb[cur_val];
    primitive("expandafter", EXPAND_AFTER, 0);
    primitive("font", DEF_FONT, 0);
    primitive("fontdimen", ASSIGN_FONT_DIMEN, 0);
    primitive("halign", HALIGN, 0);
    primitive("hrule", HRULE, 0);
    primitive("ignorespaces", IGNORE_SPACES, 0);
    primitive("insert", INSERT, 0);
    primitive("mark", MARK, 0);
    primitive("mathaccent", MATH_ACCENT, 0);
    primitive("XeTeXmathaccent", MATH_ACCENT, 1);
    primitive("Umathaccent", MATH_ACCENT, 1);
    primitive("mathchar", MATH_CHAR_NUM, 0);
    primitive("XeTeXmathcharnum", MATH_CHAR_NUM, 1);
    primitive("Umathcharnum", MATH_CHAR_NUM, 1);
    primitive("XeTeXmathchar", MATH_CHAR_NUM, 2);
    primitive("Umathchar", MATH_CHAR_NUM, 2);
    primitive("mathchoice", MATH_CHOICE, 0);
    primitive("multiply", MULTIPLY, 0);
    primitive("noalign", NO_ALIGN, 0);
    primitive("noboundary", NO_BOUNDARY, 0);
    primitive("noexpand", NO_EXPAND, 0);
    primitive("primitive", NO_EXPAND, 1);
    primitive("nonscript", NON_SCRIPT, 0);
    primitive("omit", OMIT, 0);
    primitive("parshape", SET_SHAPE, LOCAL_BASE + LOCAL__par_shape);
    primitive("penalty", BREAK_PENALTY, 0);
    primitive("prevgraf", SET_PREV_GRAF, 0);
    primitive("radical", RADICAL, 0);
    primitive("XeTeXradical", RADICAL, 1);
    primitive("Uradical", RADICAL, 1);
    primitive("read", READ_TO_CS, 0);
    primitive("relax", RELAX, TOO_BIG_USV);
    hash[FROZEN_RELAX].s1 = maketexstring("relax");
    eqtb[FROZEN_RELAX] = eqtb[cur_val];
    primitive("setbox", SET_BOX, 0);
    primitive("the", THE, 0);
    primitive("toks", TOKS_REGISTER, 0);
    primitive("vadjust", VADJUST, 0);
    primitive("valign", VALIGN, 0);
    primitive("vcenter", VCENTER, 0);
    primitive("vrule", VRULE, 0);
    primitive("par", PAR_END, TOO_BIG_USV);
    par_loc = cur_val;
    par_token = CS_TOKEN_FLAG + par_loc;

    primitive("input", INPUT, 0);
    primitive("endinput", INPUT, 1);

    primitive("topmark", TOP_BOT_MARK, TOP_MARK_CODE);
    primitive("firstmark", TOP_BOT_MARK, FIRST_MARK_CODE);
    primitive("botmark", TOP_BOT_MARK, BOT_MARK_CODE);
    primitive("splitfirstmark", TOP_BOT_MARK, SPLIT_FIRST_MARK_CODE);
    primitive("splitbotmark", TOP_BOT_MARK, SPLIT_BOT_MARK_CODE);

    primitive("count", REGISTER, 0);
    primitive("dimen", REGISTER, 1);
    primitive("skip", REGISTER, 2);
    primitive("muskip", REGISTER, 3);

    primitive("spacefactor", SET_AUX, HMODE);
    primitive("prevdepth", SET_AUX, VMODE);

    primitive("deadcycles", SET_PAGE_INT, 0);
    primitive("insertpenalties", SET_PAGE_INT, 1);

    primitive("wd", SET_BOX_DIMEN, WIDTH_OFFSET);
    primitive("ht", SET_BOX_DIMEN, HEIGHT_OFFSET);
    primitive("dp", SET_BOX_DIMEN, DEPTH_OFFSET);

    primitive("lastpenalty", LAST_ITEM, INT_VAL);
    primitive("lastkern", LAST_ITEM, DIMEN_VAL);
    primitive("lastskip", LAST_ITEM, GLUE_VAL);
    primitive("inputlineno", LAST_ITEM, INPUT_LINE_NO_CODE);
    primitive("badness", LAST_ITEM, BADNESS_CODE);
    primitive("pdflastxpos", LAST_ITEM, PDF_LAST_X_POS_CODE);
    primitive("pdflastypos", LAST_ITEM, PDF_LAST_Y_POS_CODE);
    primitive("elapsedtime", LAST_ITEM, ELAPSED_TIME_CODE);
    primitive("shellescape", LAST_ITEM, PDF_SHELL_ESCAPE_CODE);
    primitive("randomseed", LAST_ITEM, RANDOM_SEED_CODE);

    primitive("number", CONVERT, NUMBER_CODE);
    primitive("romannumeral", CONVERT, ROMAN_NUMERAL_CODE);
    primitive("string", CONVERT, STRING_CODE);
    primitive("meaning", CONVERT, MEANING_CODE);
    primitive("fontname", CONVERT, FONT_NAME_CODE);
    primitive("expanded", CONVERT, EXPANDED_CODE);
    primitive("leftmarginkern", CONVERT, LEFT_MARGIN_KERN_CODE);
    primitive("rightmarginkern", CONVERT, RIGHT_MARGIN_KERN_CODE);
    primitive("creationdate", CONVERT, PDF_CREATION_DATE_CODE);
    primitive("filemoddate", CONVERT, PDF_FILE_MOD_DATE_CODE);
    primitive("filesize", CONVERT, PDF_FILE_SIZE_CODE);
    primitive("mdfivesum", CONVERT, PDF_MDFIVE_SUM_CODE);
    primitive("filedump", CONVERT, PDF_FILE_DUMP_CODE);
    primitive("strcmp", CONVERT, PDF_STRCMP_CODE);
    primitive("uniformdeviate", CONVERT, UNIFORM_DEVIATE_CODE);
    primitive("normaldeviate", CONVERT, NORMAL_DEVIATE_CODE);
    primitive("jobname", CONVERT, JOB_NAME_CODE);
    primitive("Uchar", CONVERT, XETEX_UCHAR_CODE);
    primitive("Ucharcat", CONVERT, XETEX_UCHARCAT_CODE);

    primitive("if", IF_TEST, IF_CHAR_CODE);
    primitive("ifcat", IF_TEST, IF_CAT_CODE);
    primitive("ifnum", IF_TEST, IF_INT_CODE);
    primitive("ifdim", IF_TEST, IF_DIM_CODE);
    primitive("ifodd", IF_TEST, IF_ODD_CODE);
    primitive("ifvmode", IF_TEST, IF_VMODE_CODE);
    primitive("ifhmode", IF_TEST, IF_HMODE_CODE);
    primitive("ifmmode", IF_TEST, IF_MMODE_CODE);
    primitive("ifinner", IF_TEST, IF_INNER_CODE);
    primitive("ifvoid", IF_TEST, IF_VOID_CODE);
    primitive("ifhbox", IF_TEST, IF_HBOX_CODE);
    primitive("ifvbox", IF_TEST, IF_VBOX_CODE);
    primitive("ifx", IF_TEST, IFX_CODE);
    primitive("ifeof", IF_TEST, IF_EOF_CODE);
    primitive("iftrue", IF_TEST, IF_TRUE_CODE);
    primitive("iffalse", IF_TEST, IF_FALSE_CODE);
    primitive("ifcase", IF_TEST, IF_CASE_CODE);
    primitive("ifprimitive", IF_TEST, IF_PRIMITIVE_CODE);

    primitive("fi", FI_OR_ELSE, FI_CODE);
    hash[FROZEN_FI].s1 = maketexstring("fi");
    eqtb[FROZEN_FI] = eqtb[cur_val];
    primitive("or", FI_OR_ELSE, OR_CODE);
    primitive("else", FI_OR_ELSE, ELSE_CODE);

    primitive("nullfont", SET_FONT, FONT_BASE);
    hash[FROZEN_NULL_FONT].s1 = maketexstring("nullfont");
    eqtb[FROZEN_NULL_FONT] = eqtb[cur_val];

    primitive("span", TAB_MARK, SPAN_CODE);
    primitive("cr", CAR_RET, CR_CODE);
    hash[FROZEN_CR].s1 = maketexstring("cr");
    eqtb[FROZEN_CR] = eqtb[cur_val];
    primitive("crcr", CAR_RET, CR_CR_CODE);

    hash[FROZEN_END_TEMPLATE].s1 = maketexstring("endtemplate");
    hash[FROZEN_ENDV].s1 = maketexstring("endtemplate");
    eqtb[FROZEN_ENDV].b16.s1 = ENDV;
    eqtb[FROZEN_ENDV].b32.s1 = NULL_LIST;
    eqtb[FROZEN_ENDV].b16.s0 = LEVEL_ONE;
    eqtb[FROZEN_END_TEMPLATE] = eqtb[FROZEN_ENDV];
    eqtb[FROZEN_END_TEMPLATE].b16.s1 = END_TEMPLATE;

    primitive("pagegoal", SET_PAGE_DIMEN, 0);
    primitive("pagetotal", SET_PAGE_DIMEN, 1);
    primitive("pagestretch", SET_PAGE_DIMEN, 2);
    primitive("pagefilstretch", SET_PAGE_DIMEN, 3);
    primitive("pagefillstretch", SET_PAGE_DIMEN, 4);
    primitive("pagefilllstretch", SET_PAGE_DIMEN, 5);
    primitive("pageshrink", SET_PAGE_DIMEN, 6);
    primitive("pagedepth", SET_PAGE_DIMEN, 7);

    primitive("end", STOP, 0);
    primitive("dump", STOP, 1);

    primitive("hskip", HSKIP, SKIP_CODE);
    primitive("hfil", HSKIP, FIL_CODE);
    primitive("hfill", HSKIP, FILL_CODE);
    primitive("hss", HSKIP, SS_CODE);
    primitive("hfilneg", HSKIP, FIL_NEG_CODE);
    primitive("vskip", VSKIP, SKIP_CODE);
    primitive("vfil", VSKIP, FIL_CODE);
    primitive("vfill", VSKIP, FILL_CODE);
    primitive("vss", VSKIP, SS_CODE);
    primitive("vfilneg", VSKIP, FIL_NEG_CODE);
    primitive("mskip", MSKIP, MSKIP_CODE);

    primitive("kern", KERN, EXPLICIT);
    primitive("mkern", MKERN, MU_GLUE);
    primitive("moveleft", HMOVE, 1);
    primitive("moveright", HMOVE, 0);
    primitive("raise", VMOVE, 1);
    primitive("lower", VMOVE, 0);

    primitive("box", MAKE_BOX, BOX_CODE);
    primitive("copy", MAKE_BOX, COPY_CODE);
    primitive("lastbox", MAKE_BOX, LAST_BOX_CODE);
    primitive("vsplit", MAKE_BOX, VSPLIT_CODE);
    primitive("vtop", MAKE_BOX, VTOP_CODE);
    primitive("vbox", MAKE_BOX, VTOP_CODE + 1);
    primitive("hbox", MAKE_BOX, VTOP_CODE + 104);

    primitive("shipout", LEADER_SHIP, A_LEADERS - 1);
    primitive("leaders", LEADER_SHIP, A_LEADERS);
    primitive("cleaders", LEADER_SHIP, C_LEADERS);
    primitive("xleaders", LEADER_SHIP, X_LEADERS);

    primitive("indent", START_PAR, 1);
    primitive("noindent", START_PAR, 0);
    primitive("unpenalty", REMOVE_ITEM, PENALTY_NODE);
    primitive("unkern", REMOVE_ITEM, KERN_NODE);
    primitive("unskip", REMOVE_ITEM, GLUE_NODE);
    primitive("unhbox", UN_HBOX, BOX_CODE);
    primitive("unhcopy", UN_HBOX, COPY_CODE);
    primitive("unvbox", UN_VBOX, BOX_CODE);
    primitive("unvcopy", UN_VBOX, COPY_CODE);

    primitive("-", DISCRETIONARY, 1);
    primitive("discretionary", DISCRETIONARY, 0);

    primitive("eqno", EQ_NO, 0);
    primitive("leqno", EQ_NO, 1);

    primitive("mathord", MATH_COMP, ORD_NOAD);
    primitive("mathop", MATH_COMP, OP_NOAD);
    primitive("mathbin", MATH_COMP, BIN_NOAD);
    primitive("mathrel", MATH_COMP, REL_NOAD);
    primitive("mathopen", MATH_COMP, OPEN_NOAD);
    primitive("mathclose", MATH_COMP, CLOSE_NOAD);
    primitive("mathpunct", MATH_COMP, PUNCT_NOAD);
    primitive("mathinner", MATH_COMP, INNER_NOAD);
    primitive("underline", MATH_COMP, UNDER_NOAD);
    primitive("overline", MATH_COMP, OVER_NOAD);

    primitive("displaylimits", LIMIT_SWITCH, NORMAL);
    primitive("limits", LIMIT_SWITCH, LIMITS);
    primitive("nolimits", LIMIT_SWITCH, NO_LIMITS);

    primitive("displaystyle", MATH_STYLE, DISPLAY_STYLE);
    primitive("textstyle", MATH_STYLE, TEXT_STYLE);
    primitive("scriptstyle", MATH_STYLE, SCRIPT_STYLE);
    primitive("scriptscriptstyle", MATH_STYLE, SCRIPT_SCRIPT_STYLE);

    primitive("above", ABOVE, ABOVE_CODE);
    primitive("over", ABOVE, OVER_CODE);
    primitive("atop", ABOVE, ATOP_CODE);
    primitive("abovewithdelims", ABOVE, DELIMITED_CODE + 0);
    primitive("overwithdelims", ABOVE, DELIMITED_CODE + 1);
    primitive("atopwithdelims", ABOVE, DELIMITED_CODE + 2);

    primitive("left", LEFT_RIGHT, LEFT_NOAD);
    primitive("right", LEFT_RIGHT, RIGHT_NOAD);
    hash[FROZEN_RIGHT].s1 = maketexstring("right");
    eqtb[FROZEN_RIGHT] = eqtb[cur_val];

    primitive("long", PREFIX, 1);
    primitive("outer", PREFIX, 2);
    primitive("global", PREFIX, 4);
    primitive("def", DEF, 0);
    primitive("gdef", DEF, 1);
    primitive("edef", DEF, 2);
    primitive("xdef", DEF, 3);
    primitive("let", LET, NORMAL);
    primitive("futurelet", LET, NORMAL + 1);

    primitive("chardef", SHORTHAND_DEF, CHAR_DEF_CODE);
    primitive("mathchardef", SHORTHAND_DEF, MATH_CHAR_DEF_CODE);
    primitive("XeTeXmathcharnumdef", SHORTHAND_DEF, XETEX_MATH_CHAR_NUM_DEF_CODE);
    primitive("Umathcharnumdef", SHORTHAND_DEF, XETEX_MATH_CHAR_NUM_DEF_CODE);
    primitive("XeTeXmathchardef", SHORTHAND_DEF, XETEX_MATH_CHAR_DEF_CODE);
    primitive("Umathchardef", SHORTHAND_DEF, XETEX_MATH_CHAR_DEF_CODE);
    primitive("countdef", SHORTHAND_DEF, COUNT_DEF_CODE);
    primitive("dimendef", SHORTHAND_DEF, DIMEN_DEF_CODE);
    primitive("skipdef", SHORTHAND_DEF, SKIP_DEF_CODE);
    primitive("muskipdef", SHORTHAND_DEF, MU_SKIP_DEF_CODE);
    primitive("toksdef", SHORTHAND_DEF, TOKS_DEF_CODE);

    primitive("catcode", DEF_CODE, CAT_CODE_BASE);
    primitive("mathcode", DEF_CODE, MATH_CODE_BASE);
    primitive("XeTeXmathcodenum", XETEX_DEF_CODE, MATH_CODE_BASE);
    primitive("Umathcodenum", XETEX_DEF_CODE, MATH_CODE_BASE);
    primitive("XeTeXmathcode", XETEX_DEF_CODE, MATH_CODE_BASE + 1);
    primitive("Umathcode", XETEX_DEF_CODE, MATH_CODE_BASE + 1);
    primitive("lccode", DEF_CODE, LC_CODE_BASE);
    primitive("uccode", DEF_CODE, UC_CODE_BASE);
    primitive("sfcode", DEF_CODE, SF_CODE_BASE);
    primitive("XeTeXcharclass", XETEX_DEF_CODE, SF_CODE_BASE);
    primitive("delcode", DEF_CODE, DEL_CODE_BASE);
    primitive("XeTeXdelcodenum", XETEX_DEF_CODE, DEL_CODE_BASE);
    primitive("Udelcodenum", XETEX_DEF_CODE, DEL_CODE_BASE);
    primitive("XeTeXdelcode", XETEX_DEF_CODE, DEL_CODE_BASE + 1);
    primitive("Udelcode", XETEX_DEF_CODE, DEL_CODE_BASE + 1);

    primitive("textfont", DEF_FAMILY, MATH_FONT_BASE + TEXT_SIZE);
    primitive("scriptfont", DEF_FAMILY, MATH_FONT_BASE + SCRIPT_SIZE);
    primitive("scriptscriptfont", DEF_FAMILY, MATH_FONT_BASE + SCRIPT_SCRIPT_SIZE);

    primitive("hyphenation", HYPH_DATA, 0);
    primitive("patterns", HYPH_DATA, 1);

    primitive("hyphenchar", ASSIGN_FONT_INT, 0);
    primitive("skewchar", ASSIGN_FONT_INT, 1);
    primitive("lpcode", ASSIGN_FONT_INT, 2);
    primitive("rpcode", ASSIGN_FONT_INT, 3);

    primitive("batchmode", SET_INTERACTION, BATCH_MODE);
    primitive("nonstopmode", SET_INTERACTION, NONSTOP_MODE);
    primitive("scrollmode", SET_INTERACTION, SCROLL_MODE);
    primitive("errorstopmode", SET_INTERACTION, ERROR_STOP_MODE);

    primitive("openin", IN_STREAM, 1);
    primitive("closein", IN_STREAM, 0);
    primitive("message", MESSAGE, 0);
    primitive("errmessage", MESSAGE, 1);
    primitive("lowercase", CASE_SHIFT, LC_CODE_BASE);
    primitive("uppercase", CASE_SHIFT, UC_CODE_BASE);

    primitive("show", XRAY, SHOW_CODE);
    primitive("showbox", XRAY, SHOW_BOX_CODE);
    primitive("showthe", XRAY, SHOW_THE_CODE);
    primitive("showlists", XRAY, SHOW_LISTS);

    primitive("openout", EXTENSION, OPEN_NODE);
    primitive("write", EXTENSION, WRITE_NODE);
    write_loc = cur_val;
    primitive("closeout", EXTENSION, CLOSE_NODE);
    primitive("special", EXTENSION, SPECIAL_NODE);
    hash[FROZEN_SPECIAL].s1 = maketexstring("special");
    eqtb[FROZEN_SPECIAL] = eqtb[cur_val];
    primitive("immediate", EXTENSION, IMMEDIATE_CODE);
    primitive("setlanguage", EXTENSION, SET_LANGUAGE_CODE);
    primitive("resettimer", EXTENSION, RESET_TIMER_CODE);
    primitive("setrandomseed", EXTENSION, SET_RANDOM_SEED_CODE);

    primitive("synctex", ASSIGN_INT, INT_BASE + INT_PAR__synctex);

    no_new_control_sequence = true;
}


static void
get_strings_started(void)
{
    pool_ptr = 0;
    str_ptr = 0;
    str_start[0] = 0;
    str_ptr = TOO_BIG_CHAR;

    if (load_pool_strings(pool_size - string_vacancies) == 0)
        _tt_abort ("must increase pool_size");
}
/*:1001*/

void
tt_cleanup(void) {
    /*
        Cleanup of all intermediate buffers.
        Conceptually, final_cleanup() and close_files_and_terminate() also
        belong here, but that requires a more thorough refactor as presently
        it would result in a segfault.
    */

    pdf_files_close();
    free(TEX_format_default);
    free(font_used);
    deinitialize_shipout_variables();

    destroy_font_manager();

    for (int font_k = 0; font_k < font_max; font_k++) {
        if (font_layout_engine[font_k] != NULL) {
            release_font_engine(font_layout_engine[font_k], font_area[font_k]);
            font_layout_engine[font_k] = NULL;
        }
    }

    // Free the big allocated arrays
    free(buffer);
    free(nest);
    free(save_stack);
    free(input_stack);
    free(input_file);
    free(line_stack);
    free(eof_seen);
    free(grp_stack);
    free(if_stack);
    free(source_filename_stack);
    free(full_source_filename_stack);
    free(param_stack);
    free(hyph_word);
    free(hyph_list);
    free(hyph_link);

    // initialize_more_variables @ 3277
    free(native_text);

    // Free arrays allocated in load_fmt_file
    free(yhash);
    free(eqtb);
    free(mem);
    free(str_start);
    free(str_pool);
    free(font_info);

    free(font_mapping);
    free(font_layout_engine);
    free(font_flags);
    free(font_letter_space);
    free(font_check);
    free(font_size);
    free(font_dsize);
    free(font_params);
    free(font_name);
    free(font_area);
    free(font_bc);
    free(font_ec);
    free(font_glue);
    free(hyphen_char);
    free(skew_char);
    free(bchar_label);
    free(font_bchar);
    free(font_false_bchar);
    free(char_base);
    free(width_base);
    free(height_base);
    free(depth_base);
    free(italic_base);
    free(lig_kern_base);
    free(kern_base);
    free(exten_base);
    free(param_base);

    trie_trl = mfree(trie_trl);
    trie_tro = mfree(trie_tro);
    trie_trc = mfree(trie_trc);
}

tt_history_t
tt_run_engine(const char *dump_name, const char *input_file_name, time_t build_date)
{
    int32_t font_k;

    /* Miscellaneous initializations that were mostly originally done in the
     * main() driver routines. */

    /* Get our stdout handle */

    rust_stdout = ttstub_output_open_stdout ();

    size_t len = strlen (dump_name);
    TEX_format_default = xmalloc (len + 1);
    strcpy (TEX_format_default, dump_name);
    format_default_length = len;

    /* Not sure why these get custom initializations. */

    if (file_line_error_style_p < 0)
        file_line_error_style_p = 0;

    /* These various parameters were configurable in web2c TeX. We don't
     * bother to allow that. */

    pool_size = 6250000L;
    string_vacancies = 90000L;
    pool_free = 47500L;
    max_strings = 565536L;
    strings_free = 100;
    font_mem_size = 8000000L;
    font_max = 9000;
    trie_size = 1000000L;
    hyph_size = 8191;
    buf_size = 200000L;
    nest_size = 500;
    max_in_open = 15;
    param_size = 10000;
    save_size = 80000L;
    stack_size = 5000;
    error_line = 79;
    half_error_line = 50;
    max_print_line = 79;
    hash_extra = 600000L;
    expand_depth = 10000;

    /* Allocate many of our big arrays. */

    buffer = xmalloc_array(UnicodeScalar, buf_size);
    nest = xmalloc_array(list_state_record, nest_size);
    save_stack = xmalloc_array(memory_word, save_size);
    input_stack = xmalloc_array(input_state_t, stack_size);
    input_file = xmalloc_array(UFILE *, max_in_open);
    line_stack = xmalloc_array(int32_t, max_in_open);
    eof_seen = xmalloc_array(bool, max_in_open);
    grp_stack = xmalloc_array(save_pointer, max_in_open);
    if_stack = xmalloc_array(int32_t, max_in_open);
    source_filename_stack = xmalloc_array(str_number, max_in_open);
    full_source_filename_stack = xmalloc_array(str_number, max_in_open);
    param_stack = xmalloc_array(int32_t, param_size);
    hyph_word = xmalloc_array(str_number, hyph_size);
    hyph_list = xmalloc_array(int32_t, hyph_size);
    hyph_link = xmalloc_array(hyph_pointer, hyph_size);

    /* First bit of initex handling: more allocations. */

    if (in_initex_mode) {
        mem = xmalloc_array(memory_word, MEM_TOP + 1);
        eqtb_top = EQTB_SIZE + hash_extra;

        if (hash_extra == 0)
            hash_top = UNDEFINED_CONTROL_SEQUENCE;
        else
            hash_top = eqtb_top;

        yhash = xmalloc_array(b32x2, 1 + hash_top - hash_offset);
        hash = yhash - hash_offset;
        hash[HASH_BASE].s0 = 0;
        hash[HASH_BASE].s1 = 0;

        for (hash_used = HASH_BASE + 1; hash_used <= hash_top; hash_used++)
            hash[hash_used] = hash[HASH_BASE];

        eqtb = xcalloc_array(memory_word, eqtb_top);
        str_start = xmalloc_array(pool_pointer, max_strings);
        str_pool = xmalloc_array(packed_UTF16_code, pool_size);
        font_info = xmalloc_array(memory_word, font_mem_size);
    }

    /* Sanity-check various invariants. */

    history = HISTORY_FATAL_ERROR;
    bad = 0;

    if (half_error_line < 30 || half_error_line > error_line - 15)
        bad = 1;
    if (max_print_line < 60)
        bad = 2;
    if (1100 > MEM_TOP)
        bad = 4;
    if (HASH_PRIME > HASH_SIZE)
        bad = 5;
    if (max_in_open >= 128)
        bad = 6;
    if (MEM_TOP < 267)
        bad = 7;
    if (MIN_HALFWORD > 0)
        bad = 12;
    if (MAX_FONT_MAX < MIN_HALFWORD || MAX_FONT_MAX > MAX_HALFWORD)
        bad = 15;
    if (font_max > FONT_BASE + 9000)
        bad = 16;
    if (save_size > MAX_HALFWORD || max_strings > MAX_HALFWORD)
        bad = 17;
    if (buf_size > MAX_HALFWORD)
        bad = 18;
    if (CS_TOKEN_FLAG + EQTB_SIZE + hash_extra > MAX_HALFWORD)
        bad = 21;
    if (hash_offset < 0 || hash_offset > HASH_BASE)
        bad = 42;
    if (format_default_length > INT32_MAX)
        bad = 31;
    if (2 * MAX_HALFWORD < MEM_TOP)
        bad = 41;

    if (bad > 0)
        _tt_abort ("failed internal consistency check #%d", bad);

    /* OK, ready to keep on initializing. */

    initialize_more_variables();

    if (in_initex_mode) {
        get_strings_started();
        initialize_more_initex_variables();
        initialize_primitives();
        init_str_ptr = str_ptr;
        init_pool_ptr = pool_ptr;
    }

    /*55:*/
    initialize_math_variables();
    initialize_pagebuilder_variables();
    initialize_shipout_variables();

    get_seconds_and_micros(&epochseconds, &microseconds);
    init_start_time(build_date);

    selector = SELECTOR_TERM_ONLY;
    tally = 0;
    term_offset = 0;
    file_offset = 0;
    job_name = 0;
    name_in_progress = false;
    log_opened = false;

    if (semantic_pagination_enabled)
        output_file_extension = ".spx";
    else
        output_file_extension = ".xdv";

    input_ptr = 0;
    max_in_stack = 0;
    source_filename_stack[0] = 0;
    full_source_filename_stack[0] = 0;
    in_open = 0;
    open_parens = 0;
    max_buf_stack = 0;
    grp_stack[0] = 0;
    if_stack[0] = TEX_NULL;
    param_ptr = 0;
    max_param_stack = 0;
    used_tectonic_coda_tokens = false;
    gave_char_warning_help = false;

    memset(buffer, 0, buf_size * sizeof(buffer[0]));
    first = 0;

    scanner_status = NORMAL;
    warning_index = TEX_NULL;
    first = 1;
    cur_input.state = NEW_LINE;
    cur_input.start = 1;
    cur_input.index = 0;
    line = 0;
    cur_input.name = 0;
    force_eof = false;
    align_state = 1000000L;

    init_io();

    if (in_initex_mode) {
        no_new_control_sequence = false;

        primitive("XeTeXpicfile", EXTENSION, PIC_FILE_CODE);
        primitive("XeTeXpdffile", EXTENSION, PDF_FILE_CODE);
        primitive("XeTeXglyph", EXTENSION, GLYPH_CODE);
        primitive("XeTeXlinebreaklocale", EXTENSION, XETEX_LINEBREAK_LOCALE_EXTENSION_CODE);
        primitive("pdfsavepos", EXTENSION, PDF_SAVE_POS_NODE);

        primitive("lastnodetype", LAST_ITEM, LAST_NODE_TYPE_CODE);
        primitive("eTeXversion", LAST_ITEM, ETEX_VERSION_CODE);

        primitive("eTeXrevision", CONVERT, ETEX_REVISION_CODE);

        primitive("XeTeXversion", LAST_ITEM, XETEX_VERSION_CODE);

        primitive("XeTeXrevision", CONVERT, XETEX_REVISION_CODE);

        primitive("XeTeXcountglyphs", LAST_ITEM, XETEX_COUNT_GLYPHS_CODE);
        primitive("XeTeXcountvariations", LAST_ITEM, XETEX_COUNT_VARIATIONS_CODE);
        primitive("XeTeXvariation", LAST_ITEM, XETEX_VARIATION_CODE);
        primitive("XeTeXfindvariationbyname", LAST_ITEM, XETEX_FIND_VARIATION_BY_NAME_CODE);
        primitive("XeTeXvariationmin", LAST_ITEM, XETEX_VARIATION_MIN_CODE);
        primitive("XeTeXvariationmax", LAST_ITEM, XETEX_VARIATION_MAX_CODE);
        primitive("XeTeXvariationdefault", LAST_ITEM, XETEX_VARIATION_DEFAULT_CODE);
        primitive("XeTeXcountfeatures", LAST_ITEM, XETEX_COUNT_FEATURES_CODE);
        primitive("XeTeXfeaturecode", LAST_ITEM, XETEX_FEATURE_CODE_CODE);
        primitive("XeTeXfindfeaturebyname", LAST_ITEM, XETEX_FIND_FEATURE_BY_NAME_CODE);
        primitive("XeTeXisexclusivefeature", LAST_ITEM, XETEX_IS_EXCLUSIVE_FEATURE_CODE);
        primitive("XeTeXcountselectors", LAST_ITEM, XETEX_COUNT_SELECTORS_CODE);
        primitive("XeTeXselectorcode", LAST_ITEM, XETEX_SELECTOR_CODE_CODE);
        primitive("XeTeXfindselectorbyname", LAST_ITEM, XETEX_FIND_SELECTOR_BY_NAME_CODE);
        primitive("XeTeXisdefaultselector", LAST_ITEM, XETEX_IS_DEFAULT_SELECTOR_CODE);

        primitive("XeTeXvariationname", CONVERT, XETEX_VARIATION_NAME_CODE);
        primitive("XeTeXfeaturename", CONVERT, XeTeX_feature_name);
        primitive("XeTeXselectorname", CONVERT, XeTeX_selector_name);

        primitive("XeTeXOTcountscripts", LAST_ITEM, XETEX_OT_COUNT_SCRIPTS_CODE);
        primitive("XeTeXOTcountlanguages", LAST_ITEM, XETEX_OT_COUNT_LANGUAGES_CODE);
        primitive("XeTeXOTcountfeatures", LAST_ITEM, XETEX_OT_COUNT_FEATURES_CODE);
        primitive("XeTeXOTscripttag", LAST_ITEM, XETEX_OT_SCRIPT_CODE);
        primitive("XeTeXOTlanguagetag", LAST_ITEM, XETEX_OT_LANGUAGE_CODE);
        primitive("XeTeXOTfeaturetag", LAST_ITEM, XETEX_OT_FEATURE_CODE);
        primitive("XeTeXcharglyph", LAST_ITEM, XETEX_MAP_CHAR_TO_GLYPH_CODE);
        primitive("XeTeXglyphindex", LAST_ITEM, XETEX_GLYPH_INDEX_CODE);
        primitive("XeTeXglyphbounds", LAST_ITEM, XETEX_GLYPH_BOUNDS_CODE);

        primitive("XeTeXglyphname", CONVERT, XETEX_GLYPH_NAME_CODE);

        primitive("XeTeXfonttype", LAST_ITEM, XETEX_FONT_TYPE_CODE);
        primitive("XeTeXfirstfontchar", LAST_ITEM, XETEX_FIRST_CHAR_CODE);
        primitive("XeTeXlastfontchar", LAST_ITEM, XETEX_LAST_CHAR_CODE);
        primitive("XeTeXpdfpagecount", LAST_ITEM, XETEX_PDF_PAGE_COUNT_CODE);
        /* everyeof moved to be with other assign_toks */

        primitive("tracingassigns", ASSIGN_INT, INT_BASE + INT_PAR__tracing_assigns);
        primitive("tracinggroups", ASSIGN_INT, INT_BASE + INT_PAR__tracing_groups);
        primitive("tracingifs", ASSIGN_INT, INT_BASE + INT_PAR__tracing_ifs);
        primitive("tracingscantokens", ASSIGN_INT, INT_BASE + INT_PAR__tracing_scan_tokens);
        primitive("tracingnesting", ASSIGN_INT, INT_BASE + INT_PAR__tracing_nesting);
        primitive("predisplaydirection", ASSIGN_INT, INT_BASE + INT_PAR__pre_display_correction);
        primitive("lastlinefit", ASSIGN_INT, INT_BASE + INT_PAR__last_line_fit);
        primitive("savingvdiscards", ASSIGN_INT, INT_BASE + INT_PAR__saving_vdiscards);
        primitive("savinghyphcodes", ASSIGN_INT, INT_BASE + INT_PAR__saving_hyphs);

        primitive("currentgrouplevel", LAST_ITEM, CURRENT_GROUP_LEVEL_CODE);
        primitive("currentgrouptype", LAST_ITEM, CURRENT_GROUP_TYPE_CODE);
        primitive("currentiflevel", LAST_ITEM, CURRENT_IF_LEVEL_CODE);
        primitive("currentiftype", LAST_ITEM, CURRENT_IF_TYPE_CODE);
        primitive("currentifbranch", LAST_ITEM, CURRENT_IF_BRANCH_CODE);
        primitive("fontcharwd", LAST_ITEM, FONT_CHAR_WD_CODE);
        primitive("fontcharht", LAST_ITEM, FONT_CHAR_HT_CODE);
        primitive("fontchardp", LAST_ITEM, FONT_CHAR_DP_CODE);
        primitive("fontcharic", LAST_ITEM, FONT_CHAR_IC_CODE);
        primitive("parshapelength", LAST_ITEM, PAR_SHAPE_LENGTH_CODE);
        primitive("parshapeindent", LAST_ITEM, PAR_SHAPE_INDENT_CODE);
        primitive("parshapedimen", LAST_ITEM, PAR_SHAPE_DIMEN_CODE);

        primitive("showgroups", XRAY, SHOW_GROUPS);
        primitive("showtokens", XRAY, SHOW_TOKENS);

        primitive("unexpanded", THE, 1);
        primitive("detokenize", THE, SHOW_TOKENS);

        primitive("showifs", XRAY, SHOW_IFS);

        primitive("interactionmode", SET_PAGE_INT, 2);

        primitive("middle", LEFT_RIGHT, 1);

        primitive("suppressfontnotfounderror", ASSIGN_INT, INT_BASE + INT_PAR__suppress_fontnotfound_error);

        primitive("TeXXeTstate", ASSIGN_INT, INT_BASE + INT_PAR__texxet);
        primitive("XeTeXupwardsmode", ASSIGN_INT, INT_BASE + INT_PAR__xetex_upwards);
        primitive("XeTeXuseglyphmetrics", ASSIGN_INT, INT_BASE + INT_PAR__xetex_use_glyph_metrics);
        primitive("XeTeXinterchartokenstate", ASSIGN_INT, INT_BASE + INT_PAR__xetex_inter_char_tokens);
        primitive("XeTeXdashbreakstate", ASSIGN_INT, INT_BASE + INT_PAR__xetex_dash_break);
        primitive("XeTeXinputnormalization", ASSIGN_INT, INT_BASE + INT_PAR__xetex_input_normalization);
        primitive("XeTeXtracingfonts", ASSIGN_INT, INT_BASE + INT_PAR__xetex_tracing_fonts);
        primitive("XeTeXinterwordspaceshaping", ASSIGN_INT, INT_BASE + INT_PAR__xetex_interword_space_shaping);
        primitive("XeTeXgenerateactualtext", ASSIGN_INT, INT_BASE + INT_PAR__xetex_generate_actual_text);
        primitive("XeTeXhyphenatablelength", ASSIGN_INT, INT_BASE + INT_PAR__xetex_hyphenatable_length);
        primitive("pdfoutput", ASSIGN_INT, INT_BASE + INT_PAR__pdfoutput);

        primitive("XeTeXinputencoding", EXTENSION, XETEX_INPUT_ENCODING_EXTENSION_CODE);
        primitive("XeTeXdefaultencoding", EXTENSION, XETEX_DEFAULT_ENCODING_EXTENSION_CODE);

        primitive("beginL", VALIGN, BEGIN_L_CODE);
        primitive("endL", VALIGN, END_L_CODE);
        primitive("beginR", VALIGN, BEGIN_R_CODE);
        primitive("endR", VALIGN, END_R_CODE);

        primitive("scantokens", INPUT, 2);
        primitive("readline", READ_TO_CS, 1);
        primitive("unless", EXPAND_AFTER, 1);

        primitive("ifdefined", IF_TEST, IF_DEF_CODE);
        primitive("ifcsname", IF_TEST, IF_CS_CODE);
        primitive("iffontchar", IF_TEST, IF_FONT_CHAR_CODE);
        primitive("ifincsname", IF_TEST, IF_IN_CSNAME_CODE);

        primitive("protected", PREFIX, 8);

        primitive("numexpr", LAST_ITEM, ETEX_EXPR + 0);
        primitive("dimexpr", LAST_ITEM, ETEX_EXPR + 1);
        primitive("glueexpr", LAST_ITEM, ETEX_EXPR + 2);
        primitive("muexpr", LAST_ITEM, ETEX_EXPR + 3);
        primitive("gluestretchorder", LAST_ITEM, GLUE_STRETCH_ORDER_CODE);
        primitive("glueshrinkorder", LAST_ITEM, GLUE_SHRINK_ORDER_CODE);
        primitive("gluestretch", LAST_ITEM, GLUE_STRETCH_CODE);
        primitive("glueshrink", LAST_ITEM, GLUE_SHRINK_CODE);
        primitive("mutoglue", LAST_ITEM, MU_TO_GLUE_CODE);
        primitive("gluetomu", LAST_ITEM, GLUE_TO_MU_CODE);

        primitive("marks", MARK, 5);
        primitive("topmarks", TOP_BOT_MARK, TOP_MARK_CODE + 5);
        primitive("firstmarks", TOP_BOT_MARK, FIRST_MARK_CODE + 5);
        primitive("botmarks", TOP_BOT_MARK, BOT_MARK_CODE + 5);
        primitive("splitfirstmarks", TOP_BOT_MARK, SPLIT_FIRST_MARK_CODE + 5);
        primitive("splitbotmarks", TOP_BOT_MARK, SPLIT_BOT_MARK_CODE + 5);

        primitive("pagediscards", UN_VBOX, LAST_BOX_CODE);
        primitive("splitdiscards", UN_VBOX, VSPLIT_CODE);

        primitive("interlinepenalties", SET_SHAPE, INTER_LINE_PENALTIES_LOC);
        primitive("clubpenalties", SET_SHAPE, CLUB_PENALTIES_LOC);
        primitive("widowpenalties", SET_SHAPE, WIDOW_PENALTIES_LOC);
        primitive("displaywidowpenalties", SET_SHAPE, DISPLAY_WIDOW_PENALTIES_LOC);

        max_reg_num = 32767;
        max_reg_help_line = "A register number must be between 0 and 32767.";
    }

    no_new_control_sequence = true;

    if (!in_initex_mode) {
        if (!load_fmt_file())
            return history;
    }

    if (INTPAR(end_line_char) < 0 || INTPAR(end_line_char) > BIGGEST_CHAR)
        cur_input.limit--;
    else
        buffer[cur_input.limit] = INTPAR(end_line_char);

    if (in_initex_mode) {
        /* TeX initializes with the real date and time, but for format file
         * reproducibility we do this: */
        INTPAR(time) = 0;
        INTPAR(day) = 0;
        INTPAR(month) = 0;
        INTPAR(year) = 0;
    } else {
        get_date_and_time(build_date,
                          &(INTPAR(time)),
                          &(INTPAR(day)),
                          &(INTPAR(month)),
                          &(INTPAR(year)));
    }

    if (trie_not_ready) {
        trie_trl = xmalloc_array(trie_pointer, trie_size);
        trie_tro = xmalloc_array(trie_pointer, trie_size);
        trie_trc = xmalloc_array(uint16_t, trie_size);
        trie_c = xmalloc_array(packed_UTF16_code, trie_size);
        trie_o = xmalloc_array(trie_opcode, trie_size);
        trie_l = xmalloc_array(trie_pointer, trie_size);
        trie_r = xmalloc_array(trie_pointer, trie_size);
        trie_hash = xmalloc_array(trie_pointer, trie_size);
        trie_taken = xmalloc_array(bool, trie_size);
        trie_l[0] = 0;
        trie_c[0] = 0;
        trie_ptr = 0;
        trie_r[0] = 0;
        hyph_start = 0;
        font_mapping = xcalloc_array(void *, font_max);
        font_layout_engine = xcalloc_array(void *, font_max);
        font_flags = xcalloc_array(char, font_max);
        font_letter_space = xcalloc_array(scaled_t, font_max);
        font_check = xcalloc_array(b16x4, font_max);
        font_size = xcalloc_array(scaled_t, font_max);
        font_dsize = xcalloc_array(scaled_t, font_max);
        font_params = xcalloc_array(font_index, font_max);
        font_name = xcalloc_array(str_number, font_max);
        font_area = xcalloc_array(str_number, font_max);
        font_bc = xcalloc_array(UTF16_code, font_max);
        font_ec = xcalloc_array(UTF16_code, font_max);
        font_glue = xcalloc_array(int32_t, font_max);
        hyphen_char = xcalloc_array(int32_t, font_max);
        skew_char = xcalloc_array(int32_t, font_max);
        bchar_label = xcalloc_array(font_index, font_max);
        font_bchar = xcalloc_array(nine_bits, font_max);
        font_false_bchar = xcalloc_array(nine_bits, font_max);
        char_base = xcalloc_array(int32_t, font_max);
        width_base = xcalloc_array(int32_t, font_max);
        height_base = xcalloc_array(int32_t, font_max);
        depth_base = xcalloc_array(int32_t, font_max);
        italic_base = xcalloc_array(int32_t, font_max);
        lig_kern_base = xcalloc_array(int32_t, font_max);
        kern_base = xcalloc_array(int32_t, font_max);
        exten_base = xcalloc_array(int32_t, font_max);
        param_base = xcalloc_array(int32_t, font_max);
        font_ptr = FONT_BASE;
        fmem_ptr = 7;
        font_name[FONT_BASE] = maketexstring("nullfont");
        font_area[FONT_BASE] = EMPTY_STRING;
        hyphen_char[FONT_BASE] = '-';
        skew_char[FONT_BASE] = -1;
        bchar_label[FONT_BASE] = NON_ADDRESS;
        font_bchar[FONT_BASE] = TOO_BIG_CHAR;
        font_false_bchar[FONT_BASE] = TOO_BIG_CHAR;
        font_bc[FONT_BASE] = 1;
        font_ec[FONT_BASE] = 0;
        font_size[FONT_BASE] = 0;
        font_dsize[FONT_BASE] = 0;
        char_base[FONT_BASE] = 0;
        width_base[FONT_BASE] = 0;
        height_base[FONT_BASE] = 0;
        depth_base[FONT_BASE] = 0;
        italic_base[FONT_BASE] = 0;
        lig_kern_base[FONT_BASE] = 0;
        kern_base[FONT_BASE] = 0;
        exten_base[FONT_BASE] = 0;
        font_glue[FONT_BASE] = TEX_NULL;
        font_params[FONT_BASE] = 7;
        font_mapping[FONT_BASE] = 0;
        param_base[FONT_BASE] = -1;

        for (font_k = 0; font_k <= 6; font_k++)
            font_info[font_k].b32.s1 = 0;
    }

    font_used = xmalloc_array(bool, font_max);
    for (font_k = 0; font_k <= font_max; font_k++)
        font_used[font_k] = false;

    random_seed = (microseconds * 1000) + (epochseconds % 1000000L);
    init_randoms(random_seed);

    if (interaction == BATCH_MODE)
        selector = SELECTOR_NO_PRINT;
    else
        selector = SELECTOR_TERM_ONLY; /*:79*/

    if (semantic_pagination_enabled)
        INTPAR(xetex_generate_actual_text) = 1;

    pdf_files_init();
    synctex_init_command();
    start_input(input_file_name);
    history = HISTORY_SPOTLESS;
    main_control();
    final_cleanup();
    close_files_and_terminate();

    tt_cleanup();

    return history;
}
