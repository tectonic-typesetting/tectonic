/* engine-interface.c: programmatic interface to control the engine behavior
   Copyright 2016 The Tectonic Project
   Licensed under the MIT License.
*/

#include <tectonic/tectonic.h>
#include <tectonic/internals.h>
#include <tectonic/xetexd.h>
#include <tectonic/XeTeX_ext.h>

#include <string.h>

int
tt_set_int_variable (char *var_name, int value)
{
    if (STREQ (var_name, "no_pdf_output"))
	no_pdf_output = value;
    else
	return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}


int
tt_set_string_variable (char *var_name, char *value)
{
    if (STREQ (var_name, "output_comment")) {
	size_t len = strlen (value);

	if (len < 256) {
	    output_comment = xstrdup (value);
	} else {
	    output_comment = xmalloc (256);
	    strncpy (output_comment, value, 255);
	    output_comment[255] = '\0';
	}
    } else
	return 1; /* Uh oh: unrecognized variable */

    return 0; /* success */
}
