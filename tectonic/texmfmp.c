/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
   written by Tim Morgan, drawing from other Unix ports of TeX.  This is
   a collection of miscellany, everything that's easier (or only
   possible) to do in C.

   This file is public domain.  */

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/md5.h>
#include <tectonic/stubs.h>
#include <tectonic/xetexd.h>
#include <tectonic/XeTeX_ext.h>

#include <time.h> /* For `struct tm'.  Moved here for Visual Studio 2005.  */


static char *last_source_name = NULL;
static int last_lineno;

void
get_date_and_time (integer *minutes,  integer *day,
                   integer *month,  integer *year)
{
  struct tm *tmptr;

  time_t myclock = time ((time_t *) 0);
  tmptr = localtime (&myclock);
  *minutes = tmptr->tm_hour * 60 + tmptr->tm_min;
  *day = tmptr->tm_mday;
  *month = tmptr->tm_mon + 1;
  *year = tmptr->tm_year + 1900;
}


/* Read and write dump files.  As distributed, these files are
   architecture dependent; specifically, BigEndian and LittleEndian
   architectures produce different files.  These routines always output
   BigEndian files.  This still does not guarantee them to be
   architecture-independent, because it is possible to make a format
   that dumps a glue ratio, i.e., a floating-point number.  Fortunately,
   none of the standard formats do that.  */

#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE) /* this fn */

/* This macro is always invoked as a statement.  It assumes a variable
   `temp'.  */

#define SWAP(x, y) temp = (x); (x) = (y); (y) = temp


/* Make the NITEMS items pointed at by P, each of size SIZE, be the
   opposite-endianness of whatever they are now.  */

static void
swap_items (char *p, int nitems, int size)
{
  char temp;

  /* Since `size' does not change, we can write a while loop for each
     case, and avoid testing `size' for each time.  */
  switch (size)
    {
    /* 16-byte items happen on the DEC Alpha machine when we are not
       doing sharable memory dumps.  */
    case 16:
      while (nitems--)
        {
          SWAP (p[0], p[15]);
          SWAP (p[1], p[14]);
          SWAP (p[2], p[13]);
          SWAP (p[3], p[12]);
          SWAP (p[4], p[11]);
          SWAP (p[5], p[10]);
          SWAP (p[6], p[9]);
          SWAP (p[7], p[8]);
          p += size;
        }
      break;

    case 8:
      while (nitems--)
        {
          SWAP (p[0], p[7]);
          SWAP (p[1], p[6]);
          SWAP (p[2], p[5]);
          SWAP (p[3], p[4]);
          p += size;
        }
      break;

    case 4:
      while (nitems--)
        {
          SWAP (p[0], p[3]);
          SWAP (p[1], p[2]);
          p += size;
        }
      break;

    case 2:
      while (nitems--)
        {
          SWAP (p[0], p[1]);
          p += size;
        }
      break;

    case 1:
      /* Nothing to do.  */
      break;

    default:
      FATAL1 ("Can't swap a %d-byte item for (un)dumping", size);
  }
}
#endif /* not WORDS_BIGENDIAN and not NO_DUMP_SHARE */


/* Here we write NITEMS items, each item being ITEM_SIZE bytes long.
   The pointer to the stuff to write is P, and we write to the file
   OUT_FILE.  */

void
do_dump (char *p, int item_size, int nitems,  gzFile out_file)
{
#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif

  if (gzwrite (out_file, p, item_size * nitems) != item_size * nitems)
    {
      fprintf (stderr, "! Could not write %d %d-byte item(s) to %s.\n",
               nitems, item_size, name_of_file+1);
      exit (1);
    }

  /* Have to restore the old contents of memory, since some of it might
     get used again.  */
#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif
}


/* Here is the dual of the writing routine.  */

void
do_undump (char *p, int item_size, int nitems, gzFile in_file)
{
  if (gzread (in_file, p, item_size * nitems) != item_size * nitems)
    FATAL3 ("Could not undump %d %d-byte item(s) from %s",
            nitems, item_size, name_of_file+1);

#if !defined (WORDS_BIGENDIAN) && !defined (NO_DUMP_SHARE)
  swap_items (p, nitems, item_size);
#endif
}

/* FIXME -- some (most?) of this can/should be moved to the Pascal/WEB side. */
static void
checkpool_pointer (pool_pointer pool_ptr, size_t len)
{
  if (pool_ptr + len >= pool_size) {
    fprintf (stderr, "\nstring pool overflow [%i bytes]\n",
            (int)pool_size); /* fixme */
    exit(1);
  }
}

