/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
   written by Tim Morgan, drawing from other Unix ports of TeX.  This is
   a collection of miscellany, everything that's easier (or only
   possible) to do in C.

   This file is public domain.  */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-ext.h"

#include <stdio.h> /* For s(n)printf */
#include <time.h> /* For `struct tm'.  Moved here for Visual Studio 2005.  */
#ifndef _MSC_VER
#include <sys/time.h>
#define HAVE_GETTIMEOFDAY
#endif

static char *last_source_name = NULL;
static int last_lineno;

#define check_nprintf(size_get, size_want) \
    if ((unsigned)(size_get) >= (unsigned)(size_want)) \
        _tt_abort ("snprintf failed: file %s, line %d", __FILE__, __LINE__);

/* minimum size for time_str is 24: "D:YYYYmmddHHMMSS+HH'MM'" */
#define TIME_STR_SIZE 30
static char start_time_str[TIME_STR_SIZE];

static void
makepdftime(time_t t, char *time_str, bool utc)
{
    struct tm lt, gmt;
    size_t size;
    int i, off, off_hours, off_mins;

    /* get the time */
    if (utc) {
        lt = *gmtime(&t);
    }
    else {
        lt = *localtime(&t);
    }
    size = strftime(time_str, TIME_STR_SIZE, "D:%Y%m%d%H%M%S", &lt);
    /* expected format: "YYYYmmddHHMMSS" */
    if (size == 0) {
        /* unexpected, contents of time_str is undefined */
        time_str[0] = '\0';
        return;
    }

    /* correction for seconds: %S can be in range 00..61,
       the PDF reference expects 00..59,
       therefore we map "60" and "61" to "59" */
    if (time_str[14] == '6') {
        time_str[14] = '5';
        time_str[15] = '9';
        time_str[16] = '\0';    /* for safety */
    }

    /* get the time zone offset */
    gmt = *gmtime(&t);

    /* this calculation method was found in exim's tod.c */
    off = 60 * (lt.tm_hour - gmt.tm_hour) + lt.tm_min - gmt.tm_min;
    if (lt.tm_year != gmt.tm_year) {
        off += (lt.tm_year > gmt.tm_year) ? 1440 : -1440;
    } else if (lt.tm_yday != gmt.tm_yday) {
        off += (lt.tm_yday > gmt.tm_yday) ? 1440 : -1440;
    }

    if (off == 0) {
        time_str[size++] = 'Z';
        time_str[size] = 0;
    } else {
        off_hours = off / 60;
        off_mins = abs(off - off_hours * 60);
        i = snprintf(&time_str[size], 9, "%+03d'%02d'", off_hours, off_mins);
        check_nprintf(i, 9);
    }
}


void
init_start_time(time_t source_date_epoch)
{
  makepdftime(source_date_epoch, start_time_str, /* utc= */true);
}


void
getcreationdate(void)
{
    size_t len;
    int i;

    /* init_start_time(); -- Tectonic: not needed*/
    /* put creation date on top of string pool and update pool_ptr */
    len = strlen(start_time_str);

    /* In e-pTeX, "init len => call init_start_time()" (as pdftexdir/utils.c)
       yields  unintentional output. */

    if ((unsigned) (pool_ptr + len) >= (unsigned) (pool_size)) {
        pool_ptr = pool_size;
        /* error by str_toks that calls str_room(1) */
        return;
    }

    for (i = 0; i < len; i++)
        str_pool[pool_ptr++] = (uint16_t)start_time_str[i];
}


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


void
get_seconds_and_micros (int32_t *seconds,  int32_t *micros)
{
#ifdef HAVE_GETTIMEOFDAY
  struct timeval tv;
  gettimeofday(&tv, NULL);
  *seconds = tv.tv_sec;
  *micros  = tv.tv_usec;
#else
  /* This is what we use on Windows/MSVC. Less than ideal.
   * We should replace this with a Rust-backed cross-platform API. */
  time_t myclock = time((time_t *) NULL);
  *seconds = myclock;
  *micros  = 0;
#endif
}


/* Given a file name stored in the string pool, insert into the string pool text
 * giving its modification date in PDF-style format. */
void
getfilemoddate(str_number s)
{
  char *name;
  time_t mtime;
  size_t text_len;
  rust_input_handle_t handle;
  char buf[20];

  name = gettexstring(s);
  handle = ttstub_input_open(name, TTBC_FILE_FORMAT_TEX, 0);
  free(name);

  if (handle == NULL)
    return; /* => evaluate to the empty string; intentional */

  mtime = ttstub_input_get_mtime(handle);
  ttstub_input_close(handle);

  makepdftime(mtime, buf, /* utc= */true);
  text_len = strlen(buf);

  if ((unsigned) (pool_ptr + text_len) >= (unsigned) pool_size) {
    pool_ptr = pool_size;
    /* error by str_toks that calls str_room(1) */
  } else {
    int i;

    for (i = 0; i < text_len; i++)
      str_pool[pool_ptr++] = (uint16_t) buf[i];
  }
}

/* Given a file name stored in the string pool, insert into the string pool text
 * giving its size in bytes. */
void
getfilesize(str_number s)
{
  char *name;
  size_t file_len, text_len;
  rust_input_handle_t handle;
  char buf[20];
  int i;

  name = gettexstring(s);
  handle = ttstub_input_open(name, TTBC_FILE_FORMAT_TEX, 0);
  free(name);

  if (handle == NULL)
    return; /* => evaluate to the empty string; intentional */

  file_len = ttstub_input_get_size(handle);
  ttstub_input_close(handle);

  i = snprintf(buf, sizeof(buf), "%lu", (long unsigned int) file_len);
  check_nprintf(i, sizeof(buf));
  text_len = strlen(buf);

  if ((unsigned) (pool_ptr + text_len) >= (unsigned) pool_size) {
      pool_ptr = pool_size;
      /* error by str_toks that calls str_room(1) */
  } else {
      int i;

      for (i = 0; i < text_len; i++)
          str_pool[pool_ptr++] = (uint16_t) buf[i];
  }
}

void getfiledump(int32_t s, int offset, int length)
{
  char *name;
  rust_input_handle_t handle;
  unsigned char *buffer;
  int i, j, k;
  ssize_t actual;
  char strbuf[3];

  if (length == 0)
    return; /* => evaluate to the empty string; intentional */

  if (pool_ptr + 2 * length + 1 >= pool_size) {
      /* not enough room to hold the result; trigger an error back in TeX: */
      pool_ptr = pool_size;
      return;
  }

  buffer = (unsigned char *) xmalloc(length + 1);
  if (buffer == NULL) {
      pool_ptr = pool_size;
      return;
  }

  name = gettexstring(s);
  handle = ttstub_input_open(name, TTBC_FILE_FORMAT_TEX, 0);
  free(name);

  if (handle == NULL) {
    free(buffer);
    return; /* => evaluate to the empty string; intentional */
  }

  ttstub_input_seek(handle, offset, SEEK_SET);
  actual = ttstub_input_read(handle, (char *) buffer, length);
  ttstub_input_close(handle);

  for (j = 0; j < actual; j++) {
    i = snprintf(strbuf, 3, "%.2X", (unsigned int) buffer[j]);
    check_nprintf(i, 3);
    for (k = 0; k < i; k++)
        str_pool[pool_ptr++] = (uint16_t) strbuf[k];
  }

  free(buffer);
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
        ret = ttbc_get_data_md5 ((const unsigned char *) xname, strlen (xname), (unsigned char *) digest);

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
