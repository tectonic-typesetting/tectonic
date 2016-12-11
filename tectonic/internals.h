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

/* other lame #defines */

#define STREQ(s1, s2) (((s1) != NULL) && ((s2) != NULL) && (strcmp (s1, s2) == 0))

#ifndef isblank
#define isblank(c) ((c) == ' ' || (c) == '\t')
#endif
#define ISBLANK(c) (isascii (c) && isblank ((unsigned char)c))

/* c-pathch.h */

#define DIR_SEP '/'
#define DIR_SEP_STRING "/"
#define IS_DIR_SEP(ch) ((ch) == DIR_SEP)

/* affine transforms */

typedef struct {
    double a;
    double b;
    double c;
    double d;
    double x;
    double y;
} transform;

typedef struct {
    float x;
    float y;
} real_point;

typedef struct {
    float x;
    float y;
    float wd;
    float ht;
} real_rect;

#define SET_POINT(P,X,Y) do { (P).x = (X); (P).y = (Y); } while (0)

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

typedef enum {
    SELECTOR_FILE_0 = 0,
    SELECTOR_FILE_15 = 15,
    SELECTOR_NO_PRINT = 16,
    SELECTOR_TERM_ONLY = 17,
    SELECTOR_LOG_ONLY = 18,
    SELECTOR_TERM_AND_LOG = 19,
    SELECTOR_PSEUDO = 20,
    SELECTOR_NEW_STRING = 21
} selector_t;


BEGIN_EXTERN_C

/*
  Functions
*/

/* errors.c */
extern int _tt_setjmp ();
extern NORETURN PRINTF_FUNC(1,2) int _tt_abort (const_string format, ...);

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
extern string xgetcwd (void);
extern FILE *xfopen (const_string filename, const_string mode);
extern void xfclose (FILE *fp, const_string filename);
extern void xfseek (FILE *fp, long offset, int wherefrom, const_string filename);
extern void *xmalloc (size_t size);
extern void *xrealloc (void *old_address, size_t new_size);
extern void *xcalloc (size_t nelem, size_t elsize);
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


/*
   State variables
*/

/* openclose.c */
extern string fullnameoffile;

/* tidy_kpathutil.c */
extern const char *version_string;

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
