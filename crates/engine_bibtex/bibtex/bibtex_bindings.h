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
  BUF_TY_NAME_SEP,
} BufTy;

typedef enum {
  CResult_Error,
  CResult_Recover,
  CResult_Ok,
} CResult;

typedef enum {
  HISTORY_SPOTLESS = 0,
  HISTORY_WARNING_ISSUED = 1,
  HISTORY_ERROR_ISSUED = 2,
  HISTORY_FATAL_ERROR = 3,
  HISTORY_ABORTED = 4,
} History;

typedef struct PeekableInput PeekableInput;

typedef struct XBuf_ExecVal XBuf_ExecVal;

typedef enum {
  CResultInt_Error,
  CResultInt_Recover,
  CResultInt_Ok,
} CResultInt_Tag;

typedef struct {
  CResultInt_Tag tag;
  union {
    struct {
      int32_t ok;
    };
  };
} CResultInt;

typedef struct {
  uint32_t min_crossrefs;
  bool verbose;
} BibtexConfig;

typedef uintptr_t StrNumber;

typedef uintptr_t HashPointer;

typedef struct {
  BibtexConfig config;
  PeekableInput *bst_file;
  StrNumber bst_str;
  uintptr_t bst_line_num;
  ttbc_output_handle_t *bbl_file;
  uintptr_t bbl_line_num;
  uintptr_t num_bib_files;
  uintptr_t num_preamble_strings;
  uintptr_t impl_fn_num;
  uintptr_t cite_xptr;
  bool bib_seen;
  bool bst_seen;
  bool citation_seen;
  bool entry_seen;
  bool read_seen;
  bool read_performed;
  bool reading_completed;
  bool all_entries;
  HashPointer b_default;
  HashPointer s_null;
  HashPointer s_default;
  HashPointer s_aux_extension;
} Bibtex;

typedef struct {
  Bibtex *glbl_ctx;
  HashPointer _default;
  XBuf_ExecVal *lit_stack;
  uintptr_t lit_stk_ptr;
  bool mess_with_entries;
  /**
   * Pointer to the current top of the string pool, used to optimize certain string operations
   */
  StrNumber bib_str_ptr;
} ExecCtx;

typedef uintptr_t BufPointer;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void reset_all(void);

CResultInt initialize(Bibtex *ctx, const char *aux_file_name);

extern History tt_engine_bibtex_main(ttbc_state_t *api, Bibtex *ctx, const char *aux_name);

PeekableInput *cur_aux_file(void);

int32_t cur_aux_ln(void);

void set_cur_aux_ln(int32_t ln);

CResult get_aux_command_and_process(Bibtex *ctx);

bool pop_the_aux_stack(void);

CResult last_check_for_aux_errors(Bibtex *ctx);

int32_t bib_line_num(void);

CResult get_bst_command_and_process(ExecCtx *ctx);

void bib_set_buf_offset(BufTy ty, uintptr_t num, BufPointer offset);

BufPointer bib_buf_len(BufTy ty);

ExecCtx init_exec_ctx(Bibtex *glbl_ctx);

int32_t hash_size(void);

uintptr_t hash_prime(void);

History get_history(void);

uint32_t err_count(void);

bool init_standard_output(void);

void bib_close_log(void);

void bib_log_prints(const char *str);

void puts_log(const char *str);

void print_confusion(void);

CResult print_aux_name(void);

CResult log_pr_aux_name(void);

CResult print_bib_name(void);

int peekable_close(PeekableInput *peekable);

bool input_ln(PeekableInput *peekable);

uintptr_t bib_max_strings(void);

bool eat_bst_white_space(Bibtex *ctx);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* BIBTEX_BINDINGS_H */
