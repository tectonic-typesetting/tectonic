/* tectonic/xetex-stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_STRINGPOOL_H
#define TECTONIC_STRINGPOOL_H

#include "xetex-core.h"
#include "xetex-xetexd.h"

BEGIN_EXTERN_C

int load_pool_strings(int32_t spare_size);
void append_str(str_number s);
bool str_eq_buf(str_number s, int32_t k);

END_EXTERN_C

#endif /* not TECTONIC_STRINGPOOL_H */
