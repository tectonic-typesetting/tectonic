/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew
 Copyright (c) 2012, 2013 by Jiang Jiang
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

#ifndef __XETEXEXT_H
#define __XETEXEXT_H

#include "tectonic.h"
#include "internals.h"
#include "xetex-core.h"
#include "XeTeXLayoutInterface.h"

#define AAT_FONT_FLAG   0xFFFFu
#define OTGR_FONT_FLAG  0xFFFEu

/* some typedefs that XeTeX uses - on Mac OS, we get these from Apple headers,
   but otherwise we'll need these substitute definitions */

#define pdfbox_crop 1
#define pdfbox_media 2
#define pdfbox_bleed 3
#define pdfbox_trim 4
#define pdfbox_art 5
#define pdfbox_none 6

/* command codes for XeTeX extension commands */
#define XeTeX_count_glyphs 1
#define XeTeX_count_features 8
#define XeTeX_feature_code 9
#define XeTeX_find_feature_by_name 10
#define XeTeX_is_exclusive_feature 11
#define XeTeX_count_selectors 12
#define XeTeX_selector_code 13
#define XeTeX_find_selector_by_name 14
#define XeTeX_is_default_selector 15
#define XeTeX_OT_count_scripts 16
#define XeTeX_OT_count_languages 17
#define XeTeX_OT_count_features 18
#define XeTeX_OT_script_code 19
#define XeTeX_OT_language_code 20
#define XeTeX_OT_feature_code 21
#define XeTeX_map_char_to_glyph_code 22

#define XeTeX_feature_name 8
#define XeTeX_selector_name 9

#define LEFT_SIDE 0
#define RIGHT_SIDE 1

/* accessing info in a native_word_node */
#define width_offset 1
#define depth_offset 2
#define height_offset 3
#define native_info_offset 4
#define native_glyph_info_offset 5

#define node_width(node)            node[width_offset].hh.v.RH
#define node_depth(node)            node[depth_offset].hh.v.RH
#define node_height(node)           node[height_offset].hh.v.RH
#define native_length(node)         node[native_info_offset].qqqq.B2
#define native_font(node)           node[native_info_offset].qqqq.B1
#define native_glyph_count(node)    node[native_info_offset].qqqq.B3
#define native_glyph_info_ptr(node) node[native_glyph_info_offset].ptr
#define native_glyph_info_size      10 /* info for each glyph is location (FixedPoint) + glyph ID (uint16_t) */

#define native_glyph(p) native_length(p) /* glyph ID field in a glyph_node */

/* For Unicode encoding form interpretation... */

#ifdef XETEX_MAC
extern const CFStringRef kXeTeXEmboldenAttributeName;
#endif

BEGIN_EXTERN_C

void linebreak_start(int f, integer localeStrNum, uint16_t* text, integer textLength);
int linebreak_next(void);
int get_encoding_mode_and_info(integer* info);
void print_utf8_str(const unsigned char* str, int len);
void print_chars(const unsigned short* str, int len);
void* find_native_font(unsigned char* name, integer scaled_size);
void release_font_engine(void* engine, int type_flag);
int readCommonFeatures(const char* feat, const char* end, float* extend,
                       float* slant, float* embolden, float* letterspace, uint32_t* rgbValue);

/* the metrics params here are really TeX 'scaled' values, but that typedef
 * isn't available every place this is included */

void ot_get_font_metrics(void* engine, integer* ascent, integer* descent, integer* xheight,
                         integer* capheight, integer* slant);
void get_native_char_height_depth(integer font, integer ch, integer* height, integer* depth);
void get_native_char_sidebearings(integer font, integer ch, integer* lsb, integer* rsb);

/* single-purpose metrics accessors */
integer getnativecharwd(integer font, integer ch);
integer getnativecharht(integer font, integer ch);
integer getnativechardp(integer font, integer ch);
integer getnativecharic(integer font, integer ch);

integer get_glyph_bounds(integer font, integer edge, integer gid);

integer ot_font_get(integer what, void* engine);
integer ot_font_get_1(integer what, void* engine, integer param);
integer ot_font_get_2(integer what, void* engine, integer param1, integer param2);
integer ot_font_get_3(integer what, void* engine, integer param1, integer param2, integer param3);

