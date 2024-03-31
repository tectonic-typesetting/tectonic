#ifndef XETEX_LAYOUT_BINDINGS_H
#define XETEX_LAYOUT_BINDINGS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "harfbuzz/hb.h"
#include "harfbuzz/hb-ft.h"
#ifdef XETEX_MAC
#include <ApplicationServices/ApplicationServices.h>
#else
#include <fontconfig/fontconfig.h>
#endif

typedef struct XeTeXFont_rec* XeTeXFont;
typedef struct XeTeXLayoutEngine_rec* XeTeXLayoutEngine;

#define LEFT_SIDE 0

#define RIGHT_SIDE 1

#if !defined(XETEX_MAC)
#define FONT_FAMILY_NAME 1
#endif

#if !defined(XETEX_MAC)
#define FONT_STYLE_NAME 2
#endif

#if !defined(XETEX_MAC)
#define FONT_FULL_NAME 4
#endif

#if !defined(XETEX_MAC)
#define PREFERRED_FAMILY_NAME 16
#endif

#if !defined(XETEX_MAC)
#define PREFERRED_SUBFAMILY_NAME 17
#endif

typedef struct hb_font_t hb_font_t;

typedef struct {
  float xMin;
  float yMin;
  float xMax;
  float yMax;
} GlyphBBox;

typedef struct {
  float x;
  float y;
} FloatPoint;

#if !defined(XETEX_MAC)
typedef FcPattern *RawPlatformFontRef;
#endif

#if defined(XETEX_MAC)
typedef CTFontDescriptorRef RawPlatformFontRef;
#endif

#if !defined(XETEX_MAC)
typedef int32_t Fixed;
#endif

typedef uint32_t OTTag;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

int32_t getCachedGlyphBBox(uint16_t font_id, uint16_t glyph_id, GlyphBBox *bbox);

void cacheGlyphBBox(uint16_t font_id, uint16_t glyph_id, const GlyphBBox *bbox);

void set_cp_code(int32_t font_num, uint32_t code, int32_t side, int32_t value);

int32_t get_cp_code(int32_t font_num, uint32_t code, int32_t side);

XeTeXLayoutEngine createLayoutEngine(XeTeXFont font,
                                     hb_tag_t script,
                                     char *language,
                                     hb_feature_t *features,
                                     int n_features,
                                     const char **shapers,
                                     uint32_t rgb_value,
                                     float extend,
                                     float slant,
                                     float embolden);

void deleteLayoutEngine(XeTeXLayoutEngine this_);

XeTeXFont getFont(XeTeXLayoutEngine engine);

float getExtendFactor(XeTeXLayoutEngine engine);

float getSlantFactor(XeTeXLayoutEngine engine);

float getEmboldenFactor(XeTeXLayoutEngine engine);

float getPointSize(XeTeXLayoutEngine engine);

void getAscentAndDescent(XeTeXLayoutEngine engine, float *ascent, float *descent);

void getCapAndXHeight(XeTeXLayoutEngine engine, float *capheight, float *xheight);

int getDefaultDirection(XeTeXLayoutEngine engine);

uint32_t getRgbValue(XeTeXLayoutEngine engine);

void getGlyphBounds(XeTeXLayoutEngine engine, uint32_t glyph_id, GlyphBBox *bbox);

float getGlyphWidthFromEngine(XeTeXLayoutEngine engine, uint32_t glyph_id);

void getGlyphHeightDepth(XeTeXLayoutEngine engine, uint32_t glyph_id, float *height, float *depth);

void getGlyphSidebearings(XeTeXLayoutEngine engine, uint32_t glyph_id, float *lsb, float *rsb);

float getGlyphItalCorr(XeTeXLayoutEngine engine, uint32_t glyph_id);

uint32_t mapCharToGlyph(XeTeXLayoutEngine engine, uint32_t char_code);

int getFontCharRange(XeTeXLayoutEngine engine, int req_first);

int mapGlyphToIndex(XeTeXLayoutEngine engine, const char *glyph_name);

bool usingGraphite(XeTeXLayoutEngine engine);

bool usingOpenType(XeTeXLayoutEngine engine);

bool isOpenTypeMathFont(XeTeXLayoutEngine engine);

