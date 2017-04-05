/* tectonic/stringpool.c: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/xetexd.h>
#include <string.h>

static const char *string_constants[] = {
#include "stringpool_generated.c"
    NULL
};


int
load_pool_strings(integer spare_size)
{
    const char *s;
    int i = 0;
    size_t total_len = 0;
    str_number g = 0;

    while ((s = string_constants[i++]) != NULL) {
        size_t len = strlen(s);

        total_len += len;
        if (total_len >= spare_size)
            return 0;

        while (len-- > 0)
            str_pool[pool_ptr++] = *s++;

        g = make_string(); /* Returns 0 on error. */
    }

    return g;
}
