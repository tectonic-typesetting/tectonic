/* tectonic/core-memory.h: basic dynamic memory helpers
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_CORE_MEMORY_H
#define TECTONIC_CORE_MEMORY_H

#include "core-foundation.h"

BEGIN_EXTERN_C

char *xstrdup (const char *s);
void *xmalloc (size_t size);
void *xrealloc (void *old_address, size_t new_size);
void *xcalloc (size_t nelem, size_t elsize);

static inline void *mfree(void *ptr) {
    free(ptr);
    return NULL;
}

END_EXTERN_C

#endif /* not TECTONIC_CORE_MEMORY_H */
