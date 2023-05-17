#ifndef BIBTEX_BINDINGS_H
#define BIBTEX_BINDINGS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "tectonic_bridge_core.h"

typedef enum {
  BUF_TY_BASE,
  BUF_TY_SV,
  BUF_TY_EX,
  BUF_TY_OUT,
  BUF_TY_NAME_TOK,
} BufTy;

typedef enum {
  HISTORY_SPOTLESS = 0,
  HISTORY_WARNING_ISSUED = 1,
  HISTORY_ERROR_ISSUED = 2,
  HISTORY_FATAL_ERROR = 3,
  HISTORY_ABORTED = 4,
} History;

typedef enum {
  LEX_CLASS_ILLEGAL = 0,
  LEX_CLASS_WHITESPACE = 1,
  LEX_CLASS_ALPHA = 2,
  LEX_CLASS_NUMERIC = 3,
  LEX_CLASS_SEP = 4,
  LEX_CLASS_OTHER = 5,
} LexClass;

typedef enum {
  SCAN_RES_ID_NULL = 0,
  SCAN_RES_SPECIFIED_CHAR_ADJACENT = 1,
  SCAN_RES_OTHER_CHAR_ADJACENT = 2,
  SCAN_RES_WHITESPACE_ADJACENT = 3,
} ScanRes;

typedef enum {
  STK_TYPE_INTEGER = 0,
  STK_TYPE_STRING = 1,
  STK_TYPE_FUNCTION = 2,
  STK_TYPE_MISSING = 3,
  STK_TYPE_ILLEGAL = 4,
} StkType;

typedef int32_t StrNumber;

typedef uint8_t ASCIICode;

typedef ASCIICode *BufType;

typedef int32_t BufPointer;

typedef struct {
  int min_crossrefs;
} BibtexConfig;

typedef int32_t CiteNumber;

typedef int32_t HashPointer2;

typedef struct {
  StkType typ;
  int32_t lit;
} ExecVal;

typedef struct {
  ttbc_input_handle_t *handle;
  int peek_char;
  bool saw_eof;
} PeekableInput;

typedef struct {
  PeekableInput *bst_file;
  StrNumber bst_str;
  int32_t bst_line_num;
  int32_t num_bib_files;
  int32_t num_preamble_strings;
} BstCtx;

typedef struct {
  BstCtx *bst_ctx;
  ExecVal pop1;
  ExecVal pop2;
  ExecVal pop3;
  ExecVal *lit_stack;
  int32_t lit_stk_size;
  int32_t lit_stk_ptr;
  bool mess_with_entries;
  StrNumber bib_str_ptr;
} ExecCtx;

typedef struct {
  ASCIICode *name_of_file;
  int32_t name_length;
} NameAndLen;

typedef uintptr_t PoolPointer;

typedef int32_t AuxNumber;

typedef int32_t BibNumber;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

extern const LexClass LEX_CLASS[256];

extern const int32_t CHAR_WIDTH[256];

void reset_all(void);

bool bib_str_eq_buf(StrNumber s, BufType buf, BufPointer bf_ptr, BufPointer len);

void lower_case(BufType buf, BufPointer bf_ptr, BufPointer len);

void upper_case(BufType buf, BufPointer bf_ptr, BufPointer len);

BufPointer int_to_ascii(int32_t the_int, BufTy int_buf, BufPointer int_begin);

extern History tt_engine_bibtex_main(ttbc_state_t *api,
                                     const BibtexConfig *cfg,
                                     const char *aux_name);

int32_t bib_buf_size(void);

BufType bib_buf(BufTy ty);

ASCIICode bib_buf_at(BufTy ty, BufPointer num);

ASCIICode bib_buf_at_offset(BufTy ty, uintptr_t num);

BufPointer bib_buf_offset(BufTy ty, uintptr_t num);

void bib_set_buf_offset(BufTy ty, uintptr_t num, BufPointer offset);

BufPointer bib_buf_len(BufTy ty);

void bib_set_buf_len(BufTy ty, BufPointer len);

void buffer_overflow(void);

void quick_sort(CiteNumber left_end, CiteNumber right_end);

StrNumber cite_list(CiteNumber num);

void set_cite_list(CiteNumber num, StrNumber str);

CiteNumber cite_ptr(void);

void set_cite_ptr(CiteNumber num);

void check_cite_overflow(CiteNumber last_cite);

uintptr_t max_cites(void);

