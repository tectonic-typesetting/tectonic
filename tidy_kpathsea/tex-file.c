/* tex-file.c: high-level file searching by format.

   Copyright 1993, 1994, 1995, 1996, 1997, 2007, 2008, 2009, 2010, 2011
             2012, 2014 Karl Berry.
   Copyright 1998-2005 Olaf Weber.

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

#include <tidy_kpathsea/public.h>
#include <tidy_kpathsea/private.h>

#define ENVVAR(test, default) (getenv (test) ? (test) : (default))

#include <stdarg.h>

/* These are not in the structure
   because it's annoying to initialize lists in C.  */
#define GF_ENVS "GFFONTS", GLYPH_ENVS
#define PK_ENVS "PKFONTS", "TEXPKS", GLYPH_ENVS
#define GLYPH_ENVS "GLYPHFONTS", "TEXFONTS"
#define TFM_ENVS "TFMFONTS", "TEXFONTS"
#define AFM_ENVS "AFMFONTS", "TEXFONTS"
#define BASE_ENVS "MFBASES", "TEXMFINI"
#define BIB_ENVS "BIBINPUTS", "TEXBIB"
#define BST_ENVS "BSTINPUTS"
#define CNF_ENVS "TEXMFCNF"
#define DB_ENVS "TEXMFDBS"
#define FMT_ENVS "TEXFORMATS", "TEXMFINI"
#define FONTMAP_ENVS "TEXFONTMAPS", "TEXFONTS"
#define MEM_ENVS "MPMEMS", "TEXMFINI"
#define MF_ENVS "MFINPUTS"
#define MFPOOL_ENVS "MFPOOL", "TEXMFINI"
#define MFT_ENVS "MFTINPUTS"
#define MP_ENVS "MPINPUTS"
#define MPPOOL_ENVS "MPPOOL", "TEXMFINI"
#define MPSUPPORT_ENVS "MPSUPPORT"
#define OCP_ENVS "OCPINPUTS"
#define OFM_ENVS "OFMFONTS", "TEXFONTS"
#define OPL_ENVS "OPLFONTS", "TEXFONTS"
#define OTP_ENVS "OTPINPUTS"
#define OVF_ENVS "OVFFONTS", "TEXFONTS"
#define OVP_ENVS "OVPFONTS", "TEXFONTS"
#define PICT_ENVS "TEXPICTS", TEX_ENVS
#define TEX_ENVS "TEXINPUTS"
#define TEXDOC_ENVS "TEXDOCS"
#define TEXPOOL_ENVS "TEXPOOL", "TEXMFINI"
#define TEXSOURCE_ENVS "TEXSOURCES"
#define TEX_PS_HEADER_ENVS "TEXPSHEADERS", "PSHEADERS"
#define TROFF_FONT_ENVS "TRFONTS"
#define TYPE1_ENVS "T1FONTS", "T1INPUTS", "TEXFONTS", TEX_PS_HEADER_ENVS
#define VF_ENVS "VFFONTS", "TEXFONTS"
#define DVIPS_CONFIG_ENVS "TEXCONFIG"
#define IST_ENVS "TEXINDEXSTYLE", "INDEXSTYLE"
#define TRUETYPE_ENVS "TTFONTS", "TEXFONTS"
#define TYPE42_ENVS "T42FONTS", "TEXFONTS"
#define WEB2C_ENVS "WEB2C"
#define MISCFONTS_ENVS "MISCFONTS", "TEXFONTS"
#define WEB_ENVS "WEBINPUTS"
#define CWEB_ENVS "CWEBINPUTS"
#define ENC_ENVS "ENCFONTS", "TEXFONTS"
#define CMAP_ENVS "CMAPFONTS", "TEXFONTS"
#define SFD_ENVS "SFDFONTS", "TEXFONTS"
#define OPENTYPE_ENVS "OPENTYPEFONTS", "TEXFONTS"
#define PDFTEXCONFIG_ENVS "PDFTEXCONFIG"
#define LIG_ENVS "LIGFONTS", "TEXFONTS"
#define TEXMFSCRIPTS_ENVS "TEXMFSCRIPTS"
#define LUA_ENVS "LUAINPUTS"
#define FONTFEATURES_ENVS "FONTFEATURES"
#define FONTCIDMAPS_ENVS "FONTCIDMAPS"
#define MLBIB_ENVS "MLBIBINPUTS", BIB_ENVS
#define MLBST_ENVS "MLBSTINPUTS", BST_ENVS
#define CLUA_ENVS "CLUAINPUTS"
#define RIS_ENVS "RISINPUTS"
#define BLTXML_ENVS "BLTXMLINPUTS"

/* The compiled-in default list, DEFAULT_FONT_SIZES, is intended to be
   set from the command line (presumably via the Makefile).  */

#ifndef DEFAULT_FONT_SIZES
#define DEFAULT_FONT_SIZES ""
#endif

void
kpathsea_init_fallback_resolutions (kpathsea kpse, string envvar)
{
  string size;
  const_string size_var = ENVVAR (envvar, "TEXSIZES");
  string size_str = getenv (size_var);
  unsigned *last_resort_sizes = NULL;
  unsigned size_count = 0;
  const_string default_sizes = kpse->fallback_resolutions_string
                         ? kpse->fallback_resolutions_string
                         : DEFAULT_FONT_SIZES;
  string size_list = kpathsea_expand_default (kpse, size_str, default_sizes);

  /* Initialize the list of last-resort sizes.  */
  for (size = kpathsea_path_element (kpse, size_list); size != NULL;
       size = kpathsea_path_element (kpse, NULL))
    {
      unsigned s;
      if (! *size) /* Skip empty elements.  */
        continue;

      s = atoi (size);
      if (size_count && s < last_resort_sizes[size_count - 1]) {
    WARNING1 ("kpathsea: last resort size %s not in ascending order; ignored",
          size);
      } else {
        size_count++;
        XRETALLOC (last_resort_sizes, size_count, unsigned);
        last_resort_sizes[size_count - 1] = atoi (size);
      }
    }

  /* Add a zero to mark the end of the list.  */
  size_count++;
  XRETALLOC (last_resort_sizes, size_count, unsigned);
  last_resort_sizes[size_count - 1] = 0;

  free (size_list);

  kpse->fallback_resolutions = last_resort_sizes;
}

#if defined (KPSE_COMPAT_API)
void
kpse_init_fallback_resolutions ( string envvar)
{
  kpathsea_init_fallback_resolutions (kpse_def,  envvar);
}
#endif


/* We should be able to set the program arguments in the same way.  Not
   to mention the path values themselves.  */