hb_font_t *ttxl_get_hb_font(XeTeXLayoutEngine engine);

int layoutChars(XeTeXLayoutEngine engine,
                uint16_t *chars,
                int32_t offset,
                int32_t count,
                int32_t max,
                bool rtl);

const char *getFontFilename(XeTeXLayoutEngine engine, uint32_t *index);

void freeFontFilename(const char *filename);

void getGlyphs(XeTeXLayoutEngine engine, uint32_t *glyphs);

void getGlyphAdvances(XeTeXLayoutEngine engine, float *advances);

void getGlyphPositions(XeTeXLayoutEngine engine, FloatPoint *positions);

uint32_t countGraphiteFeatures(XeTeXLayoutEngine engine);

uint32_t getGraphiteFeatureCode(XeTeXLayoutEngine engine, uint32_t index);

uint32_t countGraphiteFeatureSettings(XeTeXLayoutEngine engine, uint32_t feature_id);

uint32_t getGraphiteFeatureSettingCode(XeTeXLayoutEngine engine,
                                       uint32_t feature_id,
                                       uint32_t index);

uint32_t getGraphiteFeatureDefaultSetting(XeTeXLayoutEngine engine, uint32_t feature_id);

const char *getGraphiteFeatureLabel(XeTeXLayoutEngine engine, uint32_t feature_id);

const char *getGraphiteFeatureSettingLabel(XeTeXLayoutEngine engine,
                                           uint32_t feature_id,
                                           uint32_t setting_id);

bool findGraphiteFeature(XeTeXLayoutEngine engine,
                         const char *s,
                         const char *e,
                         hb_tag_t *f,
                         int *v);

long findGraphiteFeatureNamed(XeTeXLayoutEngine engine, const char *name, int namelength);

long findGraphiteFeatureSettingNamed(XeTeXLayoutEngine engine,
                                     uint32_t id,
                                     const char *name,
                                     int namelength);

bool initGraphiteBreaking(XeTeXLayoutEngine engine, const uint16_t *txt_ptr, unsigned int txt_len);

int findNextGraphiteBreak(void);

XeTeXFont createFont(RawPlatformFontRef font_ref, Fixed point_size);

XeTeXFont createFontFromFile(const char *filename, int index, Fixed point_size);

void deleteFont(XeTeXFont font);

unsigned int countScripts(XeTeXFont font);

unsigned int countLanguages(XeTeXFont font, hb_tag_t script);

unsigned int countFeatures(XeTeXFont font, hb_tag_t script, hb_tag_t language);

bool hasFontTable(XeTeXFont font, OTTag table_tag);

Fixed getSlant(XeTeXFont font);

unsigned int countGlyphs(XeTeXFont font);

float getGlyphWidth(XeTeXFont font, uint32_t gid);

void setFontLayoutDir(XeTeXFont font, int vertical);

hb_tag_t getIndScript(XeTeXFont font, unsigned int index);

hb_tag_t getIndLanguage(XeTeXFont font, hb_tag_t script, unsigned int index);

hb_tag_t getIndFeature(XeTeXFont font, hb_tag_t script, hb_tag_t language, unsigned int index);

const char *getGlyphName(XeTeXFont font, uint16_t gid, int *len);

void freeGlyphName(char *name);

float ttxl_font_units_to_points(XeTeXFont font, float units);

float ttxl_font_points_to_units(XeTeXFont font, float points);

float ttxl_font_get_point_size(XeTeXFont font);

#if defined(XETEX_MAC)
const char *getNameFromCTFont(CTFontRef ct_font_ref, CFStringRef name_key);
#endif

#if defined(XETEX_MAC)
const char *getFileNameFromCTFont(CTFontRef ct_font_ref, uint32_t *index);
#endif

Fixed get_loaded_font_design_size(void);

void set_loaded_font_design_size(Fixed val);

void terminate_font_manager(void);

void destroy_font_manager(void);

RawPlatformFontRef findFontByName(const char *name, char *var, double size);

char getReqEngine(void);

void setReqEngine(char engine);

const char *getFullName(RawPlatformFontRef font);

double getDesignSize(XeTeXFont font);

const char *ttxl_platfont_get_desc(RawPlatformFontRef font);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* XETEX_LAYOUT_BINDINGS_H */
