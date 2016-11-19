/* Collected kpathsea files in the tidied workalike version.

   Copyright 1993, 1994, 2008, 2009 Karl Berry.
   Copyright 1999, 2005 Olaf Weber.

   This library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   This library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this library; if not, see <http://www.gnu.org/licenses/>.  */

#ifndef TIDY_KPATHSEA_PUBLIC_H
#define TIDY_KPATHSEA_PUBLIC_H

#include <tidy_kpathutil/public.h>

typedef void (*p_record_input) (const_string);
typedef void (*p_record_output) (const_string);

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

typedef enum
{
  kpse_src_implicit,   /* C initialization to zero */
  kpse_src_compile,    /* configure/compile-time default */
  kpse_src_texmf_cnf,  /* texmf.cnf, the kpathsea config file */
  kpse_src_client_cnf, /* application config file, e.g., config.ps */
  kpse_src_env,        /* environment variable */
  kpse_src_x,          /* X Window System resource */
  kpse_src_cmdline     /* command-line option */
} kpse_src_type;

#ifdef __cplusplus
extern "C" {
#endif

/* global instance opaque struct plus custom accessors */

extern const char *kpathsea_version_string;

#define kpse_program_name (kpse_pkgw_get_definst_program_name ())
#define kpse_invocation_name (kpse_pkgw_get_definst_invocation_name ())

/* pkgw_collected.c */

extern string kpse_pkgw_get_definst_program_name (void);
extern string kpse_pkgw_get_definst_invocation_name (void);
extern void kpse_pkgw_set_definst_record_input (p_record_input val);
extern void kpse_pkgw_set_definst_record_output (p_record_output val);
extern void kpse_pkgw_set_definst_make_tex_discard_errors (boolean val);
extern string kpse_readable_file (string name);
extern string kpse_var_value (const_string var);

/* progname.c */

extern void kpse_set_program_name (const_string argv0, const_string progname);

/* tex-file.c */

extern void kpse_set_program_enabled (kpse_file_format_type fmt, boolean value, kpse_src_type level);
extern void kpse_maketex_option (const_string fmtname,  boolean value);
extern string kpse_find_file (const_string name, kpse_file_format_type format, boolean must_exist);
extern boolean kpse_in_name_ok (const_string fname);
extern boolean kpse_out_name_ok (const_string fname);
extern void kpse_reset_program_name (const_string progname);
extern FILE *kpse_open_file (const_string name, kpse_file_format_type format);

#define kpse_find_tex(name) kpse_find_file (name, kpse_tex_format, true)

/* renames needed for web2c stuff */

#define kpseinnameok kpse_in_name_ok
#define kpseoutnameok kpse_out_name_ok
#define kpsetexformat kpse_tex_format

#ifdef __cplusplus
}
#endif

#endif /* not TIDY_KPATHSEA_PUBLIC_H */
