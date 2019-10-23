/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
   written by Tim Morgan, drawing from other Unix ports of TeX.  This is
   a collection of miscellany, everything that's easier (or only
   possible) to do in C.

   This file is public domain.  */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-ext.h"

#include <time.h> /* For `struct tm'.  Moved here for Visual Studio 2005.  */


static char *last_source_name = NULL;
static int last_lineno;

void
get_date_and_time (time_t source_date_epoch,
                   int32_t *minutes, int32_t *day,
                   int32_t *month, int32_t *year)
{
  struct tm *tmptr = localtime (&source_date_epoch);
  *minutes = tmptr->tm_hour * 60 + tmptr->tm_min;
  *day = tmptr->tm_mday;
  *month = tmptr->tm_mon + 1;
  *year = tmptr->tm_year + 1900;
}


static void
checkpool_pointer (pool_pointer pool_ptr, size_t len)
{
    if (pool_ptr + len >= pool_size)
        _tt_abort ("string pool overflow [%i bytes]", (int) pool_size);
}


int
maketexstring(const char *s)
{
  size_t len;
  UInt32 rval;
  const unsigned char *cp = (const unsigned char *)s;

  if (s == NULL || *s == 0)
    return EMPTY_STRING;

  len = strlen(s);
  checkpool_pointer (pool_ptr, len); /* in the XeTeX case, this may be more than enough */

  while ((rval = *(cp++)) != 0) {
    UInt16 extraBytes = bytesFromUTF8[rval];
    switch (extraBytes) { /* note: code falls through cases! */
      case 5: rval <<= 6; if (*cp) rval += *(cp++);
      case 4: rval <<= 6; if (*cp) rval += *(cp++);
      case 3: rval <<= 6; if (*cp) rval += *(cp++);
      case 2: rval <<= 6; if (*cp) rval += *(cp++);
      case 1: rval <<= 6; if (*cp) rval += *(cp++);
      case 0: ;
    };
    rval -= offsetsFromUTF8[extraBytes];
    if (rval > 0xffff) {
      rval -= 0x10000;
      str_pool[pool_ptr++] = 0xd800 + rval / 0x0400;
      str_pool[pool_ptr++] = 0xdc00 + rval % 0x0400;
    }
    else
      str_pool[pool_ptr++] = rval;
  }

  return make_string();
}


char *
gettexstring (str_number s)
{
  unsigned int bytesToWrite = 0;
  pool_pointer len, i, j;
  char *name;

  if (s >= 65536L)
      len = str_start[s + 1 - 65536L] - str_start[s - 65536L];
  else
      len = 0;

  name = xmalloc(len * 3 + 1); /* max UTF16->UTF8 expansion
                                  (code units, not bytes) */
  for (i = 0, j = 0; i < len; i++) {
    uint32_t c = str_pool[i + str_start[s - 65536L]];
    if (c >= 0xD800 && c <= 0xDBFF) {
      uint32_t lo = str_pool[++i + str_start[s - 65536L]];
      if (lo >= 0xDC00 && lo <= 0xDFFF)
        c = (c - 0xD800) * 0x0400 + lo - 0xDC00 + 0x10000;
      else
        c = 0xFFFD;
    }

    if (c < 0x80)
      bytesToWrite = 1;
    else if (c < 0x800)
      bytesToWrite = 2;
    else if (c < 0x10000)
      bytesToWrite = 3;
    else if (c < 0x110000)
      bytesToWrite = 4;
    else {
      bytesToWrite = 3;
      c = 0xFFFD;
    }

    j += bytesToWrite;
    switch (bytesToWrite) { /* note: everything falls through. */
      case 4: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 3: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 2: name[--j] = ((c | 0x80) & 0xBF); c >>= 6;
      case 1: name[--j] =  (c | firstByteMark[bytesToWrite]);
    }
    j += bytesToWrite;
  }
  name[j] = 0;
  return name;
}


static int
compare_paths (const char *p1, const char *p2)
{
  int ret;
  while (
         (((ret = (*p1 - *p2)) == 0) && (*p2 != 0))
                || (IS_DIR_SEP(*p1) && IS_DIR_SEP(*p2))) {
       p1++, p2++;
  }
  ret = (ret < 0 ? -1 : (ret > 0 ? 1 : 0));
  return ret;
}


bool
is_new_source (str_number srcfilename, int lineno)
{
  char *name = gettexstring(srcfilename);
  return (compare_paths(name, last_source_name) != 0 || lineno != last_lineno);
}


void
remember_source_info (str_number srcfilename, int lineno)
{
  free(last_source_name);
  last_source_name = gettexstring(srcfilename);
  last_lineno = lineno;
}


pool_pointer
make_src_special (str_number srcfilename, int lineno)
{
  pool_pointer oldpool_ptr = pool_ptr;
  char *filename = gettexstring(srcfilename);
  /* FIXME: Magic number. */
  char buf[40];
  char *s = buf;

  /* Always put a space after the number, which makes things easier
   * to parse.
   */
  sprintf (buf, "src:%d ", lineno);

  if (pool_ptr + strlen(buf) + strlen(filename) >= (size_t)pool_size)
      _tt_abort ("string pool overflow");

  s = buf;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  s = filename;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  return (oldpool_ptr);
}

/* Converts any given string in into an allowed PDF string which is
 * hexadecimal encoded;
 * sizeof(out) should be at least lin*2+1.
 */
static void
convertStringToHexString(const char *in, char *out, int lin)
{
    static const char hexchars[] = "0123456789ABCDEF";
    int i, j;
    j = 0;

    for (i = 0; i < lin; i++) {
        unsigned char c = (unsigned char) in[i];
        out[j++] = hexchars[(c >> 4) & 0xF];
        out[j++] = hexchars[c & 0xF];
    }
    out[j] = '\0';
}

#define DIGEST_SIZE 16

void getmd5sum(str_number s, bool file)
{
    char digest[DIGEST_SIZE];
    char outbuf[2 * DIGEST_SIZE + 1];
    char *xname;
    int ret, i;

    xname = gettexstring (s);

    if (file)
        ret = ttstub_get_file_md5 (xname, digest);
    else
        ret = ttstub_get_data_md5 (xname, strlen (xname), digest);

    free(xname);
    if (ret)
        return;

    if (pool_ptr + 2 * DIGEST_SIZE >= pool_size) {
        /* error by str_toks that calls str_room(1) */
        return;
    }

    convertStringToHexString((char *) digest, outbuf, DIGEST_SIZE);
    for (i = 0; i < 2 * DIGEST_SIZE; i++)
        str_pool[pool_ptr++] = (uint16_t)outbuf[i];
}
