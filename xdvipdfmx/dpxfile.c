/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.
    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.
    
    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>
    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.
    
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#ifdef _MSC_VER
#include <kpathsea/dirent.h>
#endif

#if defined(MIKTEX_WINDOWS)
#include <miktex/unxemu.h>
#endif

#include <time.h>

#include "system.h"
#include "error.h"
#include "mem.h"

#include "dpxutil.h"
#include "mfileio.h"

#include "dpxfile.h"
#include "dpxcrypt.h"
#define MAX_KEY_LEN 16

#include <kpathsea/lib.h>
#include <string.h>
#ifdef WIN32
#include <io.h>
#include <process.h>
#include <wchar.h>
#else
#if HAVE_SYS_WAIT_H
#include <sys/wait.h>
#endif
#ifndef WEXITSTATUS
#define WEXITSTATUS(val) ((unsigned)(val) >> 8)
#endif
#ifndef WIFEXITED
#define WIFEXITED(val) (((val) & 255) == 0)
#endif
#endif

#if defined(MIKTEX) || defined(TESTCOMPILE)
#if defined(__APPLE__)
#  include <sys/syslimits.h>
#endif
#if !defined(PATH_MAX)
#  define PATH_MAX 256
#endif
#endif /* MIKTEX || TESTCOMPILE */

static int verbose = 0;
int keep_cache = 0;

void
dpx_file_set_verbose (void)
{
  verbose++;
}


/* Kpathsea library does not check file type. */
static int qcheck_filetype (const char *fqpn, dpx_res_type type);

/* For testing MIKTEX enabled compilation */
#if defined(TESTCOMPILE) && !defined(MIKTEX)
#  define MIKTEX        1
#  define PATH_SEP_CHR  '/'

static int
miktex_get_acrobat_font_dir (char *buf)
{
  strcpy(buf, "/usr/share/ghostscript/Resource/Font/");
  return  1;
}

static int
miktex_find_file (const char *filename, const char *dirlist, char *buf)
{
  int    r = 0;
  char  *fqpn;

  fqpn = kpse_path_search(dirlist, filename, 0);
  if (!fqpn)
    return  0;
  if (strlen(fqpn) > PATH_MAX)
    r = 0;
  else {
    strcpy(buf, fqpn);
    r = 1;
  }
  RELEASE(fqpn);

  return  r;
}

static int
miktex_find_app_input_file (const char *progname, const char *filename, char *buf)
{
  int    r = 0;
  char  *fqpn;

  kpse_reset_program_name(progname);
  fqpn = kpse_find_file  (filename, kpse_program_text_format, false);
  kpse_reset_program_name("dvipdfmx");

  if (!fqpn)
    return  0;
  if (strlen(fqpn) > PATH_MAX)
    r = 0;
  else {
    strcpy(buf, fqpn);
    r = 1;
  }
  RELEASE(fqpn);

  return  r;
}

static int
miktex_find_psheader_file (const char *filename, char *buf)
{
  int    r;
  char  *fqpn;

  fqpn = kpse_find_file(filename, kpse_tex_ps_header_format, 0);

  if (!fqpn)
    return  0;
  if (strlen(fqpn) > PATH_MAX)
    r = 0;
  else {
    strcpy(buf, fqpn);
    r = 1;
  }
  RELEASE(fqpn);

  return  r; 
}

#endif /* TESTCOMPILE */

#ifdef  MIKTEX_NO_KPATHSEA
#ifndef PATH_SEP_CHR
#  define PATH_SEP_CHR '\\'
#endif
static char  _tmpbuf[PATH_MAX+1];
#endif /* MIKTEX */