int
maketexstring(const_string s)
{
  size_t len;
  UInt32 rval;
  const unsigned char *cp = (const unsigned char *)s;

  if (s == NULL || *s == 0)
    return get_nullstr();

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


str_number
make_full_name_string(void)
{
  return maketexstring(fullnameoffile);
}


string
gettexstring (str_number s)
{
  unsigned bytesToWrite = 0;
  pool_pointer len, i, j;
  string name;
  len = str_start[s + 1 - 65536L] - str_start[s - 65536L];
  name = xmalloc(len * 3 + 1); /* max UTF16->UTF8 expansion
                                  (code units, not bytes) */
  for (i = 0, j = 0; i < len; i++) {
    unsigned c = str_pool[i + str_start[s - 65536L]];
    if (c >= 0xD800 && c <= 0xDBFF) {
      unsigned lo = str_pool[++i + str_start[s - 65536L]];
      if (lo >= 0xDC00 && lo <= 0xDFFF)
        c = (c - 0xD800) * 0x0400 + lo - 0xDC00;
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
compare_paths (const_string p1, const_string p2)
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


boolean
is_new_source (str_number srcfilename, int lineno)
{
  char *name = gettexstring(srcfilename);
  return (compare_paths(name, last_source_name) != 0 || lineno != last_lineno);
}


void
remember_source_info (str_number srcfilename, int lineno)
{
  if (last_source_name)
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

  if (pool_ptr + strlen(buf) + strlen(filename) >= (size_t)pool_size) {
       fprintf (stderr, "\nstring pool overflow\n"); /* fixme */
       exit (1);
  }
  s = buf;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  s = filename;
  while (*s)
    str_pool[pool_ptr++] = *s++;

  return (oldpool_ptr);
}

#define xfree(p) do { if (p != NULL) free(p); p = NULL; } while (0)

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
#define FILE_BUF_SIZE 1024

void getmd5sum(str_number s, boolean file)
{
    md5_state_t state;
    md5_byte_t digest[DIGEST_SIZE];
    char outbuf[2 * DIGEST_SIZE + 1];
    int len = 2 * DIGEST_SIZE;
    char *xname;
    int i;

    if (file) {
        char file_buf[FILE_BUF_SIZE];
        int read = 0;
        FILE *f;
        char *file_name;

        xname = gettexstring (s);
        file_name = kpse_find_file (xname, kpse_tex_format, true);
        xfree (xname);
        if (file_name == NULL) {
            return;             /* empty string */
        }
        /* in case of error the empty string is returned,
           no need for xfopen that aborts on error.
         */
        f = fopen(file_name, FOPEN_RBIN_MODE);
        if (f == NULL) {
            xfree(file_name);
            return;
        }
        /*recorder_record_input(file_name);*/
        md5_init(&state);
        while ((read = fread(&file_buf, sizeof(char), FILE_BUF_SIZE, f)) > 0) {
            md5_append(&state, (const md5_byte_t *) file_buf, read);
        }
        md5_finish(&state, digest);
        fclose(f);

        xfree(file_name);
    } else {
        /* s contains the data */
        md5_init(&state);
        xname = gettexstring (s);
        md5_append(&state,
                   (md5_byte_t *) xname,
                   strlen (xname));
        xfree (xname);
        md5_finish(&state, digest);
    }

    if (pool_ptr + len >= pool_size) {
        /* error by str_toks that calls str_room(1) */
        return;
    }

    convertStringToHexString((char *) digest, outbuf, DIGEST_SIZE);
    for (i = 0; i < 2 * DIGEST_SIZE; i++)
        str_pool[pool_ptr++] = (uint16_t)outbuf[i];
}

/*
   SyncTeX file name should be full path in the case where
   --output-directory option is given.
   Borrowed from LuaTeX.
*/
char *
generic_synctex_get_current_name (void)
{
  char *pwdbuf, *ret;
  if (!fullnameoffile) {
    ret = xstrdup("");
    return ret;
  }
  if (kpse_absolute_p(fullnameoffile, false)) {
     return xstrdup(fullnameoffile);
  }
  pwdbuf = xgetcwd();
  ret = concat3(pwdbuf, DIR_SEP_STRING, fullnameoffile);
  free(pwdbuf) ;
  return ret;
}


str_number
get_job_name(str_number name)
{
    /* xetex let this be overridden on the command line, but we don't. */
    return name;
}
