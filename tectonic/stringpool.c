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



integer length(str_number s)
{
    register integer Result;
    if ((s >= 65536L))
        Result = str_start[(s + 1) - 65536L] - str_start[(s) - 65536L];
    else if ((s >= 32) && (s < 127))
        Result = 1;
    else if ((s <= 127))
        Result = 3;
    else if ((s < 256))
        Result = 4;
    else
        Result = 8;
    return Result;
}

str_number make_string(void)
{
    register str_number Result;
    if (str_ptr == max_strings)
        overflow(S(number_of_strings), max_strings - init_str_ptr);
    str_ptr++;
    str_start[(str_ptr) - 65536L] = pool_ptr;
    Result = str_ptr - 1;
    return Result;
}

void append_str(str_number s)
{
    integer i;
    pool_pointer j;
    i = length(s);
    {
        if (pool_ptr + i > pool_size)
            overflow(S(pool_size), pool_size - init_pool_ptr);
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

bool str_eq_buf(str_number s, integer k)
{
    pool_pointer j;
    bool result;
    j = str_start[(s) - 65536L];
    while (j < str_start[(s + 1) - 65536L]) {

        if (buffer[k] >= 65536L) {

            if (str_pool[j] != 55296L + (buffer[k] - 65536L) / 1024) {
                result = false;
                goto lab45;
            } else if (str_pool[j + 1] != 56320L + (buffer[k] - 65536L) % 1024) {
                result = false;
                goto lab45;
            } else
                j++;
        } else if (str_pool[j] != buffer[k]) {
            result = false;
            goto lab45;
        }
        j++;
        k++;
    }
    result = true;
 lab45:                        /*not_found */ return result;
}

bool str_eq_str(str_number s, str_number t)
{
    pool_pointer j, k;
    bool result;
    result = false;
    if (length(s) != length(t))
        goto lab45;
    if ((length(s) == 1)) {
        if (s < 65536L) {
            if (t < 65536L) {
                if (s != t)
                    goto lab45;
            } else {

                if (s != str_pool[str_start[(t) - 65536L]])
                    goto lab45;
            }
        } else {

            if (t < 65536L) {
                if (str_pool[str_start[(s) - 65536L]] != t)
                    goto lab45;
            } else {

                if (str_pool[str_start[(s) - 65536L]] != str_pool[str_start[(t) - 65536L]])
                    goto lab45;
            }
        }
    } else {

        j = str_start[(s) - 65536L];
        k = str_start[(t) - 65536L];
        while (j < str_start[(s + 1) - 65536L]) {

            if (str_pool[j] != str_pool[k])
                goto lab45;
            j++;
            k++;
        }
    }
    result = true;
 lab45:                        /*not_found */ return result;
}

str_number search_string(str_number search)
{
    str_number result;
    str_number s;
    integer len;
    result = 0;
    len = length(search);
    if (len == 0) {
        result = S();
        goto lab40;
    } else {

        s = search - 1;
        while (s > 65535L) {

            if (length(s) == len) {

                if (str_eq_str(s, search)) {
                    result = s;
                    goto lab40;
                }
            }
            s--;
        }
    }
 lab40:                        /*found */ return result;
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
            pool_ptr = str_start[(str_ptr) - 65536L];
        }
        return s;
    }
    return t;
}
