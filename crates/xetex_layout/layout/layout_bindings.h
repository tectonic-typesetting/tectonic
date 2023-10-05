#ifndef XETEX_LAYOUT_BINDINGS_H
#define XETEX_LAYOUT_BINDINGS_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "harfbuzz/hb.h"
#include "harfbuzz/hb-ft.h"
#include "fontconfig/fontconfig.h"
typedef struct XeTeXFont_rec* XeTeXFont;

#define LEFT_SIDE 0

#define RIGHT_SIDE 1

typedef int32_t Fixed;

typedef struct {
  float xMin;
  float yMin;
  float xMax;
  float yMax;
} GlyphBBox;

typedef uint32_t OTTag;

typedef FcPattern *PlatformFontRef;

typedef struct {
  void *vtable;
  unsigned short unitsPerEm;
  float pointSize;
  float ascent;
  float descent;
  float capHeight;
  float xHeight;
  float italicAngle;
  bool vertical;
  char *filename;
  uint32_t index;
  FT_Face ftFace;
  unsigned char *backingData;
  unsigned char *backingData2;
  hb_font_t *hbFont;
} XeTeXFontBase;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

double RsFix2D(Fixed f);

Fixed RsD2Fix(double d);

int32_t getCachedGlyphBBox(uint16_t font_id, uint16_t glyph_id, GlyphBBox *bbox);

void cacheGlyphBBox(uint16_t font_id, uint16_t glyph_id, const GlyphBBox *bbox);

void set_cp_code(int32_t font_num, uint32_t code, int32_t side, int32_t value);

int32_t get_cp_code(int32_t font_num, uint32_t code, int32_t side);

/**
 * Return NAME with any leading path stripped off. This returns a
 * pointer into NAME.  For example, `basename("/foo/bar.baz")`
 * returns `"bar.baz"`.
 */
const char *xbasename(const char *name);

void *getFontTablePtr(XeTeXFont font, OTTag table_tag);

Fixed getSlant(XeTeXFont font);

unsigned int countGlyphs(XeTeXFont font);

float getGlyphWidth(XeTeXFont font, uint32_t gid);

void setFontLayoutDir(XeTeXFont font, int vertical);

FT_Fixed _get_glyph_advance(FT_Face face, unsigned int gid, bool vertical);

hb_font_funcs_t *_get_font_funcs(void);

hb_blob_t *_get_table(hb_face_t*, hb_tag_t tag, void *user_data);

XeTeXFont createFont(PlatformFontRef font_ref, Fixed point_size);

XeTeXFont createFontFromFile(const char *filename, int index, Fixed point_size);

void deleteFont(XeTeXFont font);

unsigned int getLargerScriptListTable(XeTeXFont font, hb_tag_t **script_list);

unsigned int countScripts(XeTeXFont font);

unsigned int countLanguages(XeTeXFont font, hb_tag_t script);

unsigned int countFeatures(XeTeXFont font, hb_tag_t script, hb_tag_t language);

void initializeFont(XeTeXFontBase *self, const char *pathname, int index, int *status);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* XETEX_LAYOUT_BINDINGS_H */
