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

/* core-kpathutil.c */
char *xstrdup (const char *s);
void *xmalloc (size_t size);
void *xrealloc (void *old_address, size_t new_size);
void *xcalloc (size_t nelem, size_t elsize);

static inline void *mfree(void *ptr) {
    free(ptr);
    return NULL;
}

END_EXTERN_C

#endif /* not TECTONIC_INTERNALS_H */
