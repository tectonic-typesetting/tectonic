/* tectonic/internals.h: global, private header for Tectonic
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_INTERNALS_H
#define TECTONIC_INTERNALS_H

#include "tectonic.h"
#include "core-bridge.h"
#include "core-strutils.h"

#include <fcntl.h>
#include <stdio.h>
#include <sys/stat.h>
#include <sys/types.h>

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

/*
  Functions
*/

/* io.c */
rust_input_handle_t tt_open_input (int filefmt);
void set_input_file_encoding(UFILE *f, int32_t mode, int32_t encodingData);
void u_close(UFILE *f);
int u_open_in(UFILE **f, int32_t filefmt, const char* fopen_mode, int32_t mode, int32_t encodingData);
int get_uni_c(UFILE* f);
int input_line(UFILE* f);
void make_utf16_name(void);

/* core-kpathutil.c */
char *xstrdup (const char *s);
void *xmalloc (size_t size);
void *xrealloc (void *old_address, size_t new_size);
void *xcalloc (size_t nelem, size_t elsize);

static inline void *mfree(void *ptr) {
    free(ptr);
    return NULL;
}

/*
   State variables
*/

/* openclose.c */
extern char *fullnameoffile;

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
