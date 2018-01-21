/* tectonic/core-bridge.h: declarations of C/C++ => Rust bridge API
   Copyright 2016-2017 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_CORE_BRIDGE_H
#define TECTONIC_CORE_BRIDGE_H

#include "tectonic.h"

#include <stddef.h> /* size_t */
#include <sys/types.h> /* ssize_t */

/* OK maybe this isn't the best place to have this, but here we are. */

typedef enum
{
    /* The values here are in order, but because we need to map the constants
     * in the Rust code, it's convenient to make them explicit here. */

    kpse_gf_format = 0,
    kpse_pk_format = 1,
    kpse_any_glyph_format = 2,
    kpse_tfm_format = 3,
    kpse_afm_format = 4,
    kpse_base_format = 5,
    kpse_bib_format = 6,
    kpse_bst_format = 7,
    kpse_cnf_format = 8,
    kpse_db_format = 9,
    kpse_fmt_format = 10,
    kpse_fontmap_format = 11,
    kpse_mem_format = 12,
    kpse_mf_format = 13,
    kpse_mfpool_format = 14,
    kpse_mft_format = 15,
    kpse_mp_format = 16,
    kpse_mppool_format = 17,
    kpse_mpsupport_format = 18,
    kpse_ocp_format = 19,
    kpse_ofm_format = 20,
    kpse_opl_format = 21,
    kpse_otp_format = 22,
    kpse_ovf_format = 23,
    kpse_ovp_format = 24,
    kpse_pict_format = 25,
    kpse_tex_format = 26,
    kpse_texdoc_format = 27,
    kpse_texpool_format = 28,
    kpse_texsource_format = 29,
    kpse_tex_ps_header_format = 30,
    kpse_troff_font_format = 31,
    kpse_type1_format = 32,
    kpse_vf_format = 33,
    kpse_dvips_config_format = 34,
    kpse_ist_format = 35,
    kpse_truetype_format = 36,
    kpse_type42_format = 37,
    kpse_web2c_format = 38,
    kpse_program_text_format = 39,
    kpse_program_binary_format = 40,
    kpse_miscfonts_format = 41,
    kpse_web_format = 42,
    kpse_cweb_format = 43,
    kpse_enc_format = 44,
    kpse_cmap_format = 45,
    kpse_sfd_format = 46,
    kpse_opentype_format = 47,
    kpse_pdftex_config_format = 48,
    kpse_lig_format = 49,
    kpse_texmfscripts_format = 50,
    kpse_lua_format = 51,
    kpse_fea_format = 52,
    kpse_cid_format = 53,
    kpse_mlbib_format = 54,
    kpse_mlbst_format = 55,
    kpse_clua_format = 56,
    kpse_ris_format = 57,
    kpse_bltxml_format = 58,
    kpse_tectonic_primary_format = 59, /* hack to get the primary input */
    kpse_last_format = 60 /* one past last index */
} kpse_file_format_type;

typedef void *rust_output_handle_t;
typedef void *rust_input_handle_t;


/* Bridge API. Keep synchronized with src/engines/mod.rs. */

typedef struct tt_bridge_api_t {
    void *context;

    char *(*kpse_find_file)(void *context, char const *name, kpse_file_format_type format, int must_exist);

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

    rust_input_handle_t (*input_open)(void *context, char const *path, kpse_file_format_type format, int is_gz);
    rust_input_handle_t (*input_open_primary)(void *context);
    size_t (*input_get_size)(void *context, rust_input_handle_t handle);
    size_t (*input_seek)(void *context, rust_input_handle_t handle, ssize_t offset, int whence);
    ssize_t (*input_read)(void *context, rust_input_handle_t handle, char *data, size_t len);
    int (*input_getc)(void *context, rust_input_handle_t handle);
    int (*input_ungetc)(void *context, rust_input_handle_t handle, int ch);
    int (*input_close)(void *context, rust_input_handle_t handle);
} tt_bridge_api_t;


BEGIN_EXTERN_C

/* These functions are not meant to be used in the C/C++ code. They define the
 * API that we expose to the Rust side of things. */

const char *tt_get_error_message(void);
int tex_simple_main(tt_bridge_api_t *api, char *dump_name, char *input_file_name);
int dvipdfmx_simple_main(tt_bridge_api_t *api, char *dviname, char *pdfname, bool compress, bool deterministic_tags);
int bibtex_simple_main(tt_bridge_api_t *api, char *aux_file_name);

/* The internal, C/C++ interface: */

NORETURN PRINTF_FUNC(1,2) int _tt_abort(const char *format, ...);

/* Global symbols that route through the global API variable. Hopefully we
 * will one day eliminate all of the global state and get rid of all of
 * these. */

char *kpse_find_file (char const *name, kpse_file_format_type format, int must_exist);

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

rust_input_handle_t ttstub_input_open (char const *path, kpse_file_format_type format, int is_gz);
rust_input_handle_t ttstub_input_open_primary (void);
size_t ttstub_input_get_size (rust_input_handle_t handle);
size_t ttstub_input_seek (rust_input_handle_t handle, ssize_t offset, int whence);
ssize_t ttstub_input_read (rust_input_handle_t handle, char *data, size_t len);
int ttstub_input_getc (rust_input_handle_t handle);
int ttstub_input_ungetc (rust_input_handle_t handle, int ch);
int ttstub_input_close (rust_input_handle_t handle);

END_EXTERN_C

#endif /* not TECTONIC_CORE_BRIDGE_H */
