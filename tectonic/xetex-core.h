/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016-2020 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEX_CORE_H
#define TECTONIC_XETEX_CORE_H

#include "core-foundation.h"
#include "core-bridge.h"
#include "core-memory.h"
#include "core-strutils.h"

/* ICU */
#include <unicode/utypes.h>
#include <unicode/platform.h> // defines U_IS_BIG_ENDIAN for us

/* fontconfig */
#ifndef XETEX_MAC
#include <fontconfig/fontconfig.h>
#endif

/* freetype */
#include <ft2build.h>
#include FT_FREETYPE_H
#include FT_TRUETYPE_TABLES_H

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

typedef uint32_t OTTag;
typedef uint16_t GlyphID;

#ifdef XETEX_MAC /* Macs provide Fixed and FixedPoint */
# include <CoreFoundation/CoreFoundation.h>
# include <ApplicationServices/ApplicationServices.h>
#else
typedef scaled_t Fixed;

typedef struct {
    Fixed x;
    Fixed y;
} FixedPoint;

typedef void* CFDictionaryRef; /* dummy declaration just so the stubs can compile */
#endif

typedef struct {
    float x;
    float y;
} FloatPoint;

typedef struct {
    float xMin;
    float yMin;
    float xMax;
    float yMax;
} GlyphBBox;

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

#ifdef XETEX_MAC
#include <ApplicationServices/ApplicationServices.h>
typedef CTFontDescriptorRef PlatformFontRef;
#else
typedef FcPattern* PlatformFontRef;
#endif

typedef struct XeTeXFont_rec* XeTeXFont;
typedef struct XeTeXLayoutEngine_rec* XeTeXLayoutEngine;

/* Misc */

#define FONT_FLAGS_COLORED  0x01
#define FONT_FLAGS_VERTICAL 0x02

/* gFreeTypeLibrary is defined in xetex-XeTeXFontInst_FT2.cpp,
 * also used in xetex-XeTeXFontMgr_FC.cpp and xetex-ext.c.  */

BEGIN_EXTERN_C
extern FT_Library gFreeTypeLibrary;
END_EXTERN_C

#endif /* not TECTONIC_XETEX_CORE_H */
