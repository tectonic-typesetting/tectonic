/* tectonic/xetex-stringpool.c: preloaded "string pool" constants
   Copyright 2017-2018 the Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include <string.h>

static const char *string_constants[] = {
    // this catches code that relies on magic stringpool constants
    "this marks the start of the stringpool",
    "",
    NULL
};

int
load_pool_strings(int32_t spare_size)
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

        while (len-- > 0) {
            set_str_pool(pool_ptr(), *s++);
            set_pool_ptr(pool_ptr() + 1);
        }

        g = make_string(); /* Returns 0 on error. */
    }

    return g;
}

void
append_str(str_number s)
{
    int32_t i;
    pool_pointer j;

    i = length(s);

    if (pool_ptr() + i > pool_size())
        overflow("pool size", pool_size() - init_pool_ptr);

    j = str_start(s - 65536L);

    while (i > 0) {
        set_str_pool(pool_ptr(), str_pool(j));
        set_pool_ptr(pool_ptr()+1);
        j++;
        i--;
    }
}


bool
str_eq_buf(str_number s, int32_t k)
{
    pool_pointer j;

    j = str_start(s - 65536L);

    while (j < str_start(s + 1 - 65536L)) {
        if (buffer[k] >= 65536L) {
            if (str_pool(j) != 55296L + (buffer[k] - 65536L) / 1024) {
                return false;
            } else if (str_pool(j + 1) != 56320L + (buffer[k] - 65536L) % 1024) {
                return false;
            } else {
                j++;
            }
        } else if (str_pool(j) != buffer[k]) {
            return false;
        }

        j++;
        k++;
    }

    return true;
}

str_number
slow_make_string(void)
{
    str_number s;
    str_number t;

    t = make_string();
    s = search_string(t);

    if (s > 0) {
        set_str_ptr(str_ptr()-1);
        set_pool_ptr(str_start(str_ptr() - TOO_BIG_CHAR));
        return s;
    }

    return t;
}