static int exec_spawn (char *cmd)
{
  char **cmdv, **qv;
  char *p, *pp;
  char buf[1024];
  int  i, ret = -1;
#ifdef WIN32
  wchar_t **cmdvw, **qvw;
#endif

  if (!cmd)
    return -1;
  while (*cmd == ' ' || *cmd == '\t')
    cmd++;
  if (*cmd == '\0')
    return -1;
  i = 0;
  p = cmd;
  while (*p) {
    if (*p == ' ' || *p == '\t')
      i++;
    p++;
  }
  cmdv = xcalloc (i + 2, sizeof (char *));
  p = cmd;
  qv = cmdv;
  while (*p) {
    pp = buf;
    if (*p == '"') {
      p++;
      while (*p != '"') {
        if (*p == '\0') {
          goto done;
        }
        *pp++ = *p++;
      }
      p++;
    } else if (*p == '\'') {
      p++;
      while (*p != '\'') {
        if (*p == '\0') {
          goto done;
        }
        *pp++ = *p++;
      }
      p++;
    } else {
      while (*p != ' ' && *p != '\t' && *p) {
        if (*p == '\'') {
          p++;
          while (*p != '\'') {
             if (*p == '\0') {
                 goto done;
             }
             *pp++ = *p++;
          }
          p++;
        } else {
          *pp++ = *p++;
        }
      }
    }
    *pp = '\0';
#ifdef WIN32
    if (strchr (buf, ' ') || strchr (buf, '\t'))
      *qv = concat3 ("\"", buf, "\"");
    else
#endif
      *qv = xstrdup (buf);
/*
    fprintf(stderr,"\n%s", *qv);
*/
    while (*p == ' ' || *p == '\t')
      p++;
    qv++;
  }
#ifdef WIN32
#if defined(MIKTEX)
  ret = _spawnvp(_P_WAIT, *cmdv, (const char* const*)cmdv); 
#else
  cmdvw = xcalloc (i + 2, sizeof (wchar_t *));
  qv = cmdv;
  qvw = cmdvw;
  while (*qv) {
    *qvw = get_wstring_from_fsyscp(*qv, *qvw=NULL);
    qv++;
    qvw++;
  }
  *qvw = NULL;
  ret = _wspawnvp (_P_WAIT, *cmdvw, (const wchar_t* const*) cmdvw);
  if (cmdvw) {
    qvw = cmdvw;
    while (*qvw) {
      free (*qvw);
      qvw++;
    }
    free (cmdvw);
  }
#endif
#else
  i = fork ();
  if (i < 0)
    ret = -1;
  else if (i == 0) {
    if (execvp (*cmdv, cmdv))
      _exit (-1);
  } else {
    if (wait (&ret) == i) {
      ret = (WIFEXITED (ret) ? WEXITSTATUS (ret) : -1);
    } else {
      ret = -1;
    }
  }
#endif
done:
  qv = cmdv;
  while (*qv) {
    free (*qv);
    qv++;
  }
  free (cmdv);
  return ret;
}

/* ensuresuffix() returns a copy of basename if sfx is "". */
static char *
ensuresuffix (const char *basename, const char *sfx)
{
  char  *q, *p;

  p = NEW(strlen(basename) + strlen(sfx) + 1, char);
  strcpy(p, basename);
  q = strrchr(p, '.');
  if (!q && sfx[0])
    strcat(p, sfx);

  return  p;
}

#ifdef  MIKTEX_NO_KPATHSEA
static char *
dpx_find__app__xyz (const char *filename,
                    const char *suffix, int is_text)
{
  char  *fqpn = NULL;
  int    r;
  char  *q;

  q = ensuresuffix(filename, suffix);
  r = miktex_find_app_input_file("dvipdfmx", q, _tmpbuf);
  if (!r && strcmp(q, filename))
    r = miktex_find_app_input_file("dvipdfmx", filename, _tmpbuf);
  if (r) {
    fqpn = NEW(strlen(_tmpbuf) + 1, char);
    strcpy(fqpn, _tmpbuf);
  }
  RELEASE(q);

  return  fqpn;
}

static char *
dpx_foolsearch (const char  *foolname,
                const char  *filename,
                int          is_text)
{
  char  *fqpn = NULL;
  int    r;

  r = miktex_find_app_input_file(foolname, filename, _tmpbuf);
  if (r) {
    fqpn = NEW(strlen(_tmpbuf) + 1, char);
    strcpy(fqpn, _tmpbuf);
  }

  return  fqpn;
}
#else /* !MIKTEX */
#  define TDS11DOC "http://www.tug.org/ftp/tex/tds-1.1/tds.html#Fonts"
static void
insistupdate (const char      *filename,
              const char      *fqpn,
              const char      *foolname,
              kpse_file_format_type foolformat,
              kpse_file_format_type realformat)
{
#if defined(MIKTEX)
  /* users are not fools */
#else
  kpse_format_info_type *fif;
  kpse_format_info_type *fir;
  if (verbose < 1)
    return;
  fif = &kpse_format_info[foolformat];
  fir = &kpse_format_info[realformat];
  WARN("File name=\"%s\" format=\"%s\" might be found in different location than I expected:",
       filename, fir->type);
  WARN(">>   %s", fqpn);
  WARN(">> Please adjust your TEXMF as conformant with:");
  WARN(">>   " TDS11DOC);
  WARN(">> I searched it with fooling kpathsea as progname=\"%s\" format=\"%s\".",
       foolname, fif->type);
  WARN(">> Default search path for this format file is:");
  WARN(">>   %s", fir->default_path);
  WARN(">> Please read \"README\" file.");
#endif
}

