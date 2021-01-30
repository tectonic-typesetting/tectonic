/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012-2015 by Khaled Hosny

 SIL Author(s): Jonathan Kew

Permission is hereby granted, free of charge, to any person obtaining
a copy of this software and associated documentation files (the
"Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish,
distribute, sublicense, and/or sell copies of the Software, and to
permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE COPYRIGHT HOLDERS BE LIABLE
FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF
CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of the copyright holders
shall not be used in advertising or otherwise to promote the sale,
use or other dealings in this Software without prior written
authorization from the copyright holders.
\****************************************************************************/

/* Formerly known as [xetex-]XeTeXLayoutInterface.h */

#ifndef XETEX_LAYOUT_INTERFACE_H
#define XETEX_LAYOUT_INTERFACE_H 1

#include "tectonic_bridge_core.h"

/* harfbuzz: hb_tag_t and hb_font_t used below */
#include <harfbuzz/hb.h>


/* Set up our types */

#ifdef XETEX_MAC

#include <ApplicationServices/ApplicationServices.h>
typedef CTFontDescriptorRef PlatformFontRef;

#else /* XETEX_MAC */

#include <fontconfig/fontconfig.h>
typedef FcPattern* PlatformFontRef;
typedef int32_t Fixed; /* macOS defines Fixed in system headers */

#endif /* XETEX_MAC */

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

typedef uint32_t OTTag;
typedef uint16_t GlyphID;

typedef struct XeTeXFont_rec* XeTeXFont;
typedef struct XeTeXLayoutEngine_rec* XeTeXLayoutEngine;

/* Now we can defined our C APIs */

BEGIN_EXTERN_C

extern Fixed loaded_font_design_size;

#define LEFT_SIDE 0
#define RIGHT_SIDE 1

void set_cp_code(int fontNum, unsigned int code, int side, int value);
int get_cp_code(int fontNum, unsigned int code, int side);

int getCachedGlyphBBox(uint16_t fontID, uint16_t glyphID, GlyphBBox* bbox);
void cacheGlyphBBox(uint16_t fontID, uint16_t glyphID, const GlyphBBox* bbox);

void terminate_font_manager(void);
void destroy_font_manager(void);

XeTeXFont createFont(PlatformFontRef fontRef, Fixed pointSize);
XeTeXFont createFontFromFile(const char* filename, int index, Fixed pointSize);

void setFontLayoutDir(XeTeXFont font, int vertical);

PlatformFontRef findFontByName(const char* name, char* var, double size);

char getReqEngine(void);
void setReqEngine(char reqEngine);
const char* getFullName(PlatformFontRef fontRef);

char* getFontFilename(XeTeXLayoutEngine engine, uint32_t* index);

double getDesignSize(XeTeXFont font);

void deleteFont(XeTeXFont font);

void* getFontTablePtr(XeTeXFont font, uint32_t tableTag);

Fixed getSlant(XeTeXFont font);

unsigned int countScripts(XeTeXFont font);
unsigned int countLanguages(XeTeXFont font, hb_tag_t script);
unsigned int countFeatures(XeTeXFont font, hb_tag_t script, hb_tag_t language);
unsigned int countGlyphs(XeTeXFont font);

hb_tag_t getIndScript(XeTeXFont font, unsigned int index);
hb_tag_t getIndLanguage(XeTeXFont font, hb_tag_t script, unsigned int index);
hb_tag_t getIndFeature(XeTeXFont font, hb_tag_t script, hb_tag_t language, unsigned int index);

float getGlyphWidth(XeTeXFont font, uint32_t gid);

XeTeXLayoutEngine createLayoutEngine(PlatformFontRef fontRef, XeTeXFont font, hb_tag_t script, char *language,
                        hb_feature_t* features, int nFeatures, char **shapers, uint32_t rgbValue,
                        float extend, float slant, float embolden);

void deleteLayoutEngine(XeTeXLayoutEngine engine);

XeTeXFont getFont(XeTeXLayoutEngine engine);
PlatformFontRef getFontRef(XeTeXLayoutEngine engine);

float getExtendFactor(XeTeXLayoutEngine engine);
float getSlantFactor(XeTeXLayoutEngine engine);
float getEmboldenFactor(XeTeXLayoutEngine engine);

int layoutChars(XeTeXLayoutEngine engine, uint16_t* chars, int32_t offset, int32_t count, int32_t max,
                        bool rightToLeft);

void getGlyphs(XeTeXLayoutEngine engine, uint32_t* glyphs);
void getGlyphAdvances(XeTeXLayoutEngine engine, float *advances);
void getGlyphPositions(XeTeXLayoutEngine engine, FloatPoint* positions);

float getPointSize(XeTeXLayoutEngine engine);

void getAscentAndDescent(XeTeXLayoutEngine engine, float* ascent, float* descent);
void getCapAndXHeight(XeTeXLayoutEngine engine, float* capheight, float* xheight);

int getDefaultDirection(XeTeXLayoutEngine engine);

uint32_t getRgbValue(XeTeXLayoutEngine engine);

void getGlyphBounds(XeTeXLayoutEngine engine, uint32_t glyphID, GlyphBBox* bbox);

float getGlyphWidthFromEngine(XeTeXLayoutEngine engine, uint32_t glyphID);

void getGlyphHeightDepth(XeTeXLayoutEngine engine, uint32_t glyphID, float* height, float* depth);

void getGlyphSidebearings(XeTeXLayoutEngine engine, uint32_t glyphID, float* lsb, float* rsb);

float getGlyphItalCorr(XeTeXLayoutEngine engine, uint32_t glyphID);

uint32_t mapCharToGlyph(XeTeXLayoutEngine engine, uint32_t charCode);

int mapGlyphToIndex(XeTeXLayoutEngine engine, const char* glyphName);

const char* getGlyphName(XeTeXFont font, uint16_t gid, int* len);

int getFontCharRange(XeTeXLayoutEngine engine, int reqFirst);

/* graphite interface functions... */
bool initGraphiteBreaking(XeTeXLayoutEngine engine, const uint16_t* txtPtr, int txtLen);
int findNextGraphiteBreak(void);

bool usingOpenType(XeTeXLayoutEngine engine);
bool usingGraphite(XeTeXLayoutEngine engine);
bool isOpenTypeMathFont(XeTeXLayoutEngine engine);

bool findGraphiteFeature(XeTeXLayoutEngine engine, const char* s, const char* e, hb_tag_t* f, int* v);

uint32_t countGraphiteFeatures(XeTeXLayoutEngine engine);
uint32_t getGraphiteFeatureCode(XeTeXLayoutEngine engine, uint32_t index);
uint32_t countGraphiteFeatureSettings(XeTeXLayoutEngine engine, uint32_t feature);
uint32_t getGraphiteFeatureSettingCode(XeTeXLayoutEngine engine, uint32_t feature, uint32_t index);
uint32_t getGraphiteFeatureDefaultSetting(XeTeXLayoutEngine engine, uint32_t feature);
char* getGraphiteFeatureLabel(XeTeXLayoutEngine engine, uint32_t feature);
char* getGraphiteFeatureSettingLabel(XeTeXLayoutEngine engine, uint32_t feature, uint32_t setting);
long findGraphiteFeatureNamed(XeTeXLayoutEngine engine, const char* name, int namelength);
long findGraphiteFeatureSettingNamed(XeTeXLayoutEngine engine, uint32_t feature, const char* name, int namelength);

/* Extra APIs needed to encapsulate across the crate boundaries */
hb_font_t *ttxl_get_hb_font(XeTeXLayoutEngine engine);
float ttxl_font_units_to_points(XeTeXFont font, float units);
float ttxl_font_points_to_units(XeTeXFont font, float points);
const char *ttxl_platfont_get_desc(PlatformFontRef fontRef);

#ifdef XETEX_MAC
char* getFileNameFromCTFont(CTFontRef ctFontRef, uint32_t *index);
#endif

END_EXTERN_C

#endif /* XETEX_LAYOUT_INTERFACE_H */
