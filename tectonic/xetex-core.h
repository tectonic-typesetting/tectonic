/* tectonic/xetex-core.h: core XeTeX types and #includes.
   Copyright 2016 the Tectonic Project
   Licensed under the MIT License.
*/

#ifndef TECTONIC_XETEX_CORE_H
#define TECTONIC_XETEX_CORE_H

#include "tectonic.h"
#include "internals.h"

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
#include <hb.h>
#include <hb-ot.h>

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

typedef uint32_t OTTag;
typedef uint16_t GlyphID;

#ifdef XETEX_MAC /* Macs provide Fixed and FixedPoint */
# include <CoreFoundation/CoreFoundation.h>
# include <ApplicationServices/ApplicationServices.h>
#else
typedef int32_t Fixed;

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

#ifdef XETEX_MAC
#include <ApplicationServices/ApplicationServices.h>
typedef CTFontDescriptorRef PlatformFontRef;
#else
#include <ft2build.h>
#include FT_FREETYPE_H
typedef FcPattern* PlatformFontRef;
#endif

typedef struct XeTeXFont_rec* XeTeXFont;
typedef struct XeTeXLayoutEngine_rec* XeTeXLayoutEngine;

/* Misc */

#define FONT_FLAGS_COLORED  0x01
#define FONT_FLAGS_VERTICAL 0x02

/* gFreeTypeLibrary is defined in XeTeXFontInst_FT2.cpp,
 * also used in XeTeXFontMgr_FC.cpp and XeTeX_ext.c.  */

BEGIN_EXTERN_C
extern FT_Library gFreeTypeLibrary;
END_EXTERN_C

#endif /* not TECTONIC_XETEX_CORE_H */