static char *
dpx_find__app__xyz (const char *filename,
                    const char *suffix, int is_text)
{
  char  *fqpn = NULL;
  char  *q;

  q    = ensuresuffix(filename, suffix);
  fqpn = kpse_find_file(q,
                         (is_text ?
                            kpse_program_text_format : kpse_program_binary_format), 0);
  if (!fqpn && strcmp(q, filename))
    fqpn = kpse_find_file(filename,
                           (is_text ?
                              kpse_program_text_format : kpse_program_binary_format), 0);   
  RELEASE(q);

  return  fqpn;
}

static char *
dpx_foolsearch (const char  *foolname,
                const char  *filename,
                int          is_text)
{
  char  *fqpn = NULL;

  kpse_reset_program_name(foolname);
  fqpn = kpse_find_file  (filename,
                          (is_text ?
                              kpse_program_text_format :
                              kpse_program_binary_format),
                          false);
  kpse_reset_program_name("dvipdfmx");

  return  fqpn;
}
#endif /* MIKTEX */

static char *dpx_find_fontmap_file  (const char *filename);
static char *dpx_find_agl_file      (const char *filename);
static char *dpx_find_sfd_file      (const char *filename);
static char *dpx_find_cmap_file     (const char *filename);
static char *dpx_find_enc_file      (const char *filename);
static char *dpx_find_iccp_file     (const char *filename);

FILE *
dpx_open_file (const char *filename, dpx_res_type type)
{
  FILE  *fp   = NULL;
  char  *fqpn = NULL;

  switch (type) {
  case DPX_RES_TYPE_FONTMAP:
    fqpn = dpx_find_fontmap_file(filename);
    if (verbose) {
      if (fqpn != NULL)
        MESG(fqpn);
    }
    break;
  case DPX_RES_TYPE_T1FONT:
    fqpn = dpx_find_type1_file(filename);
    break;
  case DPX_RES_TYPE_TTFONT:
    fqpn = dpx_find_truetype_file(filename);
    break;
  case DPX_RES_TYPE_OTFONT:
    fqpn = dpx_find_opentype_file(filename);
    break;
  case DPX_RES_TYPE_PKFONT:
    break;
  case DPX_RES_TYPE_CMAP:
    fqpn = dpx_find_cmap_file(filename);
    break;
  case DPX_RES_TYPE_ENC:
    fqpn = dpx_find_enc_file(filename);
    break;
  case DPX_RES_TYPE_SFD:
    fqpn = dpx_find_sfd_file(filename);
    break;
  case DPX_RES_TYPE_AGL:
    fqpn = dpx_find_agl_file(filename);
    break;
  case DPX_RES_TYPE_ICCPROFILE:
    fqpn = dpx_find_iccp_file(filename);
    break;
  case DPX_RES_TYPE_DFONT:
    fqpn = dpx_find_dfont_file(filename);
    break;
  case DPX_RES_TYPE_BINARY:
    fqpn = dpx_find__app__xyz(filename, "", 0);
    break;
  case DPX_RES_TYPE_TEXT:
    fqpn = dpx_find__app__xyz(filename, "", 1);
    break;
  }
  if (fqpn) {
    fp = MFOPEN(fqpn, FOPEN_RBIN_MODE);
    RELEASE(fqpn);
  }

  return  fp;
}


static char *
dpx_find_iccp_file (const char *filename)
{
  char  *fqpn = NULL;

  fqpn = dpx_find__app__xyz(filename, "", 0);
  if (fqpn || strrchr(filename, '.'))
    return  fqpn;

  fqpn = dpx_find__app__xyz(filename, ".icc", 0);
  if (fqpn)
    return  fqpn;

  fqpn = dpx_find__app__xyz(filename, ".icm", 0);

  return  fqpn;
}


