/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016 The Tectonic Project
   Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/XeTeX_ext.h>


int
tt_set_int_variable (char *var_name, int value)
{
    if (STREQ (var_name, "no_pdf_output"))
	no_pdf_output = value;
    else
	return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}
