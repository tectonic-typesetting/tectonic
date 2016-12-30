/* tectonic/internals.h: global, private header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_INTERNALS_H
#define TECTONIC_INTERNALS_H

#include <tectonic/tectonic.h>
#include <tectonic/stubs.h>

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

#define DIR_SEP '/'
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
    rust_input_handle_t handle;
    long savedChar;
    short skipNextLF;
    short encodingMode;
    void *conversionData;
} UFILE;

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

/* Unicode constants */

extern const uint32_t offsetsFromUTF8[6];
extern const uint8_t bytesFromUTF8[256];
extern const uint8_t firstByteMark[7];

extern const int halfShift;
extern const uint32_t halfBase;
extern const uint32_t halfMask;
extern const uint32_t kSurrogateHighStart;
extern const uint32_t kSurrogateHighEnd;
extern const uint32_t kSurrogateLowStart;
extern const uint32_t kSurrogateLowEnd;
extern const uint32_t byteMask;
extern const uint32_t byteMark;

/*
  Functions
*/

/* errors.c */
extern int _tt_setjmp ();
extern NORETURN PRINTF_FUNC(1,2) int _tt_abort (const_string format, ...);

/* io.c */
extern boolean open_input (FILE **, int, const_string fopen_mode);
extern rust_input_handle_t tt_open_input (int filefmt);
extern void close_file (FILE *);
extern void set_input_file_encoding(UFILE *f, integer mode, integer encodingData);
extern void u_close(UFILE *f);
extern int u_open_in(UFILE **f, integer filefmt, const char* fopen_mode, integer mode, integer encodingData);
extern int get_uni_c(UFILE* f);
extern int input_line(UFILE* f);
extern void make_utf16_name(void);

/* pdfimage.c */
extern int pdf_get_rect(rust_input_handle_t file, int page_num, int pdf_box, real_rect* box);
extern int pdf_count_pages(rust_input_handle_t file);

/* tidy_kpathutil.c */
extern string xstrdup (const_string s);
extern void *xmalloc (size_t size);
extern void *xrealloc (void *old_address, size_t new_size);
extern void *xcalloc (size_t nelem, size_t elsize);
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

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
