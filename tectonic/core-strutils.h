/* tectonic/core-strutils.h: miscellaneous C string utilities
   Copyright 2016-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_CORE_STRUTILS_H
#define TECTONIC_CORE_STRUTILS_H

#include "core-foundation.h"

#ifndef isblank
#define isblank(c) ((c) == ' ' || (c) == '\t')
#endif
#define ISBLANK(c) (isascii (c) && isblank ((unsigned char)c))

/* Note that we explicitly do *not* change this on Windows. For maximum
 * portability, we should probably accept *either* forward or backward slashes
 * as directory separators. */
#define IS_DIR_SEP(ch) ((ch) == '/')

static inline bool streq_ptr(const char *s1, const char *s2) {
    if (s1 && s2)
        return strcmp(s1, s2) == 0;
    return false;
}

static inline const char *strstartswith(const char *s, const char *prefix) {
    size_t length;

    length = strlen(prefix);
    if (strncmp(s, prefix, length) == 0)
        return s + length;
    return NULL;
}

BEGIN_EXTERN_C
/* Nothing here yet. */
END_EXTERN_C

#endif /* not TECTONIC_CORE_STRUTILS_H */