static char *
dpx_find_fontmap_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;

  q = ensuresuffix(filename, ".map");
#ifdef  MIKTEX_NO_KPATHSEA
  fqpn = dpx_find__app__xyz(q, ".map", 1);
#else /* !MIKTEX */
  fqpn = kpse_find_file(q, kpse_fontmap_format, 0);
  if (!fqpn) {
    fqpn = dpx_find__app__xyz(q, ".map", 1);
    if (fqpn)
      insistupdate(q, fqpn, "dvipdfmx",
                   kpse_program_text_format, kpse_fontmap_format); 
  }
#endif /* MIKETEX */
  RELEASE(q);

  return  fqpn;
}


static char *
dpx_find_agl_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;

  q = ensuresuffix(filename, ".txt");
#ifdef  MIKTEX_NO_KPATHSEA
  fqpn = dpx_find__app__xyz(q, ".txt", 1);
#else /* !MIKTEX */
  fqpn = kpse_find_file(q, kpse_fontmap_format, 0);
  if (!fqpn) {
    fqpn = dpx_find__app__xyz(q, ".txt", 1);
    if (fqpn)
      insistupdate(q, fqpn, "dvipdfmx",
                   kpse_program_text_format, kpse_fontmap_format); 
  }
#endif /* MIKETEX */
  RELEASE(q);

  return  fqpn;
}


/* cmap.sty put files into tex/latex/cmap */
static char *
dpx_find_cmap_file (const char *filename)
{
  char  *fqpn = NULL;
  static const char *fools[] = {
    "cmap", "tex", NULL
  };
  int    i;

#if  defined(MIKTEX_NO_KPATHSEA)
  /* Find in Acrobat's Resource/CMap dir */
  {
    char  _acrodir[PATH_MAX+1];
    char  *q;
    int    r;

    memset(_acrodir, 0, PATH_MAX+1);
    r = miktex_get_acrobat_font_dir(_acrodir);
    if (r &&
        strlen(_acrodir) > strlen("Font")) {
      /* ....\Font\ */
      q = strrchr(_acrodir, PATH_SEP_CHR);
      if (q && q[1] == '\0')
        q[0] = '\0';
      q = strrchr(_acrodir, PATH_SEP_CHR);
      if (q && !strcmp(q + 1, "Font")) {
        sprintf(q, "%cCMap%c", PATH_SEP_CHR, PATH_SEP_CHR);
        r = miktex_find_file(filename, _acrodir, _tmpbuf);
        if (r) {
          fqpn = NEW(strlen(_tmpbuf) + 1, char);
          strcpy(fqpn, _tmpbuf);
        }
      }
    }
    memset(_tmpbuf, 0, PATH_MAX+1);
  }
#else
  fqpn = kpse_find_file(filename, kpse_cmap_format, 0); 
#endif

  /* Files found above are assumed to be CMap,
   * if it's not really CMap it will cause an error.
   */
  for (i = 0; !fqpn && fools[i]; i++) { 
    fqpn = dpx_foolsearch(fools[i], filename, 1);
    if (fqpn) {
#ifndef  MIKTEX_NO_KPATHSEA
      insistupdate(filename, fqpn, fools[i],
                   kpse_program_text_format, kpse_cmap_format); 
#endif
      if (!qcheck_filetype(fqpn, DPX_RES_TYPE_CMAP)) {
        WARN("Found file \"%s\" for PostScript CMap but it doesn't look like a CMap...", fqpn);
        RELEASE(fqpn);
        fqpn = NULL;
      }
    }
  }

  return  fqpn;
}


/* Search order:
 *   SFDFONTS (TDS 1.1)
 *   ttf2pk   (text file)
 *   ttf2tfm  (text file)
 *   dvipdfm  (text file)   
 */
static char *
dpx_find_sfd_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;
  static const char *fools[] = {
    "ttf2pk", "ttf2tfm", NULL
  };
  int    i;

  q    = ensuresuffix(filename, ".sfd");
#ifndef  MIKTEX_NO_KPATHSEA
  fqpn = kpse_find_file(q, kpse_sfd_format, 0);
#endif /* !MIKTEX */

  for (i = 0; !fqpn && fools[i]; i++) { 
    fqpn = dpx_foolsearch(fools[i], q, 1);
#ifndef  MIKTEX_NO_KPATHSEA
    if (fqpn)
      insistupdate(filename, fqpn, fools[i],
                   kpse_program_text_format, kpse_sfd_format); 
#endif
  }
  RELEASE(q);

  return  fqpn;
}


