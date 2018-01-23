/* tectonic/stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_STRINGPOOL_H
#define TECTONIC_STRINGPOOL_H

#include "tectonic.h"

#define EMPTY_STRING (65536L + 1)

BEGIN_EXTERN_C

int load_pool_strings(integer spare_size);

END_EXTERN_C

#endif /* not TECTONIC_STRINGPOOL_H */
