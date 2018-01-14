/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#include "tectonic.h"
#include "internals.h"
#include "xetexd.h"
#include "XeTeX_ext.h"

#include <string.h>

int
tt_set_int_variable (char *var_name, int value)
{
    if (streq_ptr(var_name, "halt_on_error_p"))
        halt_on_error_p = value;
    else if (streq_ptr(var_name, "in_initex_mode"))
        in_initex_mode = (value != 0);
    else if (streq_ptr(var_name, "synctex_enabled"))
        synctex_enabled = (value != 0);
    else
        return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}


int
tt_set_string_variable (char *var_name, char *value)
{
    /* Currently unused; see Git history for how we used to set output_comment */
    return 1;
}
