/* tectonic/stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_STRINGPOOL_H
#define TECTONIC_STRINGPOOL_H

#include <tectonic/tectonic.h>

/* This includes all of the S__<string-id> definitions */
#include "stringpool_generated.h"

#define S(string_id) (65536 + S__##string_id)

BEGIN_EXTERN_C

extern int load_pool_strings(integer spare_size);

END_EXTERN_C

#endif /* not TECTONIC_STRINGPOOL_H */
