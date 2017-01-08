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

#include <time.h>
#include <unistd.h>

#include "system.h"
#include "error.h"
#include "mem.h"

#include "dpxutil.h"
#include "mfileio.h"

#include "dpxfile.h"
#include "dpxcrypt.h"
#define MAX_KEY_LEN 16

/*#include <kpathsea/lib.h>*/
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


static int verbose = 0;
int keep_cache = 0;

void
dpx_file_set_verbose (void)
{
  verbose++;
}


/* Kpathsea library does not check file type. */
static int qcheck_filetype (const char *fqpn, dpx_res_type type);


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

  /*kpse_reset_program_name(foolname);*/
  fqpn = kpse_find_file  (filename,
                          (is_text ?
                              kpse_program_text_format :
                              kpse_program_binary_format),
                          false);
  /*kpse_reset_program_name("dvipdfmx");*/

  return  fqpn;
}


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
  fqpn = kpse_find_file(q, kpse_fontmap_format, 0);
  RELEASE(q);

  return  fqpn;
}


static char *
dpx_find_agl_file (const char *filename)
{
  char  *fqpn = NULL;
  char  *q;

  q = ensuresuffix(filename, ".txt");
  fqpn = kpse_find_file(q, kpse_fontmap_format, 0);
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

  fqpn = kpse_find_file(filename, kpse_cmap_format, 0); 

  /* Files found above are assumed to be CMap,
   * if it's not really CMap it will cause an error.
   */
  for (i = 0; !fqpn && fools[i]; i++) { 
    fqpn = dpx_foolsearch(fools[i], filename, 1);
    if (fqpn) {
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
  fqpn = kpse_find_file(q, kpse_sfd_format, 0);
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
  fqpn = kpse_find_file(q, kpse_enc_format, 0);
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
  if (is_absolute_path(q))
    fqpn = xstrdup(q);
  else
    fqpn = kpse_find_file(q, kpse_opentype_format, 0);
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

  return  tmp;
}

#define PREFIX "dvipdfm-x."

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
    /* Tectonic: defused */
    return -1;
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
