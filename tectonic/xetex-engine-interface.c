/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "xetex-ext.h"

#include <string.h>

int tt_xetex_set_int_variable (const char *var_name, int value);
int tt_xetex_set_string_variable (const char *var_name, const char *value);

/* These functions aren't used within the C/C++ library, but are called
 * by the Rust code to configure the XeTeX engine before launching it. */

int
tt_xetex_set_int_variable (const char *var_name, int value)
{
    if (streq_ptr(var_name, "halt_on_error_p"))
        halt_on_error_p = value;
    else if (streq_ptr(var_name, "in_initex_mode"))
        in_initex_mode = (value != 0);
    else if (streq_ptr(var_name, "synctex_enabled"))
        synctex_enabled = (value != 0);
    else if (streq_ptr(var_name, "semantic_pagination_enabled"))
        semantic_pagination_enabled = (value != 0);
    else if (streq_ptr(var_name, "shell_escape_enabled"))
        shell_escape_enabled = (value != 0);
    else
        return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}


int
tt_xetex_set_string_variable (const char *var_name, const char *value)
{
    /* Currently unused; see Git history for how we used to set output_comment */
    return 1;
}