StrNumber cite_info(CiteNumber num);

void set_cite_info(CiteNumber num, StrNumber info);

HashPointer2 type_list(CiteNumber num);

void set_type_list(CiteNumber num, HashPointer2 ty);

bool entry_exists(CiteNumber num);

void set_entry_exists(CiteNumber num, bool exists);

bool print_lit(const StrNumber *hash_text, ExecVal val);

bool print_stk_lit(const StrNumber *hash_text, ExecVal val);

bool print_wrong_stk_lit(const StrNumber *hash_text, ExecCtx *ctx, ExecVal val, StkType typ2);

bool bst_ex_warn_print(const ExecCtx *ctx);

bool bst_ln_num_print(const BstCtx *bst_ctx);

bool print_bst_name(const BstCtx *bst_ctx);

void push_lit_stk(ExecCtx *ctx, ExecVal val);

bool pop_lit_stk(ExecCtx *ctx, ExecVal *out);

History get_history(void);

void mark_warning(void);

void mark_error(void);

uint32_t err_count(void);

ttbc_output_handle_t *init_log_file(const char *file);

ttbc_output_handle_t *standard_output(void);

ttbc_output_handle_t *bib_log_file(void);

void putc_log(int c);

void puts_log(const char *str);

void ttstub_puts(ttbc_output_handle_t *handle, const char *s);

void print_overflow(void);

void print_confusion(void);

void print_a_token(void);

void print_bad_input_line(void);

void print_skipping_whatever_remains(void);

bool out_pool_str(ttbc_output_handle_t *handle, StrNumber s);

bool print_a_pool_str(StrNumber s);

void sam_wrong_file_name_print(NameAndLen file);

bool print_aux_name(void);

bool log_pr_aux_name(void);

bool aux_err_print(void);

bool aux_err_illegal_another_print(int32_t cmd_num);

void aux_err_no_right_brace_print(void);

void aux_err_stuff_after_right_brace_print(void);

void aux_err_white_space_in_argument_print(void);

void aux_end1_err_print(void);

bool aux_end2_err_print(void);

PeekableInput *peekable_open(const char *path, ttbc_file_format format);

int peekable_close(PeekableInput *peekable);

bool tectonic_eof(PeekableInput *peekable);

bool input_ln(PeekableInput *peekable);

bool str_ends_with(StrNumber s, StrNumber ext);

bool bib_str_eq_str(StrNumber s1, StrNumber s2);

void pool_overflow(void);

ASCIICode bib_str_pool(PoolPointer idx);

void bib_set_str_pool(PoolPointer idx, ASCIICode code);

StrNumber bib_str_ptr(void);

void bib_set_str_ptr(StrNumber ptr);

PoolPointer bib_str_start(StrNumber s);

void bib_set_str_start(StrNumber s, PoolPointer ptr);

uintptr_t bib_pool_size(void);

uintptr_t bib_max_strings(void);

PoolPointer bib_pool_ptr(void);

void bib_set_pool_ptr(PoolPointer ptr);

bool scan1(ASCIICode char1);

bool scan1_white(ASCIICode char1);

bool scan2(ASCIICode char1, ASCIICode char2);

bool scan2_white(ASCIICode char1, ASCIICode char2);

bool scan3(ASCIICode char1, ASCIICode char2, ASCIICode char3);

bool scan_alpha(void);

bool scan_white_space(void);

ScanRes scan_identifier(ASCIICode char1, ASCIICode char2, ASCIICode char3);

bool scan_nonneg_integer(void);

bool scan_integer(int32_t *token_value);

StrNumber cur_aux(void);

void set_cur_aux(StrNumber num);

PeekableInput *cur_aux_file(void);

void set_cur_aux_file(PeekableInput *file);

int32_t cur_aux_ln(void);

void set_cur_aux_ln(int32_t ln);

AuxNumber aux_ptr(void);

void set_aux_ptr(AuxNumber num);

StrNumber cur_bib(void);

void set_cur_bib(StrNumber num);

PeekableInput *cur_bib_file(void);

void set_cur_bib_file(PeekableInput *input);

BibNumber bib_ptr(void);

void set_bib_ptr(BibNumber num);

void check_bib_files(BibNumber ptr);

void add_preamble(StrNumber num);

StrNumber cur_preamble(void);

BibNumber preamble_ptr(void);

void set_preamble_ptr(BibNumber num);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* BIBTEX_BINDINGS_H */
