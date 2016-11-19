/* kpathsea utilities that are not really related to its core file-opening mission.

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
   along with this library; if not, see <http://www.gnu.org/licenses/>.
*/

#ifndef TIDY_KPATHUTIL_PUBLIC_H
#define TIDY_KPATHUTIL_PUBLIC_H

#include <assert.h>
#include <ctype.h>
#include <dirent.h>
#include <errno.h>
#include <fcntl.h>
#include <float.h>
#include <getopt.h>
#include <inttypes.h>
#include <limits.h>
#include <math.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <unistd.h>

/* high-level defines */

#ifndef __cplusplus
#ifndef true
#define true 1
#define false 0
#endif /* not true */
#endif /* not __cplusplus */

#ifndef FALSE
#define FALSE false
#define TRUE true
#endif /* FALSE */

#define MAKE_TEX_FMT_BY_DEFAULT 0
#define MAKE_TEX_MF_BY_DEFAULT 0
#define MAKE_TEX_PK_BY_DEFAULT 0
#define MAKE_TEX_TEX_BY_DEFAULT 0
#define MAKE_TEX_TFM_BY_DEFAULT 0

#define HAVE_ASSERT_H 1
#define HAVE_DECL_ISASCII 1
#define HAVE_DECL_PUTENV 1
#define HAVE_DIRENT_H 1
#define HAVE_INTTYPES_H 1
#define HAVE_FLOAT_H 1
#define HAVE_GETCWD 1
#define HAVE_GETWD 1
#define HAVE_LIMITS_H 1
#define HAVE_MEMCMP 1
#define HAVE_MEMCPY 1
#define HAVE_MEMORY_H 1
#define HAVE_PWD_H 1
#define HAVE_STDINT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRCHR 1
#define HAVE_STRING_H 1
#define HAVE_STRINGS_H 1
#define HAVE_STRRCHR 1
#define HAVE_SYS_PARAM_H 1
#define HAVE_UNISTD_H 1
#define SIZEOF_LONG 8
#define STDC_HEADERS 1

/* lib.h */

#define FATAL_PERROR(str) do {			       \
  fprintf (stderr, "%s: ", "(pkgwhack)"); \
  perror (str); exit (EXIT_FAILURE); } while (0)
#define START_FATAL() do { \
  fprintf (stderr, "%s: fatal: ", "(pkgwhack)");
#define END_FATAL() fputs (".\n", stderr); exit (1); } while (0)

#define FATAL(str)                                                      \
  START_FATAL (); fputs (str, stderr); END_FATAL ()
#define FATAL1(str, e1)                                                 \
  START_FATAL (); fprintf (stderr, str, e1); END_FATAL ()
#define FATAL2(str, e1, e2)                             \
   START_FATAL (); fprintf (stderr, str, e1, e2); END_FATAL ()
#define FATAL3(str, e1, e2, e3)                             \
   START_FATAL (); fprintf (stderr, str, e1, e2, e3); END_FATAL ()
#define FATAL4(str, e1, e2, e3, e4)                             \
   START_FATAL (); fprintf (stderr, str, e1, e2, e3, e4); END_FATAL ()
#define FATAL5(str, e1, e2, e3, e4, e5)                             \
   START_FATAL (); fprintf (stderr, str, e1, e2, e3, e4, e5); END_FATAL ()
#define FATAL6(str, e1, e2, e3, e4, e5, e6)                       \
   START_FATAL (); fprintf (stderr, str, e1, e2, e3, e4, e5, e6); END_FATAL ()

#define START_WARNING() do { fputs ("warning: ", stderr)
#define END_WARNING() fputs (".\n", stderr); fflush (stderr); } while (0)

#define WARNING(str)                                                    \
  START_WARNING (); fputs (str, stderr); END_WARNING ()
#define WARNING1(str, e1)                                               \
  START_WARNING (); fprintf (stderr, str, e1); END_WARNING ()
#define WARNING2(str, e1, e2)                                           \
  START_WARNING (); fprintf (stderr, str, e1, e2); END_WARNING ()
#define WARNING3(str, e1, e2, e3)                                       \
  START_WARNING (); fprintf (stderr, str, e1, e2, e3); END_WARNING ()
#define WARNING4(str, e1, e2, e3, e4)                                   \
  START_WARNING (); fprintf (stderr, str, e1, e2, e3, e4); END_WARNING ()

/* config.h */

#define STREQ(s1, s2) (((s1) != NULL) && ((s2) != NULL) && (strcmp (s1, s2) == 0))
#define STRNEQ(s1, s2, n) ((s1) && (s2) && (strncmp (s1, s2, n) == 0))
#define FILESTRCASEEQ STREQ
#define FILESTRNCASEEQ STRNEQ
#define FILECHARCASEEQ(c1,c2) ((c1) == (c2))
#define MAX_INT_LENGTH 21
#define DEV_NULL "/dev/null"

/* other lame #defines */

#define XRETALLOC(addr, n, t) ((addr) = (t *) xrealloc (addr, (n) * sizeof(t)))

#ifndef isblank
#define isblank(c) ((c) == ' ' || (c) == '\t')
#endif
#define ISBLANK(c) (isascii (c) && isblank ((unsigned char)c))

/* c-fopen.h */

#define FOPEN_A_MODE "ab"
#define FOPEN_R_MODE "r"
#define FOPEN_W_MODE "wb"
#define FOPEN_RBIN_MODE "rb"
#define FOPEN_WBIN_MODE "wb"
#define FOPEN_ABIN_MODE "ab"

/* c-pathch.h */

/* What separates filename components?  */
#ifndef DIR_SEP
# define DIR_SEP '/'
# define DIR_SEP_STRING "/"
#endif

#ifndef IS_DIR_SEP
#define IS_DIR_SEP(ch) ((ch) == DIR_SEP)
#endif

/* basic types */

typedef int boolean;

typedef char *string;
typedef const char *const_string;
typedef void *address;

#ifdef __cplusplus
extern "C" {
#endif

/* the main utility routines */

extern string concat (const_string s1, const_string s2);
extern string concat3 (const_string, const_string, const_string);
extern string concatn (const_string str1, ...);
extern string xstrdup (const_string s);
extern const_string xbasename (const_string name);
extern const_string find_suffix (const_string name);
extern void xputenv (const_string var, const_string value);
extern string xgetcwd (void);
extern FILE *xfopen (const_string filename, const_string mode);
extern void xfclose (FILE *fp, const_string filename);
extern address xmalloc (size_t size);
extern address xrealloc (address old_address, size_t new_size);
extern address xcalloc (size_t nelem, size_t elsize);
extern boolean kpse_absolute_p (const_string filename, boolean relative_ok);
extern string read_line (FILE *f);
extern string kpse_program_basename (const_string argv0);
extern const_string extend_filename (const_string name, const_string suffix);
extern string uppercasify (const_string s);

#ifdef __cplusplus
}
#endif

#endif /* not TIDY_KPATHUTIL_PUBLIC_H */
