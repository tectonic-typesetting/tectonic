/* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_CORE_BRIDGE_H
#define TECTONIC_CORE_BRIDGE_H

#include "core-foundation.h"

/* Both XeTeX and bibtex use this enum: */

typedef enum {
    HISTORY_SPOTLESS = 0,
    HISTORY_WARNING_ISSUED = 1,
    HISTORY_ERROR_ISSUED = 2,
    HISTORY_FATAL_ERROR = 3
} tt_history_t;

/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */

typedef enum
{
    TTIF_TFM = 3,
    TTIF_AFM = 4,
    TTIF_BIB = 6,
    TTIF_BST = 7,
    TTIF_CNF = 8,
    TTIF_FORMAT = 10,
    TTIF_FONTMAP = 11,
    TTIF_OFM = 20,
    TTIF_OVF = 23,
    TTIF_PICT = 25,
    TTIF_TEX = 26,
    TTIF_TEX_PS_HEADER = 30,
    TTIF_TYPE1 = 32,
    TTIF_VF = 33,
    TTIF_TRUETYPE = 36,
    TTIF_BINARY = 40,
    TTIF_MISCFONTS = 41,
    TTIF_ENC = 44,
    TTIF_CMAP = 45,
    TTIF_SFD = 46,
    TTIF_OPENTYPE = 47,
    TTIF_TECTONIC_PRIMARY = 59, /* quasi-hack to get the primary input */
} tt_input_format_type;

typedef void *rust_output_handle_t;
typedef void *rust_input_handle_t;


/* Bridge API. Keep synchronized with src/engines/mod.rs. */

typedef struct tt_bridge_api_t {
    void *context;

    void (*issue_warning)(void *context, char const *text);
    void (*issue_error)(void *context, char const *text);

    int (*get_file_md5)(void *context, char const *path, char *digest);
    int (*get_data_md5)(void *context, char const *data, size_t len, char *digest);

    rust_output_handle_t (*output_open)(void *context, char const *path, int is_gz);
    rust_output_handle_t (*output_open_stdout)(void *context);
    int (*output_putc)(void *context, rust_output_handle_t handle, int c);
    size_t (*output_write)(void *context, rust_output_handle_t handle, const char *data, size_t len);
    int (*output_flush)(void *context, rust_output_handle_t handle);
    int (*output_close)(void *context, rust_output_handle_t handle);

    rust_input_handle_t (*input_open)(void *context, char const *path, tt_input_format_type format, int is_gz);
    rust_input_handle_t (*input_open_primary)(void *context);
    size_t (*input_get_size)(void *context, rust_input_handle_t handle);
    size_t (*input_seek)(void *context, rust_input_handle_t handle, ssize_t offset, int whence, int* internal_error);
    ssize_t (*input_read)(void *context, rust_input_handle_t handle, char *data, size_t len);
    int (*input_getc)(void *context, rust_input_handle_t handle);
    int (*input_ungetc)(void *context, rust_input_handle_t handle, int ch);
    int (*input_close)(void *context, rust_input_handle_t handle);
} tt_bridge_api_t;


BEGIN_EXTERN_C

/* These functions are not meant to be used in the C/C++ code. They define the
 * API that we expose to the Rust side of things. */

const char *tt_get_error_message(void);
int tex_simple_main(tt_bridge_api_t *api, char *dump_name, char *input_file_name, time_t build_date);
int dvipdfmx_simple_main(tt_bridge_api_t *api, char *dviname, char *pdfname, bool compress, bool deterministic_tags, time_t build_date);
int bibtex_simple_main(tt_bridge_api_t *api, char *aux_file_name);

/* The internal, C/C++ interface: */

NORETURN PRINTF_FUNC(1,2) int _tt_abort(const char *format, ...);

/* Global symbols that route through the global API variable. Hopefully we
 * will one day eliminate all of the global state and get rid of all of
 * these. */

PRINTF_FUNC(1,2) void ttstub_issue_warning(const char *format, ...);
PRINTF_FUNC(1,2) void ttstub_issue_error(const char *format, ...);
PRINTF_FUNC(2,3) int ttstub_fprintf(rust_output_handle_t handle, const char *format, ...);

int ttstub_get_file_md5 (char const *path, char *digest);
int ttstub_get_data_md5 (char const *data, size_t len, char *digest);

rust_output_handle_t ttstub_output_open (char const *path, int is_gz);
rust_output_handle_t ttstub_output_open_stdout (void);
int ttstub_output_putc (rust_output_handle_t handle, int c);
size_t ttstub_output_write (rust_output_handle_t handle, const char *data, size_t len);
int ttstub_output_flush (rust_output_handle_t handle);
int ttstub_output_close (rust_output_handle_t handle);

rust_input_handle_t ttstub_input_open (char const *path, tt_input_format_type format, int is_gz);
rust_input_handle_t ttstub_input_open_primary (void);
size_t ttstub_input_get_size (rust_input_handle_t handle);
size_t ttstub_input_seek (rust_input_handle_t handle, ssize_t offset, int whence);
ssize_t ttstub_input_read (rust_input_handle_t handle, char *data, size_t len);
int ttstub_input_getc (rust_input_handle_t handle);
int ttstub_input_ungetc (rust_input_handle_t handle, int ch);
int ttstub_input_close (rust_input_handle_t handle);

END_EXTERN_C

#endif /* not TECTONIC_CORE_BRIDGE_H */