void
kpathsea_set_program_enabled (kpathsea kpse, kpse_file_format_type fmt,
                              boolean value, kpse_src_type level)
{
  kpse_format_info_type *f = &(kpse->format_info[fmt]);
  if (level >= f->program_enable_level) {
    f->program_enabled_p = value;
    f->program_enable_level = level;
  }
}


#if defined (KPSE_COMPAT_API)
void
kpse_set_program_enabled (kpse_file_format_type fmt,
                          boolean value, kpse_src_type level)
{
  kpathsea_set_program_enabled (kpse_def, fmt, value, level);
}

#endif


/* Web2c and kpsewhich have command-line options to set this stuff.  May
   as well have a common place.  */

void
kpathsea_maketex_option (kpathsea kpse, const_string fmtname, boolean value)
{
  kpse_file_format_type fmt = kpse_last_format;

  /* Trying to match up with the suffix lists unfortunately doesn't work
     well, since that would require initializing the formats.  */
  if (FILESTRCASEEQ (fmtname, "pk")) {
    fmt = kpse_pk_format;
  } else if (FILESTRCASEEQ (fmtname, "mf")) {
    fmt = kpse_mf_format;
  } else if (FILESTRCASEEQ (fmtname, "tex")) {
    fmt = kpse_tex_format;
  } else if (FILESTRCASEEQ (fmtname, "tfm")) {
    fmt = kpse_tfm_format;
  } else if (FILESTRCASEEQ (fmtname, "fmt")) {
    fmt = kpse_fmt_format;
  } else if (FILESTRCASEEQ (fmtname, "ofm")) {
    fmt = kpse_ofm_format;
  } else if (FILESTRCASEEQ (fmtname, "ocp")) {
    fmt = kpse_ocp_format;
  } else {
    fprintf (stderr, "\nkpathsea: Unknown mktex format: %s\n", fmtname);
  }

  if (fmt != kpse_last_format) {
    kpathsea_set_program_enabled (kpse, fmt, value, kpse_src_cmdline);
  }
}

#if defined (KPSE_COMPAT_API)
void
kpse_maketex_option (const_string fmtname,  boolean value)
{
  kpathsea_maketex_option (kpse_def, fmtname,  value);
}
#endif


/* Macro subroutine for `init_path'.  EXPAND_DEFAULT calls
   kpse_expand_default on try_path and the present info->path.  */
#define EXPAND_DEFAULT(try_path, source_string)                 \
  if (try_path) {                                               \
    info->raw_path = try_path;                                  \
    tmp = info->path;                                           \
    info->path = kpathsea_expand_default (kpse, try_path, info->path);  \
    free (tmp);                                                 \
    info->path_source = source_string;                          \
  }

/* Find the final search path to use for the format entry INFO, given
   the compile-time default (DEFAULT_PATH), and the environment
   variables to check (the remaining arguments, terminated with NULL).
   We set the `path' and `path_source' members of INFO.  The
   `client_path' member must already be set upon entry.  */

static void
init_path (kpathsea kpse, kpse_format_info_type *info,
           const_string default_path, ...)
{
  string env_name;
  string env_value = NULL;
  string var = NULL;
  string tmp;
  va_list ap;

  info->default_path = default_path;

  va_start (ap, default_path);
  /* First envvar that's set to a nonempty value will exit the loop.  If
     none are set, we want the first cnf entry that matches.  Find the
     cnf value simultaneously with the envvar value, to avoid having to
     go through the envvar list twice,
     that would mean having to create a str_list and use it twice.  */
  while ((env_name = va_arg (ap, string)) != NULL) {
    /* Since sh doesn't like envvar names with `.', check PATH_prog
       as well as PATH.prog.  */
    if (!var) { /* Try PATH.prog. */
      string evar = concat3 (env_name, ".", kpse->program_name);
      env_value = getenv (evar);
      if (env_value && *env_value) {
        var = evar;
      } else { /* Try PATH_prog. */
        free (evar);
        evar = concat3 (env_name, "_", kpse->program_name);
        env_value = getenv (evar);
        if (env_value && *env_value) {
          var = evar;
        } else { /* Try simply PATH.  */
          free (evar);
          env_value = getenv (env_name);
          if (env_value && *env_value) {
            var = env_name;
          }
        }
      }
    }

    /* If we are initializing the cnf path, don't try to get any
       values from the cnf files; that's infinite loop time.  */
    if (!info->cnf_path && info != &(kpse->format_info[kpse_cnf_format]))
      info->cnf_path = kpathsea_cnf_get (kpse, env_name);

    if (var && info->cnf_path)
      break;
  }
  va_end (ap);

  /* Expand any extra :'s.  For each level, we replace an extra : with
     the path at the next lower level.  For example, an extra : in a
     user-set envvar should be replaced with the path from the cnf file.
     things are complicated because none of the levels above the very
     bottom are guaranteed to exist.  */

  /* Assume we can reliably start with the compile-time default.  */
  info->raw_path = info->default_path;
  info->path = xstrdup (info->raw_path);
  info->path_source = "compile-time paths.h";

  EXPAND_DEFAULT (info->cnf_path, "texmf.cnf");
  EXPAND_DEFAULT (info->client_path, "program config file");

  if (var) {
    /* Translate `;' in the envvar into `:' if that's our ENV_SEP. */
    if (IS_ENV_SEP (':')) {
      string loc;
      env_value = xstrdup (env_value);  /* Freed below. */
      for (loc = env_value; *loc; loc++) {
        if (*loc == ';')
          *loc = ':';
      }
    }
    EXPAND_DEFAULT (env_value, concat (var, " environment variable"));
    /* Do not free the copied env_value, because EXPAND_DEFAULT set
       raw_path to point to it.  If it gets overwritten again, tough.  */
  }

  EXPAND_DEFAULT (info->override_path, "application override variable");
  tmp = info->path;
  info->path = kpathsea_brace_expand (kpse, info->path);
  free (tmp);
}


/* Some file types have more than one suffix, and sometimes it is
   convenient to modify the list of searched suffixes.  */

static void
kpathsea_set_suffixes_va_list (kpathsea kpse, kpse_file_format_type format,
                               boolean alternate, va_list ap)
{
  const_string **list;
  const_string s;
  int count = 0;

  if (alternate) {
    list = &(kpse->format_info[format].alt_suffix);
  } else {
    list = &(kpse->format_info[format].suffix);
  }

  while ((s = va_arg (ap, string)) != NULL) {
    count++;
    /* This is essentially
    XRETALLOC (*list, count + 1, const_string);
       except that MSVC warns without the cast to `void *'.  */
    *list = (const_string *) xrealloc ((void *) *list,
                                       (count + 1) * sizeof(const_string));
    (*list)[count - 1] = s;
  }
  (*list)[count] = NULL;
}

