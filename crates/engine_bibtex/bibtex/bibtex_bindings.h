#ifndef BIBTEX_BINDINGS_H
#define BIBTEX_BINDINGS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "tectonic_bridge_core.h"

typedef enum {
  HISTORY_SPOTLESS = 0,
  HISTORY_WARNING_ISSUED = 1,
  HISTORY_ERROR_ISSUED = 2,
  HISTORY_FATAL_ERROR = 3,
  HISTORY_ABORTED = 4,
} History;

typedef struct PeekableInput PeekableInput;

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

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

History bibtex_main(Bibtex *ctx, const char *aux_file_name);

extern History tt_engine_bibtex_main(ttbc_state_t *api, Bibtex *ctx, const char *aux_name);

History get_history(void);

uint32_t err_count(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* BIBTEX_BINDINGS_H */
