/* tectonic/tectonic.h: global, public header for Tectonic
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_TECTONIC_H
#define TECTONIC_TECTONIC_H

#include "core-foundation.h"

/* TeX-specific enums */

typedef enum {
    HISTORY_SPOTLESS = 0,
    HISTORY_WARNING_ISSUED = 1,
    HISTORY_ERROR_ISSUED = 2,
    HISTORY_FATAL_ERROR = 3
} tt_history_t;

#endif /* not TECTONIC_TECTONIC_H */