int makeXDVGlyphArrayData(void* p);
int make_font_def(integer f);
int apply_mapping(void* cnv, uint16_t* txtPtr, int txtLen);
void store_justified_native_glyphs(void* node);
void measure_native_node(void* node, int use_glyph_metrics);
Fixed real_get_native_italic_correction(void* node);
Fixed real_get_native_glyph_italic_correction(void* node);
integer real_get_native_word_cp(void* node, int side);
void measure_native_glyph(void* node, int use_glyph_metrics);
integer map_char_to_glyph(integer font, integer ch);
integer map_glyph_to_index(integer font);
integer get_font_char_range(integer font, int first);
void print_glyph_name(integer font, integer gid);
uint16_t real_get_native_glyph(void* pNode, unsigned int index);

void gr_print_font_name(integer what, void* pEngine, integer param1, integer param2);
integer gr_font_get_named(integer what, void* pEngine);
integer gr_font_get_named_1(integer what, void* pEngine, integer param);

double read_double(const char** s);
unsigned int read_rgb_a(const char** cp);

int count_pdf_file_pages(void);
int find_pic_file(char** path, real_rect* bounds, int pdfBoxType, int page);

int maketexstring(const char* s);
void set_cp_code(int fontNum, unsigned int code, int side, int value);
int get_cp_code(int fontNum, unsigned int code, int side);
double Fix2D(Fixed f);
Fixed D2Fix(double d);

void check_for_tfm_font_mapping(void);
void* load_tfm_font_mapping(void);
int apply_tfm_font_mapping(void* mapping, int c);

int aat_font_get(int what, CFDictionaryRef attrs);
int aat_font_get_1(int what, CFDictionaryRef attrs, int param);
int aat_font_get_2(int what, CFDictionaryRef attrs, int param1, int param2);
int aat_font_get_named(int what, CFDictionaryRef attrs);
int aat_font_get_named_1(int what, CFDictionaryRef attrs, int param);
void aat_print_font_name(int what, CFDictionaryRef attrs, int param1, int param2);
/* the metrics params here are really TeX 'scaled' (or MacOS 'Fixed') values, but that typedef isn't available every place this is included */
void aat_get_font_metrics(CFDictionaryRef attrs, integer* ascent, integer* descent, integer* xheight, integer* capheight, integer* slant);

#ifdef XETEX_MAC
/* functions in XeTeX_mac.c */
void* loadAATfont(CTFontDescriptorRef descriptor, integer scaled_size, const char* cp1);
void DoAATLayout(void* node, int justify);
void GetGlyphBBox_AAT(CFDictionaryRef fontAttrs, uint16_t gid, GlyphBBox* bbox);
double GetGlyphWidth_AAT(CFDictionaryRef fontAttrs, uint16_t gid);
void GetGlyphHeightDepth_AAT(CFDictionaryRef fontAttrs, uint16_t gid, float* ht, float* dp);
void GetGlyphSidebearings_AAT(CFDictionaryRef fontAttrs, uint16_t gid, float* lsb, float* rsb);
double GetGlyphItalCorr_AAT(CFDictionaryRef fontAttrs, uint16_t gid);
int MapCharToGlyph_AAT(CFDictionaryRef fontAttrs, UInt32 ch);
int MapGlyphToIndex_AAT(CFDictionaryRef attributes, const char* glyphName);
char* GetGlyphNameFromCTFont(CTFontRef ctFontRef, uint16_t gid, int* len);
CFDictionaryRef findDictionaryInArray(CFArrayRef array, const void* nameKey, const char* name, int nameLength);
CFDictionaryRef findDictionaryInArrayWithIdentifier(CFArrayRef array, const void* identifierKey, int identifier);
CFNumberRef findSelectorByName(CFDictionaryRef feature, const char* name, int nameLength);
char* getNameFromCTFont(CTFontRef ctFontRef, CFStringRef nameKey);
char* getFileNameFromCTFont(CTFontRef ctFontRef, uint32_t* index);
int GetFontCharRange_AAT(CFDictionaryRef fontAttrs, int reqFirst);
CTFontRef fontFromAttributes(CFDictionaryRef fontAttrs);
CTFontRef fontFromInteger(integer font);
#endif

END_EXTERN_C

#endif /* __XETEX_EXT_H */