void
kpathsea_set_suffixes (kpathsea kpse, kpse_file_format_type format,
  boolean alternate, ...)
{
  va_list ap;
  va_start (ap, alternate);
  kpathsea_set_suffixes_va_list (kpse, format, alternate, ap);
  va_end (ap);
}


#if defined (KPSE_COMPAT_API)
void
kpse_set_suffixes (kpse_file_format_type format,
                   boolean alternate, ...)
{
  va_list ap;
  va_start (ap, alternate);
  kpathsea_set_suffixes_va_list (kpse_def, format, alternate, ap);
  va_end (ap);
}
#endif


/* The path spec we are defining, one element of the global array.  */
#define FMT_INFO (kpse->format_info[format])
/* Call kpse_set_add_suffixes.  */
#define SUFFIXES(args) kpathsea_set_suffixes(kpse, format, false, args, NULL)
#define ALT_SUFFIXES(args) kpathsea_set_suffixes(kpse, format, true, args, NULL)

/* Call `init_path', including appending the trailing NULL to the envvar
   list. Also initialize the fields not needed in setting the path.  */
#define INIT_FORMAT(text, default_path, envs) \
  FMT_INFO.type = text; \
  init_path (kpse, &FMT_INFO, default_path, envs, NULL);   \
  envvar_list = concatn_with_spaces (envs, NULL);


/* A few file types allow for runtime generation by an external program.
   kpse_init_prog may have already initialized it (the `program'
   member).  Here we allow people to turn it off or on in the config
   file, by setting the variable whose name is the uppercasified program
   name to 0 or 1.  */

static void
init_maketex (kpathsea kpse, kpse_file_format_type fmt,
              const_string dflt_prog, ...)
{
  kpse_format_info_type *f = &(kpse->format_info[fmt]);
  const_string prog = f->program ? f->program : dflt_prog; /* mktexpk */
  string PROG = uppercasify (prog);                  /* MKTEXPK */
  string progval = kpathsea_var_value (kpse, PROG);  /* $ENV/cnf{"MKTEXPK"} */
  const_string arg;
  va_list ap;

  /* Doesn't hurt to always set this info.  */
  f->program = prog;

  /* Set up the argument vector. */
  f->argc = 0;
  f->argv = XTALLOC (2, const_string);
  f->argv[f->argc++] = dflt_prog;
  va_start (ap, dflt_prog);
  while ((arg = va_arg (ap, string)) != NULL) {
    f->argc++;
    f->argv = (const_string *) xrealloc ((void *) f->argv,
                                         (f->argc + 1) * sizeof(const_string));
    f->argv[f->argc - 1] = arg;
  }
  va_end (ap);
  f->argv[f->argc] = NULL;

  if (progval && *progval) {
    /* This might actually be from an environment variable value, but in
       that case, we'll have previously set it from kpse_init_prog.  */
    kpathsea_set_program_enabled (kpse, fmt, *progval == '1',
                                  kpse_src_client_cnf);
  }

  free (PROG);
}

/* We need this twice, so ... */
#define MKTEXPK_ARGS \
  "--mfmode","$MAKETEX_MODE",\
  "--bdpi","$MAKETEX_BASE_DPI",\
  "--mag","$MAKETEX_MAG",\
  "--dpi","$KPATHSEA_DPI",\
  NULL

static string
remove_dbonly (const_string path)
{
  string ret = XTALLOC(strlen (path) + 1, char), q=ret;
  const_string p=path;
  boolean new_elt=true;

  while (*p) {
    if (new_elt && *p && *p == '!' && *(p+1) == '!')
      p += 2;
    else {
      new_elt = (*p == ENV_SEP);
      *q++ = *p++;
    }
  }
  *q = '\0';
  return(ret);
}

/* Same as concatn but puts a space between each element.  All this just
   for nice debugging output.  But it's useful.  */

static string
concatn_with_spaces (const_string str1, ...)
{
  string arg;
  string ret;
  va_list ap;

  if (!str1)
    return NULL;

  ret = xstrdup (str1);

  va_start (ap, str1);
  while ((arg = va_arg (ap, string)) != NULL)
    {
      string temp = concat3 (ret, " ", arg);
      free (ret);
      ret = temp;
    }
  va_end (ap);

  return ret;
}


/* Initialize everything for FORMAT.  Return its search path.  */

const_string
kpathsea_init_format (kpathsea kpse, kpse_file_format_type format)
{
  /* If we get called twice, don't redo all the work.  */
  if (! FMT_INFO.path) {
    /* Don't care about the list of variable names.  */
    (void) kpathsea_init_format_return_varlist (kpse, format);
  }
  
  return FMT_INFO.path;
}

/* Initialize everything for FORMAT.  Return the list of
   environment/config variable names considered, which is not otherwise
   saved.  Although in principle we could add that list to our format
   struct, it seems a waste, since this is only used by kpsewhich --help.  */

