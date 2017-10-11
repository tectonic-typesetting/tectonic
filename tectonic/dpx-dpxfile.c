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

#include <stdlib.h>
#include <stdbool.h>
#include <time.h>
#include <unistd.h>

#include "dpx-system.h"
#include "dpx-error.h"
#include "dpx-mem.h"

#include "dpx-dpxutil.h"
#include "dpx-mfileio.h"

#include "dpx-dpxfile.h"
#include "dpx-dpxcrypt.h"

#include "internals.h"

#define MAX_KEY_LEN 16

#include <stdlib.h>
#include <string.h>

static int verbose = 0;
int keep_cache = 0;

void
dpx_file_set_verbose (int level)
{
    verbose = level;
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
static bool
check_stream_is_truetype (rust_input_handle_t handle)
{
    int n;

    ttstub_input_seek (handle, 0, SEEK_SET);
    n = ttstub_input_read (handle, _sbuf, 4);
    ttstub_input_seek (handle, 0, SEEK_SET);

    if (n != 4)
        return false;

    if (!memcmp(_sbuf, "true", 4) || !memcmp(_sbuf, "\0\1\0\0", 4)) /* This doesn't help... */
        return true;

    if (!memcmp(_sbuf, "ttcf", 4))
        return true;

    return false;
}


/* "OpenType" is only for ".otf" here */
static bool
check_stream_is_opentype (rust_input_handle_t handle)
{
    int n;

    ttstub_input_seek (handle, 0, SEEK_SET);
    n = ttstub_input_read (handle, _sbuf, 4);
    ttstub_input_seek (handle, 0, SEEK_SET);

    if (n != 4)
        return false;

    if (!memcmp(_sbuf, "OTTO", 4))
        return true;

    return false;
}


static bool
check_stream_is_type1 (rust_input_handle_t handle)
{
    char *p = _sbuf;
    int n;

    ttstub_input_seek (handle, 0, SEEK_SET);
    n = ttstub_input_read (handle, p, 21);
    ttstub_input_seek (handle, 0, SEEK_SET);

    if (n != 21)
        return false;

    if (p[0] != (char) 0x80 || p[1] < 0 || p[1] > 3)
        return false;

    if (!memcmp(p + 6, "%!PS-AdobeFont", 14) || !memcmp(p + 6, "%!FontType1", 11))
        return true;

    if (!memcmp(p + 6, "%!PS", 4)) {
        /* This was #if-0'd out:
         * p[20] = '\0'; p += 6;
         * dpx_warning("Ambiguous PostScript resource type: %s", (char *) p);
         */
        return true;
    }

    return false;
}


static bool
check_stream_is_dfont (rust_input_handle_t handle)
{
    int i, n;
    uint32_t pos;

    ttstub_input_seek (handle, 0, SEEK_SET);
    tt_get_unsigned_quad(handle);
    pos = tt_get_unsigned_quad (handle);
    ttstub_input_seek (handle, pos + 0x18, SEEK_SET);
    ttstub_input_seek (handle, pos + tt_get_unsigned_pair (handle), SEEK_SET);

    n = tt_get_unsigned_pair (handle);

    for (i = 0; i <= n; i++) {
        if (tt_get_unsigned_quad(handle) == 0x73666e74UL) /* "sfnt" */
            return true;
        tt_get_unsigned_quad(handle);
    }

    return false;
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
    free(q);

    return  fqpn;
}

static char *dpx_find_sfd_file      (const char *filename);
static char *dpx_find_iccp_file     (const char *filename);

FILE *
dpx_open_file (const char *filename, dpx_res_type type)
{
    FILE  *fp   = NULL;
    char  *fqpn = NULL;

    switch (type) {
    case DPX_RES_TYPE_PKFONT:
        break;
    case DPX_RES_TYPE_SFD:
        fqpn = dpx_find_sfd_file(filename);
        break;
    case DPX_RES_TYPE_ICCPROFILE:
        fqpn = dpx_find_iccp_file(filename);
        break;
    case DPX_RES_TYPE_BINARY:
        fqpn = dpx_find__app__xyz(filename, "", 0);
        break;
    case DPX_RES_TYPE_TEXT:
        fqpn = dpx_find__app__xyz(filename, "", 1);
        break;
    default:
        _tt_abort("XXX unhandled dpx_open_file(%s, %d)", filename, type);
    }
    if (fqpn) {
        fp = fopen(fqpn, FOPEN_RBIN_MODE);
        free(fqpn);
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


rust_input_handle_t
dpx_tt_open (const char *filename, const char *suffix, kpse_file_format_type format)
{
    char *q;
    rust_input_handle_t handle;

    q = ensuresuffix(filename, suffix);
    handle = ttstub_input_open(q, format, 0);
    free(q);
    return handle;
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

    q    = ensuresuffix(filename, ".sfd");
    fqpn = kpse_find_file(q, kpse_sfd_format, 0);
    free(q);

    return  fqpn;
}

rust_input_handle_t
dpx_open_type1_file (const char *filename)
{
    rust_input_handle_t handle;

    handle = ttstub_input_open (filename, kpse_type1_format, 0);
    if (handle == NULL)
        return NULL;

    if (!check_stream_is_type1 (handle)) {
        ttstub_input_close (handle);
        return NULL;
    }

    return handle;
}


rust_input_handle_t
dpx_open_truetype_file (const char *filename)
{
    rust_input_handle_t handle;

    handle = ttstub_input_open (filename, kpse_truetype_format, 0);
    if (handle == NULL)
        return NULL;

    if (!check_stream_is_truetype (handle)) {
        ttstub_input_close (handle);
        return NULL;
    }

    return handle;
}


rust_input_handle_t
dpx_open_opentype_file (const char *filename)
{
    rust_input_handle_t handle;
    char *q;

    q = ensuresuffix(filename, ".otf");
    handle = ttstub_input_open (q, kpse_opentype_format, 0);
    free (q);

    if (handle == NULL)
        return NULL;

    if (!check_stream_is_opentype (handle)) {
        ttstub_input_close (handle);
        return NULL;
    }

    return handle;
}


rust_input_handle_t
dpx_open_dfont_file (const char *filename)
{
    char *q;
    rust_input_handle_t handle;
    int len = strlen(filename);

    if (len > 6 && strncmp(filename + len - 6, ".dfont", 6)) {
        /* I've double-checked that we're accurately representing the original
         * code -- the above strncmp() is *not* missing a logical negation.
         */
        q = NEW(len + 6, char);
        strcpy(q, filename);
        strcat(q, "/rsrc");
    } else {
        q = xstrdup (filename);
    }

    handle = ttstub_input_open (q, kpse_truetype_format, 0);
    free (q);
    if (handle == NULL)
        return NULL;

    if (!check_stream_is_dfont (handle)) {
        ttstub_input_close (handle);
        return NULL;
    }

    return handle;
}


static char *
dpx_get_tmpdir (void)
{
#  define __TMPDIR     "/tmp"
    size_t i;
    char *ret;
    const char *_tmpd;

    _tmpd = getenv("TMPDIR");
    if (!_tmpd)
        _tmpd = __TMPDIR;
    ret = xstrdup(_tmpd);
    i = strlen(ret);
    while(i > 1 && IS_DIR_SEP(ret[i-1])) {
        ret[i-1] = '\0';
        i--;
    }
    return ret;
}


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
        free(_tmpd);
        strcat(tmp, TEMPLATE);
        _fd  = mkstemp(tmp);
        if (_fd != -1) {
            close(_fd);
        } else {
            tmp = mfree(tmp);
        }
    }

    return  tmp;
}

#define DPX_PREFIX "dvipdfm-x."

static int
dpx_clear_cache_filter (const struct dirent *ent) {
    size_t plen = strlen(DPX_PREFIX);
    if (strlen(ent->d_name) != plen + MAX_KEY_LEN * 2) return 0;
    return strstartswith(ent->d_name, DPX_PREFIX) == 0;
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
    pathname = NEW(strlen(dir)+1+strlen(DPX_PREFIX)+MAX_KEY_LEN*2 + 1, char);
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
    free(dir);
    free(pathname);
}

void
dpx_delete_temp_file (char *tmp, int force)
{
    if (!tmp)
        return;
    if (force || keep_cache != 1) remove (tmp);
    free(tmp);

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
