/* tectonic/internals.h: global, private header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_INTERNALS_H
#define TECTONIC_INTERNALS_H

#include <tectonic/tectonic.h>

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
#include <stdbool.h>
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

#define _DARWIN_USE_64_BIT_INODE 1
#define HAVE_ACCESS 1
#define HAVE_ASSERT_H 1
#define HAVE_ATOI 1
#define HAVE__BOOL 1
#define HAVE_DECL_ISASCII 1
#define HAVE_DECL_STRNDUP 1
#define HAVE_DIRENT_H 1
#define HAVE_DLFCN_H 1
#define HAVE_ERRNO_H 1
#define HAVE_FLOAT_H 1
#define HAVE_FMAX 1
#define HAVE_FSEEKO 1
#define HAVE_FTIME 1
#define HAVE_GETCWD 1
#define HAVE_GETTIMEOFDAY 1
#define HAVE_GETWD 1
#define HAVE_INTTYPES_H 1
#define HAVE_IOSTREAM 1
#define HAVE_LANGINFO_H 1
#define HAVE_LIBFONTCONFIG 1
#define HAVE_LIMITS_H 1
#define HAVE_LOCALE_H 1
#define HAVE_LONG_DOUBLE 1
#define HAVE_MEMCMP 1
#define HAVE_MEMCPY 1
#define HAVE_MEMORY_H 1
#define HAVE_MKDTEMP 1
#define HAVE_MKSTEMP 1
#define HAVE_MKTEMP 1
#define HAVE_OBJECT_INITCMD_CONST_CHARP 1
#define HAVE_PUTENV 1
#define HAVE_PWD_H 1
#define HAVE_SETLOCALE 1
#define HAVE_STDBOOL_H 1
#define HAVE_STDINT_H 1
#define HAVE_STDLIB_H 1
#define HAVE_STRCHR 1
#define HAVE_STRERROR 1
#define HAVE_STRING_H 1
#define HAVE_STRINGS_H 1
#define HAVE_STRNDUP 1
#define HAVE_STRRCHR 1
#define HAVE_STRUCT_STAT_ST_MTIM 1
#define HAVE_SYS_PARAM_H 1
#define HAVE_SYS_STAT_H 1
#define HAVE_SYS_TIMEB_H 1
#define HAVE_SYS_TIME_H 1
#define HAVE_SYS_TYPES_H 1
#define HAVE_SYS_WAIT_H 1
#define HAVE_TIME_H 1
#define HAVE_UINTPTR_T 1
#define HAVE_UNISTD_H 1
#define RETSIGTYPE void
#define STDC_HEADERS 1
#define ZLIB_CONST 1

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

typedef void* voidpointer;
typedef voidpointer void_pointer;

/* affine transforms */

typedef struct {
	double	a;
	double	b;
	double	c;
	double	d;
	double	x;
	double	y;
} transform;

typedef struct {
	float	x;
	float	y;
} real_point;

typedef struct {
	float	x;
	float	y;
	float	wd;
	float	ht;
} real_rect;

/* Unicode files */

typedef struct {
    FILE *f;
    long savedChar;
    short skipNextLF;
    short encodingMode;
    void *conversionData;
} UFILE;

typedef UFILE *unicodefile;
typedef unicodefile unicode_file;

/* TODO: eliminate these */
#define xCoord(p) (p).x
#define yCoord(p) (p).y
#define wdField(r) (r).wd
#define htField(r) (r).ht
#define aField(t) (t).a
#define bField(t) (t).b
#define cField(t) (t).c
#define dField(t) (t).d
#define xField(t) (t).x
#define yField(t) (t).y
#define setPoint(P,X,Y) do { (P).x = X; (P).y = Y; } while (0)

BEGIN_EXTERN_C

/*
  Functions
*/

/* openclose.c */
extern boolean open_input (FILE **, int, const_string fopen_mode);
extern boolean open_output (FILE **, const_string fopen_mode);
extern void close_file (FILE *);

/* pdfimage.c */
extern int pdf_get_rect(char* filename, int page_num, int pdf_box, real_rect* box);
extern int pdf_count_pages(char* filename);

/* tidy_kpathutil.c */
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
extern void xfseek (FILE *fp, long offset, int wherefrom, const_string filename);
extern address xmalloc (size_t size);
extern address xrealloc (address old_address, size_t new_size);
extern address xcalloc (size_t nelem, size_t elsize);
extern boolean kpse_absolute_p (const_string filename, boolean relative_ok);
extern string read_line (FILE *f);
extern const_string extend_filename (const_string name, const_string suffix);
extern string uppercasify (const_string s);
extern integer zround (double);
extern void make_identity(transform* t);
extern void make_scale(transform* t, double xscale, double yscale);
extern void make_translation(transform* t, double dx, double dy);
extern void make_rotation(transform* t, double a);
extern void transform_point(real_point* p, const transform* t);
extern void transform_concat(transform* t1, const transform* t2);
extern unsigned char get_unsigned_byte (FILE *);
extern unsigned short get_unsigned_pair (FILE *);

/* xetexini.c */
extern void main_body (string input_file_name);

/* 
   State variables 
*/

/* openclose.c */
extern string fullnameoffile;

/* tidy_kpathutil.c */
extern const char *version_string;

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
