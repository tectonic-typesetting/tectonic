/* tectonic/xetex-scaledmath.c: low-level math functions
   Copyright 2017 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex_bindings.h"

int32_t
m_log(int32_t x)
{
    int32_t y, z;
    int32_t k;

    if (x <= 0) { /*125: */
        error_here_with_diagnostic("Logarithm of ");
        print_scaled(x);
        print_cstr(" has been replaced by 0");
        capture_to_diagnostic(NULL);
        help_ptr = 2;
        help_line[1] = "Since I don't take logs of non-positive numbers,";
        help_line[0] = "I'm zeroing this one. Proceed, with fingers crossed.";
        error();
        return 0;
    } else {
        y = 1302456860L;
        z = 6581195L;

        while (x < 0x40000000) {
            x = x + x;
            y = y - 93032639L;
            z = z - 48782L;
        }

        y = y + (z / 65536L);
        k = 2;

        while (x > 0x40000004) { /*124: */
            z = ((x - 1) / two_to_the[k]) + 1;

            while (x < 0x40000000 + z) {
                z = (z + 1) / 2;
                k = k + 1;
            }

            y = y + spec_log[k];
            x = x - z;
        }

        return y / 8;
    }
}
