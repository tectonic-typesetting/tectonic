/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016-2020 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEX_CORE_H
#define TECTONIC_XETEX_CORE_H

#include "tectonic_bridge_core.h"

/* ICU */
#include <unicode/utypes.h>
#include <unicode/platform.h> // defines U_IS_BIG_ENDIAN for us

/* fontconfig */
#ifndef XETEX_MAC
#include <fontconfig/fontconfig.h>
#endif

/* harfbuzz */
#include <harfbuzz/hb.h>
#include <harfbuzz/hb-ot.h>

/* Endianness foo */

#ifdef WORDS_BIGENDIAN
#define US_NATIVE_UTF16 UTF16BE
#define UTF16_NATIVE kForm_UTF16BE
#define NATIVE_UTF32 kForm_UTF32BE
#define UCNV_UTF32_NativeEndian UCNV_UTF32_BigEndian
#else
#define US_NATIVE_UTF16 UTF16LE
#define UTF16_NATIVE kForm_UTF16LE
#define NATIVE_UTF32 kForm_UTF32LE
#define UCNV_UTF32_NativeEndian UCNV_UTF32_LittleEndian
#endif

/* our typedefs */

typedef int32_t scaled_t;

typedef enum {
    SELECTOR_FILE_0 = 0,
    SELECTOR_FILE_15 = 15,
    SELECTOR_NO_PRINT = 16,
    SELECTOR_TERM_ONLY = 17,
    SELECTOR_LOG_ONLY = 18,
    SELECTOR_TERM_AND_LOG = 19,
    SELECTOR_PSEUDO = 20,
    SELECTOR_NEW_STRING = 21
} selector_t;

/* Misc */

#define FONT_FLAGS_COLORED  0x01
#define FONT_FLAGS_VERTICAL 0x02

#endif /* not TECTONIC_XETEX_CORE_H */