static char *
dpx_find_enc_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;
  static const char *fools[] = {
    "dvips", NULL
  };
  int    i;

  q = ensuresuffix(filename, ".enc");
#ifdef  MIKTEX_NO_KPATHSEA
  if (miktex_find_psheader_file(q, _tmpbuf)) {
    fqpn = NEW(strlen(_tmpbuf) + 1, char);
    strcpy(fqpn, _tmpbuf);
  }
#else
  fqpn = kpse_find_file(q, kpse_enc_format, 0);
#endif /* MIKTEX */

  for (i = 0; !fqpn && fools[i]; i++) { 
    fqpn = dpx_foolsearch(fools[i], q, 1);
#ifndef  MIKTEX_NO_KPATHSEA
    if (fqpn)
      insistupdate(filename, fqpn, fools[i],
                   kpse_program_text_format, kpse_enc_format); 
#endif
  }
  RELEASE(q);

  return  fqpn;
}

static int
is_absolute_path(const char *filename)
{
#ifdef WIN32
  if (isalpha(filename[0]) && filename[1] == ':')
    return 1;
  if (filename[0] == '\\' && filename[1] == '\\')
    return 1;
  if (filename[0] == '/' && filename[1] == '/')
    return 1;
#else
  if (filename[0] == '/')
    return 1;
#endif
  return 0;
}

char *
dpx_find_type1_file (const char *filename)
{
  char  *fqpn = NULL;

  if (is_absolute_path(filename))
    fqpn = xstrdup(filename);
  else
    fqpn = kpse_find_file(filename, kpse_type1_format, 0);
  if (fqpn && !qcheck_filetype(fqpn, DPX_RES_TYPE_T1FONT)) {
    RELEASE(fqpn);
    fqpn = NULL;
  }

  return  fqpn;
}


char *
dpx_find_truetype_file (const char *filename)
{
  char  *fqpn = NULL;

  if (is_absolute_path(filename))
    fqpn = xstrdup(filename);
  else
    fqpn = kpse_find_file(filename, kpse_truetype_format, 0);
  if (fqpn && !qcheck_filetype(fqpn, DPX_RES_TYPE_TTFONT)) {
    RELEASE(fqpn);
    fqpn = NULL;
  }

  return  fqpn;
}


char *
dpx_find_opentype_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;

  q = ensuresuffix(filename, ".otf");
#ifndef MIKTEX_NO_KPATHSEA
  if (is_absolute_path(q))
    fqpn = xstrdup(q);
  else
    fqpn = kpse_find_file(q, kpse_opentype_format, 0);
  if (!fqpn) {
#endif
    fqpn = dpx_foolsearch("dvipdfmx", q, 0);
#ifndef  MIKTEX_NO_KPATHSEA
    if (fqpn)
      insistupdate(filename, fqpn, "dvipdfmx",
                   kpse_program_binary_format, kpse_opentype_format); 
  }
#endif
  RELEASE(q);

  /* *We* use "opentype" for ".otf" (CFF). */
  if (fqpn && !qcheck_filetype(fqpn, DPX_RES_TYPE_OTFONT)) {
    RELEASE(fqpn);
    fqpn = NULL;
  }

  return  fqpn;
}


char *
dpx_find_dfont_file (const char *filename)
{
  char *fqpn = NULL;

  fqpn = kpse_find_file(filename, kpse_truetype_format, 0);
  if (fqpn) {
    int len = strlen(fqpn);
    if (len > 6 && strncmp(fqpn+len-6, ".dfont", 6)) {
      fqpn = RENEW(fqpn, len+6, char);
      strcat(fqpn, "/rsrc");
    }
  }
  if (!qcheck_filetype(fqpn, DPX_RES_TYPE_DFONT)) {
    RELEASE(fqpn);
    fqpn = NULL;
  }
  return fqpn;
}
 