const_string
kpathsea_init_format_return_varlist(kpathsea kpse,kpse_file_format_type format)
{
  string envvar_list;  /* only for debug output, set in INIT_FORMAT */

  switch (format)
    { /* We might be able to avoid repeating `gf' or whatever so many
         times with token pasting, but it doesn't seem worth it.  */
    case kpse_gf_format:
      INIT_FORMAT ("gf", DEFAULT_GFFONTS, GF_ENVS);
      SUFFIXES ("gf");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_pk_format:
      init_maketex (kpse, format, "mktexpk", MKTEXPK_ARGS);
      INIT_FORMAT ("pk", DEFAULT_PKFONTS, PK_ENVS);
      SUFFIXES ("pk");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_any_glyph_format:
      init_maketex (kpse, format, "mktexpk", MKTEXPK_ARGS);
      INIT_FORMAT ("bitmap font", DEFAULT_GLYPHFONTS, GLYPH_ENVS);
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_tfm_format:
      /* Must come before kpse_ofm_format. */
      init_maketex (kpse, format, "mktextfm", NULL);
      INIT_FORMAT ("tfm", DEFAULT_TFMFONTS, TFM_ENVS);
      SUFFIXES (".tfm");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_afm_format:
      INIT_FORMAT ("afm", DEFAULT_AFMFONTS, AFM_ENVS);
      SUFFIXES (".afm");
      break;
    case kpse_base_format:
      init_maketex (kpse, format, "mktexfmt", NULL);
      INIT_FORMAT ("base", DEFAULT_MFBASES, BASE_ENVS);
      SUFFIXES (".base");
      FMT_INFO.binmode = true;
      break;
    case kpse_bib_format:
      INIT_FORMAT ("bib", DEFAULT_BIBINPUTS, BIB_ENVS);
      SUFFIXES (".bib");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_bst_format:
      INIT_FORMAT ("bst", DEFAULT_BSTINPUTS, BST_ENVS);
      SUFFIXES (".bst");
      break;
    case kpse_cnf_format:
      INIT_FORMAT ("cnf", DEFAULT_TEXMFCNF, CNF_ENVS);
      SUFFIXES (".cnf");
      break;
    case kpse_db_format:
      INIT_FORMAT ("ls-R", DEFAULT_TEXMFDBS, DB_ENVS);
#define LSR_SUFFIXES "ls-R", "ls-r"
      SUFFIXES (LSR_SUFFIXES);
      FMT_INFO.path = remove_dbonly (FMT_INFO.path);
      break;
    case kpse_fmt_format:
      init_maketex (kpse, format, "mktexfmt", NULL);
      INIT_FORMAT ("fmt", DEFAULT_TEXFORMATS, FMT_ENVS);
      SUFFIXES (".fmt");
      FMT_INFO.binmode = true;
      break;
    case kpse_fontmap_format:
      INIT_FORMAT ("map", DEFAULT_TEXFONTMAPS, FONTMAP_ENVS);
      SUFFIXES (".map");
      break;
    case kpse_mem_format:
      init_maketex (kpse, format, "mktexfmt", NULL);
      INIT_FORMAT ("mem", DEFAULT_MPMEMS, MEM_ENVS);
      SUFFIXES (".mem");
      FMT_INFO.binmode = true;
      break;
    case kpse_mf_format:
      init_maketex (kpse, format, "mktexmf", NULL);
      INIT_FORMAT ("mf", DEFAULT_MFINPUTS, MF_ENVS);
      SUFFIXES (".mf");
      break;
    case kpse_mft_format:
      INIT_FORMAT ("mft", DEFAULT_MFTINPUTS, MFT_ENVS);
      SUFFIXES (".mft");
      break;
    case kpse_mfpool_format:
      INIT_FORMAT ("mfpool", DEFAULT_MFPOOL, MFPOOL_ENVS);
      SUFFIXES (".pool");
      break;
    case kpse_mp_format:
      INIT_FORMAT ("mp", DEFAULT_MPINPUTS, MP_ENVS);
      SUFFIXES (".mp");
      break;
    case kpse_mppool_format:
      INIT_FORMAT ("mppool", DEFAULT_MPPOOL, MPPOOL_ENVS);
      SUFFIXES (".pool");
      break;
    case kpse_mpsupport_format:
      INIT_FORMAT ("MetaPost support", DEFAULT_MPSUPPORT, MPSUPPORT_ENVS);
      break;
    case kpse_ocp_format:
      init_maketex (kpse, format, "mkocp", NULL);
      INIT_FORMAT ("ocp", DEFAULT_OCPINPUTS, OCP_ENVS);
      SUFFIXES (".ocp");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_ofm_format:
      init_maketex (kpse, format, "mkofm", NULL);
      INIT_FORMAT ("ofm", DEFAULT_OFMFONTS, OFM_ENVS);
#define OFM_SUFFIXES ".ofm", ".tfm"
      SUFFIXES (OFM_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_opl_format:
      INIT_FORMAT ("opl", DEFAULT_OPLFONTS, OPL_ENVS);
      SUFFIXES (".opl");
      ALT_SUFFIXES (".pl");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_otp_format:
      INIT_FORMAT ("otp", DEFAULT_OTPINPUTS, OTP_ENVS);
      SUFFIXES (".otp");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_ovf_format:
      INIT_FORMAT ("ovf", DEFAULT_OVFFONTS, OVF_ENVS);
#define OVF_SUFFIXES ".ovf", ".vf"
      SUFFIXES (OVF_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_ovp_format:
      INIT_FORMAT ("ovp", DEFAULT_OVPFONTS, OVP_ENVS);
      SUFFIXES (".ovp");
      ALT_SUFFIXES (".vpl");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_pict_format:
      INIT_FORMAT ("graphic/figure", DEFAULT_TEXINPUTS, PICT_ENVS);
#define ALT_PICT_SUFFIXES ".eps", ".epsi"
      ALT_SUFFIXES (ALT_PICT_SUFFIXES);
      FMT_INFO.binmode = true;
      break;
    case kpse_tex_format:
      init_maketex (kpse, format, "mktextex", NULL);
      INIT_FORMAT ("tex", DEFAULT_TEXINPUTS, TEX_ENVS);
      SUFFIXES (".tex");
      /* TeX files can have any obscure suffix in the world (or none at
         all).  Only check for the most common ones.  */
#define ALT_TEX_SUFFIXES ".sty",".cls",".fd",".aux",".bbl",".def",".clo",".ldf"
      ALT_SUFFIXES (ALT_TEX_SUFFIXES);
      break;
    case kpse_tex_ps_header_format:
      INIT_FORMAT ("PostScript header", DEFAULT_TEXPSHEADERS,
                   TEX_PS_HEADER_ENVS);
/* Unfortunately, at one time dvips used this format for type1 fonts.  */
#define ALT_TEXPSHEADER_SUFFIXES ".pro"
      ALT_SUFFIXES (ALT_TEXPSHEADER_SUFFIXES);
      FMT_INFO.binmode = true;
      break;
    case kpse_texdoc_format:
      INIT_FORMAT ("TeX system documentation", DEFAULT_TEXDOCS, TEXDOC_ENVS);
      break;
    case kpse_texpool_format:
      INIT_FORMAT ("texpool", DEFAULT_TEXPOOL, TEXPOOL_ENVS);
      SUFFIXES (".pool");
      break;
    case kpse_texsource_format:
      INIT_FORMAT ("TeX system sources", DEFAULT_TEXSOURCES, TEXSOURCE_ENVS);
#define ALT_SOURCES_SUFFIXES ".dtx", ".ins"
      ALT_SUFFIXES (ALT_SOURCES_SUFFIXES);
      break;
    case kpse_troff_font_format:
      INIT_FORMAT ("Troff fonts", DEFAULT_TRFONTS, TROFF_FONT_ENVS);
      FMT_INFO.binmode = true;
      break;
    case kpse_type1_format:
      INIT_FORMAT ("type1 fonts", DEFAULT_T1FONTS, TYPE1_ENVS);
#define TYPE1_SUFFIXES ".pfa", ".pfb"
      SUFFIXES (TYPE1_SUFFIXES);
      FMT_INFO.binmode = true;
      break;
    case kpse_vf_format:
      INIT_FORMAT ("vf", DEFAULT_VFFONTS, VF_ENVS);
      SUFFIXES (".vf");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_dvips_config_format:
      INIT_FORMAT ("dvips config", DEFAULT_TEXCONFIG, DVIPS_CONFIG_ENVS);
      break;
    case kpse_ist_format:
      INIT_FORMAT ("ist", DEFAULT_INDEXSTYLE, IST_ENVS);
      SUFFIXES (".ist");
      break;
    case kpse_truetype_format:
      INIT_FORMAT ("truetype fonts", DEFAULT_TTFONTS, TRUETYPE_ENVS);
#define TRUETYPE_SUFFIXES ".ttf", ".ttc", ".TTF", ".TTC", ".dfont"
      SUFFIXES (TRUETYPE_SUFFIXES);
      FMT_INFO.suffix_search_only = false;
      FMT_INFO.binmode = true;
      break;
    case kpse_type42_format:
      INIT_FORMAT ("type42 fonts", DEFAULT_T42FONTS, TYPE42_ENVS);
#define TYPE42_SUFFIXES ".t42", ".T42"
      SUFFIXES (TYPE42_SUFFIXES);
      FMT_INFO.binmode = true;
      break;
    case kpse_web2c_format:
      INIT_FORMAT ("web2c files", DEFAULT_WEB2C, WEB2C_ENVS);
      break;
    case kpse_program_text_format:
      INIT_FORMAT ("other text files",
                   concatn (".", ENV_SEP_STRING, "$TEXMF/",
                            kpse->program_name, "//", NULL),
                   concat (uppercasify (kpse->program_name), "INPUTS"));
      break;
    case kpse_program_binary_format:
      INIT_FORMAT ("other binary files",
                   concatn (".", ENV_SEP_STRING, "$TEXMF/",
                            kpse->program_name, "//", NULL),
                   concat (uppercasify (kpse->program_name), "INPUTS"));
      FMT_INFO.binmode = true;
      break;
    case kpse_miscfonts_format:
      INIT_FORMAT ("misc fonts", DEFAULT_MISCFONTS, MISCFONTS_ENVS);
      FMT_INFO.binmode = true;
      break;
    case kpse_web_format:
      INIT_FORMAT ("web", DEFAULT_WEBINPUTS, WEB_ENVS);
      SUFFIXES (".web");
      ALT_SUFFIXES (".ch");
      break;
    case kpse_cweb_format:
      INIT_FORMAT ("cweb", DEFAULT_CWEBINPUTS, CWEB_ENVS);
#define CWEB_SUFFIXES ".w", ".web"
      SUFFIXES (CWEB_SUFFIXES);
      ALT_SUFFIXES (".ch");
      break;
    case kpse_enc_format:
      INIT_FORMAT ("enc files", DEFAULT_ENCFONTS, ENC_ENVS);
      SUFFIXES (".enc");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_cmap_format:
      INIT_FORMAT ("cmap files", DEFAULT_CMAPFONTS, CMAP_ENVS);
      break;
    case kpse_sfd_format:
      INIT_FORMAT ("subfont definition files", DEFAULT_SFDFONTS, SFD_ENVS);
      SUFFIXES (".sfd");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_opentype_format:
      INIT_FORMAT ("opentype fonts", DEFAULT_OPENTYPEFONTS, OPENTYPE_ENVS);
      SUFFIXES (".otf");
      FMT_INFO.suffix_search_only = true;
      FMT_INFO.binmode = true;
      break;
    case kpse_pdftex_config_format:
      INIT_FORMAT ("pdftex config", DEFAULT_PDFTEXCONFIG, PDFTEXCONFIG_ENVS);
      break;
    case kpse_lig_format:
      INIT_FORMAT ("lig files", DEFAULT_LIGFONTS, LIG_ENVS);
      SUFFIXES (".lig");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_texmfscripts_format:
      INIT_FORMAT ("texmfscripts", DEFAULT_TEXMFSCRIPTS, TEXMFSCRIPTS_ENVS);
      break;
    case kpse_lua_format:
      INIT_FORMAT ("lua", DEFAULT_LUAINPUTS, LUA_ENVS);
#define LUA_SUFFIXES \
  ".lua", ".luatex", ".luc", ".luctex", ".texlua", ".texluc", ".tlu"
      SUFFIXES (LUA_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_fea_format:
      INIT_FORMAT("font feature files", DEFAULT_FONTFEATURES, FONTFEATURES_ENVS);
      SUFFIXES (".fea");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_cid_format:
      INIT_FORMAT ("cid maps", DEFAULT_FONTCIDMAPS, FONTCIDMAPS_ENVS);
#define CID_SUFFIXES ".cid", ".cidmap"
      SUFFIXES (CID_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_mlbib_format:
      INIT_FORMAT ("mlbib", DEFAULT_MLBIBINPUTS, MLBIB_ENVS);
#define MLBIB_SUFFIXES ".mlbib", ".bib"
      SUFFIXES (MLBIB_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_mlbst_format:
      INIT_FORMAT ("mlbst", DEFAULT_MLBSTINPUTS, MLBST_ENVS);
#define MLBST_SUFFIXES ".mlbst", ".bst"
      SUFFIXES (MLBST_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_clua_format:
      INIT_FORMAT ("clua", DEFAULT_CLUAINPUTS, CLUA_ENVS);
#define CLUA_SUFFIXES ".dll", ".so"
      SUFFIXES (CLUA_SUFFIXES);
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_ris_format:
      INIT_FORMAT ("ris", DEFAULT_RISINPUTS, RIS_ENVS);
      SUFFIXES (".ris");
      FMT_INFO.suffix_search_only = true;
      break;
    case kpse_bltxml_format:
      INIT_FORMAT ("bltxml", DEFAULT_BLTXMLINPUTS, BLTXML_ENVS);
      SUFFIXES (".bltxml");
      FMT_INFO.suffix_search_only = true;
      break;
    default:
      LIB_FATAL1 ("kpse_init_format: Unknown format %d", format);
    }

#ifdef KPSE_DEBUG
#define MAYBE(member) (FMT_INFO.member ? FMT_INFO.member : "(none)")

  /* Describe the monster we've created.  */
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_PATHS))
    {
      DEBUGF2 ("Search path for %s files (from %s)\n",
              FMT_INFO.type, FMT_INFO.path_source);
      DEBUGF1 ("  = %s\n", FMT_INFO.path);
      DEBUGF1 ("  before expansion = %s\n", FMT_INFO.raw_path);
      DEBUGF1 ("  application override path = %s\n", MAYBE (override_path));
      DEBUGF1 ("  application config file path = %s\n", MAYBE (client_path));
      DEBUGF1 ("  texmf.cnf path = %s\n", MAYBE (cnf_path));
      DEBUGF1 ("  compile-time path = %s\n", MAYBE (default_path));
      DEBUGF1 ("  environment variables = %s\n", envvar_list);
      DEBUGF  ("  default suffixes =");
      if (FMT_INFO.suffix) {
        const_string *ext;
        for (ext = FMT_INFO.suffix; ext && *ext; ext++) {
          fprintf (stderr, " %s", *ext);
        }
        putc ('\n', stderr);
      } else {
        fputs (" (none)\n", stderr);
      }
      DEBUGF  ("  other suffixes =");
      if (FMT_INFO.alt_suffix) {
        const_string *alt;
        for (alt = FMT_INFO.alt_suffix; alt && *alt; alt++) {
          fprintf (stderr, " %s", *alt);
        }
        putc ('\n', stderr);
      } else {
        fputs (" (none)\n", stderr);
      }
      DEBUGF1 ("  search only with suffix = %d\n",FMT_INFO.suffix_search_only);
      DEBUGF1 ("  runtime generation program = %s\n", MAYBE (program));
      DEBUGF  ("  runtime generation command =");
      if (FMT_INFO.argv) {
        const_string *arg;
        for (arg = FMT_INFO.argv; *arg; arg++) {
          fprintf (stderr, " %s", *arg);
        }
        putc ('\n', stderr);
      } else {
          fputs(" (none)\n", stderr);
      }
      DEBUGF1 ("  program enabled = %d\n", FMT_INFO.program_enabled_p);
      DEBUGF1 ("  program enable level = %d\n", FMT_INFO.program_enable_level);
      DEBUGF1 ("  open files in binary mode = %d\n", FMT_INFO.binmode);
      DEBUGF1 ("  numeric format value = %d\n", format);
    }
#endif /* KPSE_DEBUG */

  return envvar_list;
}

#if defined (KPSE_COMPAT_API)
const_string
kpse_init_format (kpse_file_format_type format)
{
  return kpathsea_init_format (kpse_def, format);
}
#endif


/* These are subroutines called twice when finding file, to construct
   the list of names to search for.  */

/* We don't even use fontmaps any more in practice, they were for things
   like the lcircle10/lcirc10 name change many years ago, but let's keep
   the support working nonetheless.  */

static void
target_fontmaps (kpathsea kpse, string **target, unsigned *count,
                 const_string name)
{
  const_string *mapped_names = kpathsea_fontmap_lookup (kpse, name);

  if (mapped_names != NULL) {
    const_string mapped_name;
    /* We leak mapped_names and its elements, some of the time.  */
    while ((mapped_name = *mapped_names++) != NULL) {
      (*target)[(*count)] = xstrdup (mapped_name);
      (*count)++;
      XRETALLOC ((*target), (*count)+1, string);
    }
  }
}


/* Possibly add NAME (and any fontmap equivalents) to the string list
   in TARGET, depending on the various other parameters.  */

static void
target_asis_name (kpathsea kpse, string **target, unsigned *count,
    kpse_file_format_type format,
    const_string name, boolean use_fontmaps, boolean has_potential_suffix,
    string has_any_suffix)
{
  (void) has_any_suffix; /* -Wunused */
  /* Look for the name we've been given, provided non-suffix
     searches are allowed or the name already includes a suffix. */
  if (has_potential_suffix || !FMT_INFO.suffix_search_only) {
    (*target)[(*count)] = xstrdup (name);
    (*count)++;
    XRETALLOC ((*target), (*count)+1, string);

    if (use_fontmaps) {
      target_fontmaps (kpse, target, count, name);
    }
  }
}


/* Possibly add NAME (and any fontmap equivalents), with any suffixes
   for this FORMAT appended, to TARGET -- if it doesn't already have one
   of the potential suffixes for FORMAT.  */

static void
target_suffixed_names (kpathsea kpse, string **target, unsigned *count,
    kpse_file_format_type format,
    const_string name, boolean use_fontmaps, boolean has_potential_suffix)
{
  const_string *ext;
  if (has_potential_suffix || !FMT_INFO.suffix) {
    return;
  }

  for (ext = FMT_INFO.suffix; *ext; ext++) {
    string name_with_suffix = concat (name, *ext);
    (*target)[(*count)] = name_with_suffix;
    (*count)++;
    XRETALLOC ((*target), (*count)+1, string);

    if (use_fontmaps) {
      target_fontmaps (kpse, target, count, name_with_suffix);
    }
  }
}

/* Look up a file NAME of type FORMAT, and the given MUST_EXIST.  This
   initializes the path spec for FORMAT if it's the first lookup of that
   type.  Return the filename found, or NULL.  This is the most likely
   thing for clients to call.  */

string
kpathsea_find_file (kpathsea kpse, const_string name,
                    kpse_file_format_type format, boolean must_exist)
{
  string *ret_list = kpathsea_find_file_generic (kpse, name, format,
                                                 must_exist, false);
  string ret = *ret_list;
  free (ret_list);
  return ret;
}

#if defined (KPSE_COMPAT_API)
string
kpse_find_file (const_string name,  kpse_file_format_type format,
                boolean must_exist)
{
  return kpathsea_find_file(kpse_def, name, format, must_exist);
}
#endif

/* As with `kpse_find_file', but also allow passing ALL for the search,
   hence we always return a NULL-terminated list.  */

string *
kpathsea_find_file_generic (kpathsea kpse, const_string const_name,
                            kpse_file_format_type format,
                            boolean must_exist, boolean all)
{
  string *target, name;
  const_string *ext;
  unsigned count;
  unsigned name_len = 0;
  boolean has_potential_suffix = false;
  string has_any_suffix = NULL;
  string try_std_extension_first = NULL;
  boolean use_fontmaps = (format == kpse_tfm_format
                          || format == kpse_gf_format
                          || format == kpse_pk_format
                          || format == kpse_ofm_format);
  string *ret = NULL;

  /* NAME being NULL is a programming bug somewhere.  NAME can be empty,
     though; this happens with constructs like `\input\relax'.  */
  assert (const_name);

  if (FMT_INFO.path == NULL)
    kpathsea_init_format (kpse, format);

#ifdef KPSE_DEBUG
  if (KPATHSEA_DEBUG_P (KPSE_DEBUG_SEARCH))
    DEBUGF3 ("kpse_find_file: searching for %s of type %s (from %s)\n",
             const_name, FMT_INFO.type, FMT_INFO.path_source);
#endif /* KPSE_DEBUG */

  /* Do variable and tilde expansion. */
  name = kpathsea_expand (kpse, const_name);

  try_std_extension_first
    = kpathsea_var_value (kpse, "try_std_extension_first");
  has_any_suffix = strrchr (name, '.');
  if (has_any_suffix) {
    string p = strchr (has_any_suffix, DIR_SEP);
    if (p) {
      has_any_suffix = NULL;
    }
  }

  /* Does NAME already end in a possible suffix?  */
  name_len = strlen (name);
  if (FMT_INFO.suffix) {
    for (ext = FMT_INFO.suffix; !has_potential_suffix && *ext; ext++) {
      unsigned suffix_len = strlen (*ext);
      has_potential_suffix = (name_len >= suffix_len
          && FILESTRCASEEQ (*ext, name + name_len - suffix_len));
    }
  }
  if (!has_potential_suffix && FMT_INFO.alt_suffix) {
    for (ext = FMT_INFO.alt_suffix; !has_potential_suffix && *ext; ext++) {
      unsigned suffix_len = strlen (*ext);
      has_potential_suffix = (name_len >= suffix_len
          && FILESTRCASEEQ (*ext, name + name_len - suffix_len));
    }
  }

  /* Set up list of target names to search for, the order depending on
     try_std_extension_first.  */
  count = 0;
  target = XTALLOC1 (string);

  if (has_any_suffix
      && (try_std_extension_first == NULL || *try_std_extension_first == 'f'
          || *try_std_extension_first == '0')) {
    target_asis_name (kpse, &target, &count, format, name, use_fontmaps,
                      has_potential_suffix, has_any_suffix);
    target_suffixed_names (kpse, &target, &count, format, name, use_fontmaps,
                           has_potential_suffix);
  } else {
    target_suffixed_names (kpse, &target, &count, format, name, use_fontmaps,
                           has_potential_suffix);
    target_asis_name (kpse, &target, &count, format, name, use_fontmaps,
                      has_potential_suffix, has_any_suffix);
  }

  /* Terminate list. */
  target[count] = NULL;

  if (try_std_extension_first) {
    free (try_std_extension_first);
  }

  /* Search, trying to minimize disk-pounding.  */
  ret = kpathsea_path_search_list_generic (kpse, FMT_INFO.path,
                                           target, false, all);

  /* Do we need to pound the disk? */
  if (! *ret && must_exist) {
    for (count = 0; target[count]; count++)
      free (target[count]);
    count = 0;
    /* We look for a subset of the previous set of names, so the
       target array is large enough.  In particular, we don't pound
       the disk for alternate names from the fontmaps.  */
    if (!has_potential_suffix && FMT_INFO.suffix_search_only) {
      for (ext = FMT_INFO.suffix; *ext; ext++)
        target[count++] = concat (name, *ext);
    }
    if (has_potential_suffix || !FMT_INFO.suffix_search_only) {
      target[count++] = xstrdup (name);
    }
    target[count] = NULL;
    ret = kpathsea_path_search_list_generic (kpse, FMT_INFO.path,
                                             target, true, all);
  }

  /* Free the list we created. */
  for (count = 0; target[count]; count++)
    free (target[count]);
  free (target);

  /* If nothing was found, call mktex* to create a missing file.  Since
     this returns a single string, morph it into a list.  */
  if (! *ret && must_exist) {
    ret = XTALLOC (2, string);
    ret[0] = kpathsea_make_tex (kpse, format, name);
    if (ret[0]) {
      ret[1] = NULL;
    }
  }

  free (name);

  return ret;
}

#if defined (KPSE_COMPAT_API)
string *
kpse_find_file_generic (const_string name,  kpse_file_format_type format,
                        boolean must_exist,  boolean all)
{
  return kpathsea_find_file_generic(kpse_def, name, format, must_exist, all);
}
#endif



/* Return true if FNAME is acceptable to open for reading or writing.  */

typedef enum ok_type {
    ok_reading,
    ok_writing
} ok_type;

static const_string ok_type_name[] = {
    "reading from",
    "writing to"
};

static boolean
kpathsea_name_ok (kpathsea kpse, const_string fname, const_string check_var,
                  const_string default_choice, ok_type action, boolean silent)
{
  /* We distinguish three cases:
     'a' (any)        allows any file to be opened.
     'r' (restricted) means disallowing special file names.
     'p' (paranoid)   means being really paranoid: disallowing special file
                      names and restricting output files to be in or below
                      the working directory or $TEXMFOUTPUT, while input files
                      must be below the current directory, $TEXMFOUTPUT, or
                      (implicitly) in the system areas.
     We default to "paranoid".  The error messages from TeX may be puzzling.
     This function contains several return and goto statements, be careful.  */

  const_string open_choice = kpathsea_var_value (kpse, check_var);

  if (!open_choice)
    open_choice = default_choice;

  if (*open_choice == 'a' || *open_choice == 'y' || *open_choice == '1')
    return true;

#if defined (unix) && !defined (MSDOS)
  {
    /* Disallow .rhosts, .login, .ssh/, ..somefile, ..somedir/somefile,
       etc.  But allow .tex (for base LaTeX).  */
    const_string q;
    const_string qq = fname;
    while ((q = strchr (qq, '.'))) {            /* at each dot */
      if ((q == fname || IS_DIR_SEP (*(q - 1))) /* start or / precedes dot? */
          && !IS_DIR_SEP (*(q + 1))             /* ok if /./ */
          && !(*(q + 1) == '.' && IS_DIR_SEP (*(q + 2))) /* ok  if /../ */
          && !STREQ (q, ".tex")) {              /* specially allow .tex */
        goto not_ok;
      }
      qq = q + 1;
    }
  }
#else
  /* Other OSs don't have special names? */
#endif

  if (*open_choice == 'r' || *open_choice == 'n' || *open_choice == '0')
    return true;

  /* Paranoia originally supplied by Charles Karney.  */
  if (kpathsea_absolute_p (kpse, fname, false)) {
    const_string texmfoutput = kpathsea_var_value (kpse, "TEXMFOUTPUT");
    /* Absolute pathname is only OK if TEXMFOUTPUT is set, it's not empty,
       fname begins the TEXMFOUTPUT, and is followed by / */
    if (!texmfoutput || *texmfoutput == '\0'
        || fname != strstr (fname, texmfoutput)
        || !IS_DIR_SEP (fname[strlen (texmfoutput)]))
      goto not_ok;
  }
  /* For all pathnames, we disallow "../" at the beginning or "/../"
     anywhere.  */
  if (fname[0] == '.' && fname[1] == '.' && IS_DIR_SEP(fname[2]))
    goto not_ok;
  else {
    /* Check for "/../".  Since more than one character can be matched
       by IS_DIR_SEP, we cannot use "/../" itself. */
    const_string dotpair = strstr (fname, "..");
    while (dotpair) {
      /* If dotpair[2] == DIR_SEP, then dotpair[-1] is well-defined,
         because the "../" case was handled above. */
      if (IS_DIR_SEP (dotpair[2]) && IS_DIR_SEP (dotpair[-1]))
        goto not_ok;
      /* Continue after the dotpair. */
      dotpair = strstr (dotpair+2, "..");
    }
  }

  /* We passed all tests.  */
  return true;

 not_ok: /* Some test failed.  */
  if (!silent)
    fprintf (stderr, "\n%s: Not %s %s (%s = %s).\n",
             kpse->invocation_name, ok_type_name[action], fname,
             check_var, open_choice);
  return false;
}

/* For input default to all. */

boolean
kpathsea_in_name_ok_silent (kpathsea kpse, const_string fname)
{
  return kpathsea_name_ok (kpse, fname, "openin_any", "a", ok_reading, true);
}

boolean
kpathsea_in_name_ok (kpathsea kpse, const_string fname)
{
  return kpathsea_name_ok (kpse, fname, "openin_any", "a", ok_reading, false);
}


#if defined(WIN32) || defined(__CYGWIN__)
static int
Isspace (char c)
{
  return (c == ' ' || c == '\t');
}

static boolean
executable_filep (kpathsea kpse, const_string fname, boolean silent)
{
    const_string fn;
    string p, q, base;
    string *pp;

/*  check openout_any */
    p = kpathsea_var_value (kpse, "openout_any");
    if (p && *p == 'p') {
      free (p);
/* get base name
   we cannot use xbasename() for abnormal names.
*/
      p = strrchr (fname, '/');
      if (p)
        fn = p + 1;
      else
        fn = fname;
      p = strrchr (fn, '\\');
      if (p)
        fn = p + 1;
      base = xstrdup (fn);
#if defined(__CYGWIN__)
      for (p = base; *p; p++)
        *p = TOLOWER (*p);
      p = base;
#else
      p = (char *) strlwr (base);
#endif
      for (q = p + strlen (p) - 1;
           (q >= p) && ((*q == '.') || (Isspace (*q))); q--) {
        *q = '\0'; /* remove trailing '.' , ' ' and '\t' */
      }
      q = strrchr (p, '.'); /* get extension part */
      pp = kpse->suffixlist;
      if (pp && q) {
        while (*pp) {
          if (!strcmp (q, *pp)) {
            if (!silent)
              fprintf (stderr, "\n%s: Forbidden to open for writing\n", fname);
            free (base);
            return true;
          }
          pp++;
        }
      }
      free (base);
    } else if (p) {
      free (p);
    }
    return false;
}
#endif /* WIN32 || __CYGWIN__ */

static boolean
kpathsea_out_name_ok_1 (kpathsea kpse, const_string fname, boolean silent)
{
#if defined(WIN32) || defined(__CYGWIN__)
  /* Output of an executable file is restricted on Windows */
  if (executable_filep (kpse, fname, silent))
    return false;
#endif /* WIN32 || __CYGWIN__ */
  /* For output, default to paranoid. */
  return kpathsea_name_ok (kpse, fname, "openout_any", "p", ok_writing,silent);
}

boolean
kpathsea_out_name_ok_silent (kpathsea kpse, const_string fname)
{
  return kpathsea_out_name_ok_1 (kpse, fname, true);
}

boolean
kpathsea_out_name_ok (kpathsea kpse, const_string fname)
{
  return kpathsea_out_name_ok_1 (kpse, fname, false);
}

#if defined (KPSE_COMPAT_API)
boolean
kpse_in_name_ok (const_string fname)
{
  /* For input default to all. */
  return kpathsea_in_name_ok (kpse_def, fname);
}

boolean
kpse_out_name_ok (const_string fname)
{
  /* For output, default to paranoid. */
  return kpathsea_out_name_ok (kpse_def, fname);
}
#endif



/* Open NAME along the search path for TYPE for reading and return the
   resulting file, or exit with an error message.  */

FILE *
kpathsea_open_file (kpathsea kpse, const_string name,
                    kpse_file_format_type type)
{
  string fullname = kpathsea_find_file (kpse, name, type, true);
  const_string mode = kpse->format_info[type].binmode
                      ? FOPEN_RBIN_MODE
                      : FOPEN_R_MODE;
  FILE *f = fullname ? fopen (fullname, mode) : NULL;
  if (!f) {
    if (fullname) {
      perror (fullname);
      exit (1);
    } else {
      LIB_FATAL2 ("%s file `%s' not found", kpse->format_info[type].type, name);
    }
  }

  return f;
}

#if defined (KPSE_COMPAT_API)
FILE *
kpse_open_file (const_string name,  kpse_file_format_type type)
{
    return kpathsea_open_file(kpse_def, name, type);
}
#endif


/* When using the %&<format> construct, we'd like to use the paths for
   that format, rather than those for the name we were called with.
   Of course this happens after various initializations have been
   performed, so we have this function to force the issue.  Note that
   the paths for kpse_cnf_format and kpse_db_format are not cleared.

   This function is defined here, and not in progname.c, because it
   need format_info, and would cause all of tex-file to be pulled
   in by programs that do not need it. */

void
kpathsea_reset_program_name (kpathsea kpse, const_string progname)
{
  int i;

  /* It is a fatal error for either of these to be NULL. */
  assert (progname && kpse->program_name);
  /* Do nothing if the name is unchanged. */
  if (STREQ(kpse->program_name, progname))
    return;

  free (kpse->program_name);
  kpse->program_name = xstrdup (progname);
  kpathsea_xputenv(kpse, "progname", kpse->program_name);

  /* Clear paths -- do we want the db path to be cleared? */
  for (i = 0; i != kpse_last_format; ++i) {
    /* Do not erase the cnf of db paths.  This means that the filename
       database is not rebuilt, nor are different configuration files
       searched.  The alternative is to tolerate a memory leak of up
       to 100k if this function is called. */
    if (i == kpse_cnf_format || i == kpse_db_format)
      continue;
    /* Wipe the path (it is tested) and the cnf_path because their
       values may differ with the new program name.  */
    if (kpse->format_info[i].path != NULL) {
      free (kpse->format_info[i].path);
      kpse->format_info[i].path = NULL;
    }
    /* We cannot free the cnf_path: it points into the cnf hash, which
       means all hell will break loose if we did. */
    if (kpse->format_info[i].cnf_path != NULL) {
      kpse->format_info[i].cnf_path = NULL;
    }
    /* We do not wipe the override_path at this point, though arguably
       we should provide new values.  It is not likely to matter for
       the programs that call this function. */
  }
}

#if defined (KPSE_COMPAT_API)
void
kpse_reset_program_name (const_string progname)
{
  kpathsea_reset_program_name (kpse_def, progname);
}
#endif
