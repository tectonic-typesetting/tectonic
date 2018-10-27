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

        while (len-- > 0)
            str_pool[pool_ptr++] = *s++;

        g = make_string(); /* Returns 0 on error. */
    }

    return g;
}



int32_t length(str_number s)
{
    if ((s >= 65536L))
        return str_start[(s + 1) - 65536L] - str_start[(s) - 65536L];
    else if ((s >= 32) && (s < 127))
        return 1;
    else if ((s <= 127))
        return 3;
    else if ((s < 256))
        return 4;
    else
        return 8;
}

str_number make_string(void)
{
    if (str_ptr == max_strings)
        overflow("number of strings", max_strings - init_str_ptr);
    str_ptr++;
    str_start[str_ptr - TOO_BIG_CHAR] = pool_ptr;
    return str_ptr - 1;
}

void append_str(str_number s)
{
    int32_t i;
    pool_pointer j;
    i = length(s);
    {
        if (pool_ptr + i > pool_size)
            overflow("pool size", pool_size - init_pool_ptr);
    }
    j = str_start[(s) - 65536L];
    while ((i > 0)) {

        {
            str_pool[pool_ptr] = str_pool[j];
            pool_ptr++;
        }
        j++;
        i--;
    }
}

bool str_eq_buf(str_number s, int32_t k)
{
    pool_pointer j;
    j = str_start[(s) - 65536L];
    while (j < str_start[(s + 1) - 65536L]) {

        if (buffer[k] >= 65536L) {

            if (str_pool[j] != 55296L + (buffer[k] - 65536L) / 1024) {
                return false;
            } else if (str_pool[j + 1] != 56320L + (buffer[k] - 65536L) % 1024) {
                return false;
            } else
                j++;
        } else if (str_pool[j] != buffer[k]) {
            return false;
        }
        j++;
        k++;
    }
    return true;
}

bool str_eq_str(str_number s, str_number t)
{
    pool_pointer j, k;
    if (length(s) != length(t))
        return false;
    if ((length(s) == 1)) {
        if (s < 65536L) {
            if (t < 65536L) {
                if (s != t)
                    return false;
            } else {

                if (s != str_pool[str_start[(t) - 65536L]])
                    return false;
            }
        } else {

            if (t < 65536L) {
                if (str_pool[str_start[(s) - 65536L]] != t)
                    return false;
            } else {

                if (str_pool[str_start[(s) - 65536L]] != str_pool[str_start[(t) - 65536L]])
                    return false;
            }
        }
    } else {

        j = str_start[(s) - 65536L];
        k = str_start[(t) - 65536L];
        while (j < str_start[(s + 1) - 65536L]) {

            if (str_pool[j] != str_pool[k])
                return false;
            j++;
            k++;
        }
    }
    return true;
}

str_number search_string(str_number search)
{
    str_number s;
    int32_t len;
    len = length(search);
    if (len == 0) {
        return EMPTY_STRING;
    } else {

        s = search - 1;
        while (s > 65535L) {

            if (length(s) == len) {

                if (str_eq_str(s, search)) {
                    return s;
                }
            }
            s--;
        }
    }
    return 0;
}

str_number slow_make_string(void)
{
    str_number s;
    str_number t;
    t = make_string();
    s = search_string(t);
    if (s > 0) {
        {
            str_ptr--;
            pool_ptr = str_start[str_ptr - TOO_BIG_CHAR];
        }
        return s;
    }
    return t;
}