static char *
dpx_get_tmpdir (void)
{
#ifdef WIN32
#  define __TMPDIR     "."
#else /* WIN32 */
#  define __TMPDIR     "/tmp"
#endif /* WIN32 */
    size_t i;
    char *ret;
    const char *_tmpd;

#ifdef  HAVE_GETENV
    _tmpd = getenv("TMPDIR");
#  ifdef WIN32
    if (!_tmpd)
      _tmpd = getenv("TMP");
    if (!_tmpd)
      _tmpd = getenv("TEMP");
#  endif /* WIN32 */
    if (!_tmpd)
      _tmpd = __TMPDIR;
#else /* HAVE_GETENV */
    _tmpd = __TMPDIR;
#endif /* HAVE_GETENV */
    ret = xstrdup(_tmpd);
    i = strlen(ret);
    while(i > 1 && IS_DIR_SEP(ret[i-1])) {
      ret[i-1] = '\0';
      i--;
    }
    return ret;
}

#ifdef  HAVE_MKSTEMP
#  include <stdlib.h>
#endif

char *
dpx_create_temp_file (void)
{
  char  *tmp = NULL;

#if defined(MIKTEX)
  {
    tmp = NEW(PATH_MAX + 1, char);
    miktex_create_temp_file_name(tmp); /* FIXME_FIXME */
#if defined(MIKTEX_WINDOWS)
    {
      char * lpsz;
      for (lpsz = tmp; *lpsz != 0; ++ lpsz)
      {
	if (*lpsz == '\\')
	{
	  *lpsz = '/';
	}
      }
    }
#endif
  }
#elif defined(HAVE_MKSTEMP)
#  define TEMPLATE     "/dvipdfmx.XXXXXX"
  {
    char *_tmpd;
    int  _fd = -1;
    _tmpd = dpx_get_tmpdir();
    tmp = NEW(strlen(_tmpd) + strlen(TEMPLATE) + 1, char);
    strcpy(tmp, _tmpd);
    RELEASE(_tmpd);
    strcat(tmp, TEMPLATE);
    _fd  = mkstemp(tmp);
    if (_fd != -1) {
#  ifdef WIN32
      char *p;
      for (p = tmp; *p; p++) {
        if (IS_KANJI (p))
          p++;
        else if (*p == '\\')
          *p = '/';
      }
      _close(_fd);
#  else
      close(_fd);
#  endif /* WIN32 */
    } else {
      RELEASE(tmp);
      tmp = NULL;
    }
  }
#else /* use _tempnam or tmpnam */
  {
#  ifdef WIN32
    char *_tmpd;
    char *p;
    _tmpd = dpx_get_tmpdir();
    tmp = _tempnam (_tmpd, "dvipdfmx.");
    RELEASE(_tmpd);
    for (p = tmp; *p; p++) {
      if (IS_KANJI(p))
        p++;
      else if (*p == '\\')
        *p = '/';
    }
#  else /* WIN32 */
    char *_tmpa = NEW(L_tmpnam + 1, char);
    tmp = tmpnam(_tmpa);
    if (!tmp)
      RELEASE(_tmpa);
#  endif /* WIN32 */
  }
#endif /* MIKTEX */

  return  tmp;
}

char *
dpx_create_fix_temp_file (const char *filename)
{
#define PREFIX "dvipdfm-x."
  static char *dir = NULL;
  static char *cwd = NULL;
  char *ret, *s;
  int i;
  MD5_CONTEXT state;
  unsigned char digest[MAX_KEY_LEN];
#ifdef WIN32
  char *p;
#endif

  if (!dir) {
      dir = dpx_get_tmpdir();
      cwd = xgetcwd();
  }

  MD5_init(&state);
  MD5_write(&state, (unsigned char *)cwd,      strlen(cwd));
  MD5_write(&state, (unsigned const char *)filename, strlen(filename));
  MD5_final(digest, &state);

  ret = NEW(strlen(dir)+1+strlen(PREFIX)+MAX_KEY_LEN*2 + 1, char);
  sprintf(ret, "%s/%s", dir, PREFIX);
  s = ret + strlen(ret);
  for (i=0; i<MAX_KEY_LEN; i++) {
      sprintf(s, "%02x", digest[i]);
      s += 2;
  }
#ifdef WIN32
  for (p = ret; *p; p++) {
#if defined(MIKTEX)
    if (*p == '\\')
      *p = '/';
#else
    if (IS_KANJI (p))
      p++;
    else if (*p == '\\')
      *p = '/';
#endif
  }
#endif
  /* printf("dpx_create_fix_temp_file: %s\n", ret); */
  return ret;
}

