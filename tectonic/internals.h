/* tectonic/internals.h: global, private header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_INTERNALS_H
#define TECTONIC_INTERNALS_H

#include <tectonic/tectonic.h>
#include <tectonic/core-bridge.h>

#if defined(_MSC_VER)
#define _USE_MATH_DEFINES
#endif

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
#if HAVE_UNISTD_H
#include <unistd.h>
#endif

/* high-level defines */

#define _DARWIN_USE_64_BIT_INODE 1

/* other lame #defines */

#define STREQ(s1, s2) (((s1) != NULL) && ((s2) != NULL) && (strcmp (s1, s2) == 0))

#ifndef isblank
#define isblank(c) ((c) == ' ' || (c) == '\t')
#endif
#define ISBLANK(c) (isascii (c) && isblank ((unsigned char)c))

#define DIR_SEP '/'
#define IS_DIR_SEP(ch) ((ch) == DIR_SEP)

/* Core typedefs. */

typedef integer scaled;

/* affine transforms */

typedef struct {
    double a;
    double b;
    double c;
    double d;
    double x;
    double y;
} transform_t;

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

/* io.c */
rust_input_handle_t tt_open_input (int filefmt);
void set_input_file_encoding(UFILE *f, integer mode, integer encodingData);
void u_close(UFILE *f);
int u_open_in(UFILE **f, integer filefmt, const char* fopen_mode, integer mode, integer encodingData);
int get_uni_c(UFILE* f);
bool input_line(UFILE* f);
void make_utf16_name(void);

/* mathutil.c */
integer tex_round (double);
integer half(integer x);
scaled mult_and_add(integer n, scaled x, scaled y, scaled max_answer);
scaled x_over_n(scaled x, integer n);
scaled xn_over_d(scaled x, integer n, integer d);

/* core-kpathutil.c */
char *xstrdup (const char *s);
void *xmalloc (size_t size);
void *xrealloc (void *old_address, size_t new_size);
void *xcalloc (size_t nelem, size_t elsize);
void make_identity(transform_t* t);
void make_scale(transform_t* t, double xscale, double yscale);
void make_translation(transform_t* t, double dx, double dy);
void make_rotation(transform_t* t, double a);
void transform_point(real_point* p, const transform_t* t);
void transform_concat(transform_t* t1, const transform_t* t2);


/*
   State variables
*/

/* openclose.c */
extern char *fullnameoffile;

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
