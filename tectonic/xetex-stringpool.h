/* tectonic/xetex-stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_STRINGPOOL_H
#define TECTONIC_STRINGPOOL_H

#include "xetex-core.h"
#include "xetex-xetexd.h"

#define EMPTY_STRING (65536L + 1)

BEGIN_EXTERN_C

int load_pool_strings(int32_t spare_size);
int32_t length(str_number s);
str_number make_string(void);
void append_str(str_number s);
bool str_eq_buf(str_number s, int32_t k);
bool str_eq_str(str_number s, str_number t);
str_number search_string(str_number search);
str_number slow_make_string(void);

END_EXTERN_C

#endif /* not TECTONIC_STRINGPOOL_H */