static int
dpx_clear_cache_filter (const struct dirent *ent) {
    int plen = strlen(PREFIX);
    if (strlen(ent->d_name) != plen + MAX_KEY_LEN * 2) return 0;
#ifdef WIN32
    return strncasecmp(ent->d_name, PREFIX, plen) == 0;
#else
    return strncmp(ent->d_name, PREFIX, plen) == 0;
#endif
}

void
dpx_delete_old_cache (int life)
{
  char *dir;
  char *pathname;
  DIR *dp;
  struct dirent *de;
  time_t limit;

  if (life == -2) {
      keep_cache = -1;
      return;
  }

  dir = dpx_get_tmpdir();
  pathname = NEW(strlen(dir)+1+strlen(PREFIX)+MAX_KEY_LEN*2 + 1, char);
  limit = time(NULL) - life * 60 * 60;

  if (life >= 0) keep_cache = 1;
  if ((dp = opendir(dir)) != NULL) {
      while((de = readdir(dp)) != NULL) {
          if (dpx_clear_cache_filter(de)) {
              struct stat sb;
              sprintf(pathname, "%s/%s", dir, de->d_name);
              stat(pathname, &sb);
              if (sb.st_mtime < limit) {
                  remove(pathname);
                  /* printf("remove: %s\n", pathname); */
              }
          }
      }
      closedir(dp);
  }
  RELEASE(dir);
  RELEASE(pathname);
}

void
dpx_delete_temp_file (char *tmp, int force)
{
  if (!tmp)
    return;
  if (force || keep_cache != 1) remove (tmp);
  RELEASE(tmp);

  return;
}

/* dpx_file_apply_filter() is used for converting unsupported graphics
 * format to one of the formats that dvipdfmx can natively handle.
 * 'input' is the filename of the original file and 'output' is actually
 * temporal files 'generated' by the above routine.   
 * This should be system dependent. (MiKTeX may want something different)
 * Please modify as appropriate (see also pdfximage.c and dvipdfmx.c).
 */
int
dpx_file_apply_filter (const char *cmdtmpl,
                      const char *input, const char *output,
                      unsigned char version)
{
  char   *cmd = NULL;
  const char   *p, *q;
  size_t  n, size;
  int     error = 0;

  if (!cmdtmpl)
    return -1;
  else if (!input || !output)
    return -1;

  size = strlen(cmdtmpl) + strlen(input) + strlen(output) + 3;
  cmd  = NEW(size, char);
  memset(cmd, 0, size);
  for (n = 0, p = cmdtmpl; *p != 0; p++) {
#define need(s,l,m,n) \
if ((l) + (n) >= (m)) { \
  (m) += (n) + 128; \
  (s)  = RENEW((s), (m), char); \
}
    if (p[0] == '%') {
      p++;
      switch (p[0]) {
      case 'o': /* Output file name */
        need(cmd, n, size, strlen(output));
        strcpy(cmd + n, output); n += strlen(output);
        break;
      case 'i': /* Input filename */
        need(cmd, n, size, strlen(input));
        strcpy(cmd + n, input);  n += strlen(input);
        break;
      case 'b':
        need(cmd, n, size, strlen(input));
        q = strrchr(input, '.'); /* wrong */
        if (q) {
          memcpy(cmd + n, input, (int) (q - input));
          n += (int) (q - input);
        } else {
          strcpy(cmd + n, input); n += strlen(input);
        }
        break;
      case  'v': /* Version number, e.g. 1.4 */ {
       char buf[6];
       sprintf(buf, "1.%hu", (unsigned short) version);
       need(cmd, n, size, strlen(buf));
       strcpy(cmd + n, buf);  n += strlen(buf);
       break;
      }
      case  0:
        break;
      case '%':
        need(cmd, n, size, 1);
        cmd[n] = '%'; n++;
        break;
      }
    } else {
      need(cmd, n, size, 1);
      cmd[n] = p[0]; n++;
    }
  }
  need(cmd, n, size, 1);
  cmd[n] = '\0';
  if (strlen(cmd) == 0) {
    RELEASE(cmd);
    return -1;
  }

  error = exec_spawn(cmd);
  if (error)
    WARN("Filtering file via command -->%s<-- failed.", cmd);
  RELEASE(cmd);

  return  error;
}

