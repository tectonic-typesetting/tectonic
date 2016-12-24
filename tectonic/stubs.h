/* tectonic/stubs.h: declarations of Rust functions to be called from the Tectonic C code
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_STUBS_H
#define TECTONIC_STUBS_H

#include <tectonic/tectonic.h>

#include <stddef.h> /* size_t */

/* OK maybe this isn't the best place to have this, but here we are. */

typedef enum
{
  kpse_gf_format,
  kpse_pk_format,
  kpse_any_glyph_format,
  kpse_tfm_format,
  kpse_afm_format,
  kpse_base_format,
  kpse_bib_format,
  kpse_bst_format,
  kpse_cnf_format,
  kpse_db_format,
  kpse_fmt_format,
  kpse_fontmap_format,
  kpse_mem_format,
  kpse_mf_format,
  kpse_mfpool_format,
  kpse_mft_format,
  kpse_mp_format,
  kpse_mppool_format,
  kpse_mpsupport_format,
  kpse_ocp_format,
  kpse_ofm_format,
  kpse_opl_format,
  kpse_otp_format,
  kpse_ovf_format,
  kpse_ovp_format,
  kpse_pict_format,
  kpse_tex_format,
  kpse_texdoc_format,
  kpse_texpool_format,
  kpse_texsource_format,
  kpse_tex_ps_header_format,
  kpse_troff_font_format,
  kpse_type1_format,
  kpse_vf_format,
  kpse_dvips_config_format,
  kpse_ist_format,
  kpse_truetype_format,
  kpse_type42_format,
  kpse_web2c_format,
  kpse_program_text_format,
  kpse_program_binary_format,
  kpse_miscfonts_format,
  kpse_web_format,
  kpse_cweb_format,
  kpse_enc_format,
  kpse_cmap_format,
  kpse_sfd_format,
  kpse_opentype_format,
  kpse_pdftex_config_format,
  kpse_lig_format,
  kpse_texmfscripts_format,
  kpse_lua_format,
  kpse_fea_format,
  kpse_cid_format,
  kpse_mlbib_format,
  kpse_mlbst_format,
  kpse_clua_format,
  kpse_ris_format,
  kpse_bltxml_format,
  kpse_last_format /* one past last index */
} kpse_file_format_type;

typedef void *rust_output_handle_t;
typedef void *rust_input_handle_t;

BEGIN_EXTERN_C

extern char *kpse_find_file (char const *name, kpse_file_format_type format, int must_exist);
extern int kpsezip_get_readable_fd (char const *name, kpse_file_format_type format, int must_exist);

extern int ttstub_get_file_md5 (char const *path, unsigned char *digest);
extern int ttstub_get_data_md5 (unsigned char const *data, size_t len, unsigned char *digest);

extern rust_output_handle_t ttstub_output_open (char const *path, int is_gz);
extern rust_output_handle_t ttstub_output_open_stdout (void);
extern int ttstub_output_putc (rust_output_handle_t handle, int c);
extern size_t ttstub_output_write (rust_output_handle_t handle, unsigned char *data, size_t len);
extern int ttstub_output_flush (rust_output_handle_t handle);
extern int ttstub_output_close (rust_output_handle_t handle);

extern rust_input_handle_t ttstub_input_open (char const *path, kpse_file_format_type format, int is_gz);
extern int ttstub_input_getc (rust_input_handle_t handle);
extern int ttstub_input_is_eof (rust_input_handle_t handle);
extern int ttstub_input_close (rust_input_handle_t handle);

END_EXTERN_C

#endif /* not TECTONIC_STUBS_H */
