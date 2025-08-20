/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-synctex.h"
#include "tectonic_bridge_core.h"
#include "xetex_bindings.h"

void
print_roman_int(int32_t n)
{
    int32_t u, v;

    const char* roman_data = "m2d5c2l5x2v5i";
    unsigned char j = 0;
    unsigned char k = 0;
    v = 1000;

    while (true) {
        while (n >= v) {
            print_char(roman_data[j]);
            n = n - v;
        }

        if (n <= 0)
            return;

        k = j + 2;
        u = v / (roman_data[k - 1] - '0');
        if (roman_data[k - 1] == '2' ) {
            k = k + 2;
            u = u / (roman_data[k - 1] - '0');
        }

        if (n + u >= v) {
            print_char(roman_data[k]);
            n = n + u;
        } else {
            j = j + 2;
            v = v / (roman_data[j - 1] - '0');
        }
    }
}


void
print_current_string(void)
{
    pool_pointer j = str_start(str_ptr() - 0x10000);

    while (j < pool_ptr()) {
        print_char(str_pool(j));
        j++;
    }
}