static char _sbuf[128];
/*
 * SFNT type sigs:
 *  `true' (0x74727565): TrueType (Mac)
 *  `typ1' (0x74797031) (Mac): PostScript font housed in a sfnt wrapper
 *  0x00010000: TrueType (Win)/OpenType
 *  `OTTO': PostScript CFF font with OpenType wrapper
 *  `ttcf': TrueType Collection
 */
static int
istruetype (FILE *fp)
{
  int   n;

  rewind(fp);
  n = fread(_sbuf, 1, 4, fp);
  rewind(fp);

  if (n != 4)
    return  0;
  else if (!memcmp(_sbuf, "true", 4) ||
           !memcmp(_sbuf, "\0\1\0\0", 4)) /* This doesn't help... */
    return  1;
  else if (!memcmp(_sbuf, "ttcf", 4))
    return  1;

  return  0;
}
      
/* "OpenType" is only for ".otf" here */
static int
isopentype (FILE *fp)
{
  int   n;

  rewind(fp);
  n = fread(_sbuf, 1, 4, fp);
  rewind(fp);

  if (n != 4)
    return  0;
  else if (!memcmp(_sbuf, "OTTO", 4))
    return  1;
  else
    return  0;
}

static int
ist1binary (FILE *fp)
{
  char *p;
  int   n;

  rewind(fp);
  n = fread(_sbuf, 1, 21, fp);
  rewind(fp);

  p = _sbuf;
  if (n != 21)
    return  0;
  else if (p[0] != (char) 0x80 || p[1] < 0 || p[1] > 3)
    return  0;
  else if (!memcmp(p + 6, "%!PS-AdobeFont", 14) ||
           !memcmp(p + 6, "%!FontType1", 11))
    return  1;
  else if (!memcmp(p + 6, "%!PS", 4)) {
#if  0
    p[20] = '\0'; p += 6;
    WARN("Ambiguous PostScript resource type: %s", (char *) p);
#endif
    return  1;
  }
  /* Otherwise ambiguious */
  return  0;
}

/* %!PS-Adobe-x.y Resource-CMap */
static int
ispscmap (FILE *fp)
{
  char  *p;
  p = mfgets(_sbuf, 128, fp); p[127] = '\0';
  if (!p || strlen(p) < 4 || memcmp(p, "%!PS", 4))
    return 0;
  for (p += 4; *p && !isspace((unsigned char)*p); p++);
  for ( ; *p && (*p == ' ' || *p == '\t'); p++);
  if (*p == '\0' || strlen(p) < strlen("Resource-CMap"))
    return  0;
  else if (!memcmp(p, "Resource-CMap", strlen("Resource-CMap")))
    return  1;
  /* Otherwise ambiguious */
  return  0;
}

static int
isdfont (FILE *fp)
{
  int i, n;
  uint32_t pos;

  rewind(fp);

  get_unsigned_quad(fp);
  seek_absolute(fp, (pos = get_unsigned_quad(fp)) + 0x18);
  seek_absolute(fp, pos + get_unsigned_pair(fp));
  n = get_unsigned_pair(fp);
  for (i = 0; i <= n; i++) {
    if (get_unsigned_quad(fp) == 0x73666e74UL) /* "sfnt" */
      return 1;
    get_unsigned_quad(fp);
  }
  return 0;
}
      
/* This actually opens files. */
static int
qcheck_filetype (const char *fqpn, dpx_res_type type)
{
  int    r = 1;
  FILE  *fp;
  struct stat sb;

  if (!fqpn)
    return  0;

  if (stat(fqpn, &sb) != 0)
    return 0;

  if (sb.st_size == 0)
    return 0;

  fp = MFOPEN(fqpn, FOPEN_RBIN_MODE);
  if (!fp) {
    WARN("File \"%s\" found but I could not open that...", fqpn);
    return  0;
  }
  switch (type) {
  case DPX_RES_TYPE_T1FONT:
    r = ist1binary(fp);
    break;
  case DPX_RES_TYPE_TTFONT:
    r = istruetype(fp);
    break;
  case DPX_RES_TYPE_OTFONT:
    r = isopentype(fp);
    break;
  case DPX_RES_TYPE_CMAP:
    r = ispscmap(fp);
    break;
  case DPX_RES_TYPE_DFONT:
    r = isdfont(fp);
    break;
  default:
    break;
  }
  MFCLOSE(fp);

  return  r;
}
