/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
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

/* XeTeX_mac.c
 * additional plain C extensions for XeTeX - MacOS-specific routines
 */

#include "xetex-core.h"
#include "xetex-xetexd.h"
#include "teckit-c-Engine.h"
#include "xetex-ext.h"

#include <ApplicationServices/ApplicationServices.h>


static inline double
TeXtoPSPoints(double pts)
{
    return pts * 72.0 / 72.27;
}

static inline double
PStoTeXPoints(double pts)
{
    return pts * 72.27 / 72.0;
}

static inline Fixed
FixedPStoTeXPoints(double pts)
{
    return D2Fix(PStoTeXPoints(pts));
}

CTFontRef
fontFromAttributes(CFDictionaryRef attributes)
{
    return CFDictionaryGetValue(attributes, kCTFontAttributeName);
}

CTFontRef
fontFromInteger(int32_t font)
{
    CFDictionaryRef attributes = (CFDictionaryRef) font_layout_engine[font];
    return fontFromAttributes(attributes);
}

void
DoAATLayout(void* p, int justify)
{
    CFArrayRef glyphRuns;
    CFIndex i, j, runCount;
    CFIndex totalGlyphCount = 0;
    UInt16* glyphIDs;
    Fixed* glyphAdvances;
    void* glyph_info = NULL;
    FixedPoint* locations;
    CGFloat width;

    long txtLen;
    const UniChar* txtPtr;

    CFDictionaryRef attributes;
    CFStringRef string;
    CFAttributedStringRef attrString;
    CTTypesetterRef typesetter;
    CTLineRef line;

    memory_word* node = (memory_word*)p;

    unsigned int f = native_font(node);
    if (font_area[f] != AAT_FONT_FLAG)
        _tt_abort("DoAATLayout called for non-AAT font");

    txtLen = native_length(node);
    txtPtr = (UniChar*)(node + NATIVE_NODE_SIZE);

    attributes = font_layout_engine[native_font(node)];
    string = CFStringCreateWithCharactersNoCopy(NULL, txtPtr, txtLen, kCFAllocatorNull);
    attrString = CFAttributedStringCreate(NULL, string, attributes);
    CFRelease(string);

    typesetter = CTTypesetterCreateWithAttributedString(attrString);
    CFRelease(attrString);
    line = CTTypesetterCreateLine(typesetter, CFRangeMake(0, txtLen));
    if (justify) {
        CGFloat lineWidth = TeXtoPSPoints(Fix2D(node_width(node)));
        CTLineRef justifiedLine = CTLineCreateJustifiedLine(line, TeXtoPSPoints(Fix2D(fract1)), lineWidth);
        // TODO(jjgod): how to handle the case when justification failed? for
        // now we just fallback to use the original line.
        if (justifiedLine) {
            CFRelease(line);
            line = justifiedLine;
        }
    }

    glyphRuns = CTLineGetGlyphRuns(line);
    runCount = CFArrayGetCount(glyphRuns);
    totalGlyphCount = CTLineGetGlyphCount(line);

    if (totalGlyphCount > 0) {
        glyph_info = xmalloc(totalGlyphCount * native_glyph_info_size);
        locations = (FixedPoint*)glyph_info;
        glyphIDs = (UInt16*)(locations + totalGlyphCount);
        glyphAdvances = xmalloc(totalGlyphCount * sizeof(Fixed));
        totalGlyphCount = 0;

        width = 0;
        for (i = 0; i < runCount; i++) {
            CTRunRef run = CFArrayGetValueAtIndex(glyphRuns, i);
            CFIndex count = CTRunGetGlyphCount(run);
            CFDictionaryRef runAttributes = CTRunGetAttributes(run);
            CFBooleanRef vertical = CFDictionaryGetValue(runAttributes, kCTVerticalFormsAttributeName);
            // TODO(jjgod): Avoid unnecessary allocation with CTRunGetFoosPtr().
            CGGlyph* glyphs = xmalloc(count * sizeof(CGGlyph));
            CGPoint* positions = xmalloc(count * sizeof(CGPoint));
            CGSize* advances = xmalloc(count * sizeof(CGSize));
            CGFloat runWidth = CTRunGetTypographicBounds(run, CFRangeMake(0, 0), NULL, NULL, NULL);
            CTRunGetGlyphs(run, CFRangeMake(0, 0), glyphs);
            CTRunGetPositions(run, CFRangeMake(0, 0), positions);
            CTRunGetAdvances(run, CFRangeMake(0, 0), advances);
            for (j = 0; j < count; j++) {
                // XXX Core Text has that font cascading thing that will do
                // font substitution for missing glyphs, which we do not want
                // but I can not find a way to disable it yet, so if the font
                // of the resulting run is not the same font we asked for, use
                // the glyph at index 0 (usually .notdef) instead or we will be
                // showing garbage or even invalid glyphs
                if (!CFEqual(fontFromAttributes(attributes), fontFromAttributes(runAttributes)))
                    glyphIDs[totalGlyphCount] = 0;
                else
                    glyphIDs[totalGlyphCount] = glyphs[j];

                // Swap X and Y when doing vertical layout
                if (vertical == kCFBooleanTrue) {
                    locations[totalGlyphCount].x = -FixedPStoTeXPoints(positions[j].y);
                    locations[totalGlyphCount].y =  FixedPStoTeXPoints(positions[j].x);
                } else {
                    locations[totalGlyphCount].x =  FixedPStoTeXPoints(positions[j].x);
                    locations[totalGlyphCount].y = -FixedPStoTeXPoints(positions[j].y);
                }
                glyphAdvances[totalGlyphCount] = advances[j].width;
                totalGlyphCount++;
            }
            width += FixedPStoTeXPoints(runWidth);
            free(glyphs);
            free(positions);
            free(advances);
        }
    }

    native_glyph_count(node) = totalGlyphCount;
    native_glyph_info_ptr(node) = glyph_info;

    if (!justify) {
        node_width(node) = width;

        if (totalGlyphCount > 0) {
            /* this is essentially a copy from similar code in XeTeX_ext.c, easier
             * to be done here */
            if (font_letter_space[f] != 0) {
                Fixed lsDelta = 0;
                Fixed lsUnit = font_letter_space[f];
                int i;
                for (i = 0; i < totalGlyphCount; ++i) {
                    if (glyphAdvances[i] == 0 && lsDelta != 0)
                        lsDelta -= lsUnit;
                    locations[i].x += lsDelta;
                    lsDelta += lsUnit;
                }
                if (lsDelta != 0) {
                    lsDelta -= lsUnit;
                    node_width(node) += lsDelta;
                }
            }
        }
    }

    free(glyphAdvances);
    CFRelease(line);
    CFRelease(typesetter);
}

static void
getGlyphBBoxFromCTFont(CTFontRef font, UInt16 gid, GlyphBBox* bbox)
{
    CGRect rect;

    bbox->xMin = 65536.0;
    bbox->yMin = 65536.0;
    bbox->xMax = -65536.0;
    bbox->yMax = -65536.0;

    rect = CTFontGetBoundingRectsForGlyphs(font,
        0, /* Use default orientation for now, handle vertical later */
        (const CGGlyph *) &gid, NULL, 1);

    if (CGRectIsNull(rect))
        bbox->xMin = bbox->yMin = bbox->xMax = bbox->yMax = 0;
    else {
        bbox->yMin = PStoTeXPoints(rect.origin.y);
        bbox->yMax = PStoTeXPoints(rect.origin.y + rect.size.height);
        bbox->xMin = PStoTeXPoints(rect.origin.x);
        bbox->xMax = PStoTeXPoints(rect.origin.x + rect.size.width);
    }
}

void
GetGlyphBBox_AAT(CFDictionaryRef attributes, UInt16 gid, GlyphBBox* bbox)
    /* returns glyph bounding box in TeX points */
{
    CTFontRef font = fontFromAttributes(attributes);
    return getGlyphBBoxFromCTFont(font, gid, bbox);
}

static double
getGlyphWidthFromCTFont(CTFontRef font, UInt16 gid)
{
    return PStoTeXPoints(CTFontGetAdvancesForGlyphs(font, kCTFontOrientationHorizontal, &gid, NULL, 1));
}

double
GetGlyphWidth_AAT(CFDictionaryRef attributes, UInt16 gid)
    /* returns TeX points */
{
    CTFontRef font = fontFromAttributes(attributes);
    return getGlyphWidthFromCTFont(font, gid);
}

void
GetGlyphHeightDepth_AAT(CFDictionaryRef attributes, UInt16 gid, float* ht, float* dp)
    /* returns TeX points */
{
    GlyphBBox bbox;

    GetGlyphBBox_AAT(attributes, gid, &bbox);

    *ht = bbox.yMax;
    *dp = -bbox.yMin;
}

void
GetGlyphSidebearings_AAT(CFDictionaryRef attributes, UInt16 gid, float* lsb, float* rsb)
    /* returns TeX points */
{
    CTFontRef font = fontFromAttributes(attributes);
    CGSize advances[1] = { CGSizeMake(0, 0) };
    double advance = CTFontGetAdvancesForGlyphs(font, 0, &gid, advances, 1);
    GlyphBBox bbox;
    getGlyphBBoxFromCTFont(font, gid, &bbox);
    *lsb = bbox.xMin;
    *rsb = PStoTeXPoints(advance) - bbox.xMax;
}

double
GetGlyphItalCorr_AAT(CFDictionaryRef attributes, UInt16 gid)
{
    CTFontRef font = fontFromAttributes(attributes);
    CGSize advances[1] = { CGSizeMake(0, 0) };
    double advance = CTFontGetAdvancesForGlyphs(font, 0, &gid, advances, 1);

    GlyphBBox bbox;
    getGlyphBBoxFromCTFont(font, gid, &bbox);

    if (bbox.xMax > PStoTeXPoints(advance))
        return bbox.xMax - PStoTeXPoints(advance);
    return 0;
}

static int
mapCharToGlyphFromCTFont(CTFontRef font, UInt32 ch)
{
    CGGlyph glyphs[2] = { 0 };
    UniChar txt[2];
    int     len = 1;

    if (ch > 0xffff) {
        ch -= 0x10000;
        txt[0] = 0xd800 + ch / 1024;
        txt[1] = 0xdc00 + ch % 1024;
        len = 2;
    } else {
        txt[0] = ch;
    }

    if (CTFontGetGlyphsForCharacters(font, txt, glyphs, len))
        return glyphs[0];

    return 0;
}

int
MapCharToGlyph_AAT(CFDictionaryRef attributes, UInt32 ch)
{
    CTFontRef font = fontFromAttributes(attributes);
    return mapCharToGlyphFromCTFont(font, ch);
}

static int
GetGlyphIDFromCTFont(CTFontRef ctFontRef, const char* glyphName)
{
    CFStringRef glyphname = CFStringCreateWithCStringNoCopy(kCFAllocatorDefault,
                                                            glyphName,
                                                            kCFStringEncodingUTF8,
                                                            kCFAllocatorNull);
    int rval = CTFontGetGlyphWithName(ctFontRef, glyphname);
    CFRelease(glyphname);
    return rval;
}

int
MapGlyphToIndex_AAT(CFDictionaryRef attributes, const char* glyphName)
{
    CTFontRef font = fontFromAttributes(attributes);
    return GetGlyphIDFromCTFont(font, glyphName);
}

char*
GetGlyphNameFromCTFont(CTFontRef ctFontRef, UInt16 gid, int* len)
{
    CGFontRef cgfont;
    static char buffer[256];
    buffer[0] = 0;
    *len = 0;

    cgfont = CTFontCopyGraphicsFont(ctFontRef, 0);
    if (cgfont && gid < CGFontGetNumberOfGlyphs(cgfont)) {
        CFStringRef glyphname = CGFontCopyGlyphNameForGlyph(cgfont, gid);
        if (glyphname) {
            if (CFStringGetCString(glyphname, buffer, 256, kCFStringEncodingUTF8)) {
                *len = strlen(buffer);
            }
            CFRelease(glyphname);
        }
        CGFontRelease(cgfont);
    }

    return &buffer[0];
}

int
GetFontCharRange_AAT(CFDictionaryRef attributes, int reqFirst)
{
    if (reqFirst) {
        int ch = 0;
        while (MapCharToGlyph_AAT(attributes, ch) == 0 && ch < 0x10ffff)
            ++ch;
        return ch;
    } else {
        int ch = 0x10ffff;
        while (MapCharToGlyph_AAT(attributes, ch) == 0 && ch > 0)
            --ch;
        return ch;
    }
}

CFDictionaryRef
findDictionaryInArrayWithIdentifier(CFArrayRef array, const void* identifierKey, int identifier)
{
    CFDictionaryRef dict = NULL;

    if (array) {
        int value = -1;
        CFIndex i;
        for (i = 0; i < CFArrayGetCount(array); i++) {
            CFDictionaryRef item = CFArrayGetValueAtIndex(array, i);
            CFNumberRef itemId = CFDictionaryGetValue(item, identifierKey);
            if (itemId) {
                CFNumberGetValue(itemId, kCFNumberIntType, &value);
                if (value == identifier) {
                    dict = item;
                    break;
                }
            }
        }
    }
    return dict;
}

CFDictionaryRef
findDictionaryInArray(CFArrayRef array, const void* nameKey, const char* name, int nameLength)
{
    CFDictionaryRef dict = NULL;

    if (array) {
        CFStringRef itemName;
        CFIndex i;
        itemName = CFStringCreateWithBytes(NULL, (UInt8 *) name, nameLength,
                                           kCFStringEncodingUTF8, false);
        for (i = 0; i < CFArrayGetCount(array); i++) {
            CFDictionaryRef item = CFArrayGetValueAtIndex(array, i);
            CFStringRef iName = CFDictionaryGetValue(item, nameKey);
            if (iName && !CFStringCompare(itemName, iName, kCFCompareCaseInsensitive)) {
                dict = item;
                break;
            }
        }
        CFRelease(itemName);
    }
    return dict;
}

CFNumberRef
findSelectorByName(CFDictionaryRef feature, const char* name, int nameLength)
{
    CFNumberRef selector = NULL;
    CFArrayRef selectors = CFDictionaryGetValue(feature, kCTFontFeatureTypeSelectorsKey);
    if (selectors) {
        CFDictionaryRef s = findDictionaryInArray(selectors, kCTFontFeatureSelectorNameKey, name, nameLength);
        if (s)
            selector = CFDictionaryGetValue(s, kCTFontFeatureSelectorIdentifierKey);
    }
    return selector;
}

static CFDictionaryRef
createFeatureSettingDictionary(CFNumberRef featureTypeIdentifier, CFNumberRef featureSelectorIdentifier)
{
    const void* settingKeys[] = { kCTFontFeatureTypeIdentifierKey, kCTFontFeatureSelectorIdentifierKey };
    const void* settingValues[] = { featureTypeIdentifier, featureSelectorIdentifier };

    return CFDictionaryCreate(kCFAllocatorDefault, settingKeys, settingValues, 2,
                              &kCFTypeDictionaryKeyCallBacks, &kCFTypeDictionaryValueCallBacks);
}

const CFStringRef kXeTeXEmboldenAttributeName = CFSTR("XeTeXEmbolden");

void*
loadAATfont(CTFontDescriptorRef descriptor, int32_t scaled_size, const char* cp1)
{
    CTFontRef font, actualFont;
    CGFloat ctSize;
    CFMutableDictionaryRef stringAttributes, attributes;
    CGAffineTransform matrix;
    CFMutableArrayRef cascadeList;
    CTFontDescriptorRef lastResort;
    double  tracking    = 0.0;
    float   extend      = 1.0;
    float   slant       = 0.0;
    float   embolden    = 0.0;
    float   letterspace = 0.0;
    uint32_t rgbValue;

    // create a base font instance for applying further attributes
    ctSize = TeXtoPSPoints(Fix2D(scaled_size));
    font = CTFontCreateWithFontDescriptor(descriptor, ctSize, NULL);
    if (!font)
        return NULL;

    stringAttributes = CFDictionaryCreateMutable(NULL, 0,
                                  &kCFTypeDictionaryKeyCallBacks,
                                  &kCFTypeDictionaryValueCallBacks);
    attributes = CFDictionaryCreateMutable(NULL, 0,
                                  &kCFTypeDictionaryKeyCallBacks,
                                  &kCFTypeDictionaryValueCallBacks);
    if (cp1) {
        CFArrayRef features = CTFontCopyFeatures(font);
        CFMutableArrayRef featureSettings =
            CFArrayCreateMutable(NULL, 0, &kCFTypeArrayCallBacks);

        // interpret features following ":"
        while (*cp1) {
            CFDictionaryRef feature;
            int ret;
            const char* cp2;
            const char* cp3;
            // locate beginning of name=value pair
            if (*cp1 == ':' || *cp1 == ';') // skip over separator
                ++cp1;
            while (*cp1 == ' ' || *cp1 == '\t') // skip leading whitespace
                ++cp1;
            if (*cp1 == 0) // break if end of string
                break;

            // scan to end of pair
            cp2 = cp1;
            while (*cp2 && (*cp2 != ';') && (*cp2 != ':'))
                ++cp2;

            // look for the '=' separator
            cp3 = cp1;
            while ((cp3 < cp2) && (*cp3 != '='))
                ++cp3;
            if (cp3 == cp2)
                goto bad_option;

            // now cp1 points to option name, cp3 to '=', cp2 to ';' or null

            // first try for a feature by this name
            feature = findDictionaryInArray(features, kCTFontFeatureTypeNameKey, cp1, cp3 - cp1);
            if (feature) {
                // look past the '=' separator for setting names
                int featLen = cp3 - cp1;
                int zeroInteger = 0;
                CFNumberRef zero = CFNumberCreate(NULL, kCFNumberIntType, &zeroInteger);
                ++cp3;
                while (cp3 < cp2) {
                    CFNumberRef selector;
                    int disable = 0;
                    const char* cp4;
                    // skip leading whitespace
                    while (*cp3 == ' ' || *cp3 == '\t')
                        ++cp3;

                    // possibly multiple settings...
                    if (*cp3 == '!') { // check for negation
                        disable = 1;
                        ++cp3;
                    }

                    // scan for end of setting name
                    cp4 = cp3;
                    while (cp4 < cp2 && *cp4 != ',')
                        ++cp4;

                    // now cp3 points to name, cp4 to ',' or ';' or null
                    selector = findSelectorByName(feature, cp3, cp4 - cp3);
                    if (selector && CFNumberCompare(selector, zero, NULL) >= 0) {
                        CFNumberRef featureType = CFDictionaryGetValue(feature, kCTFontFeatureTypeIdentifierKey);
                        CFDictionaryRef featureSetting = createFeatureSettingDictionary(featureType, selector);
                        CFArrayAppendValue(featureSettings, featureSetting);
                        CFRelease(featureSetting);
                    } else {
                        font_feature_warning(cp1, featLen, cp3, cp4 - cp3);
                    }

                    // point beyond setting name terminator
                    cp3 = cp4 + 1;
                }
                CFRelease(zero);

                goto next_option;
            }

            // didn't find feature, try other options...
            ret = readCommonFeatures(cp1, cp2, &extend, &slant, &embolden, &letterspace, &rgbValue);
            if (ret == 1)
                goto next_option;
            else if (ret == -1)
                goto bad_option;

            cp3 = strstartswith(cp1, "tracking");
            if (cp3) {
                CFNumberRef trackingNumber;
                if (*cp3 != '=')
                    goto bad_option;
                ++cp3;
                tracking = read_double(&cp3);
                trackingNumber = CFNumberCreate(NULL, kCFNumberDoubleType, &tracking);
                CFDictionaryAddValue(stringAttributes, kCTKernAttributeName, trackingNumber);
                CFRelease(trackingNumber);
                goto next_option;
            }

            bad_option:
                // not a name=value pair, or not recognized....
                // check for plain "vertical" before complaining
                if (strstartswith(cp1, "vertical")) {
                    cp3 = cp2;
                    if (*cp3 == ';' || *cp3 == ':')
                        --cp3;
                    while (*cp3 == '\0' || *cp3 == ' ' || *cp3 == '\t')
                        --cp3;
                    if (*cp3)
                        ++cp3;
                    if (cp3 == cp1 + 8) {
                        int orientation = kCTFontOrientationVertical;
                        CFNumberRef orientationNumber = CFNumberCreate(NULL, kCFNumberIntType, &orientation);
                        CFDictionaryAddValue(attributes, kCTFontOrientationAttribute, orientationNumber);
                        CFRelease(orientationNumber);
                        CFDictionaryAddValue(stringAttributes, kCTVerticalFormsAttributeName, kCFBooleanTrue);
                        goto next_option;
                    }
                }

                font_feature_warning(cp1, cp2 - cp1, 0, 0);

            next_option:
                // go to next name=value pair
                cp1 = cp2;
        }

        if (features)
            CFRelease(features);

        if (CFArrayGetCount(featureSettings))
            CFDictionaryAddValue(attributes, kCTFontFeatureSettingsAttribute, featureSettings);
        CFRelease(featureSettings);
    }

    if ((loaded_font_flags & FONT_FLAGS_COLORED) != 0) {
        CGFloat red   = ((rgbValue & 0xFF000000) >> 24) / 255.0;
        CGFloat green = ((rgbValue & 0x00FF0000) >> 16) / 255.0;
        CGFloat blue  = ((rgbValue & 0x0000FF00) >> 8 ) / 255.0;
        CGFloat alpha = ((rgbValue & 0x000000FF)) / 255.0;
        CGColorRef color = CGColorCreateGenericRGB(red, green, blue, alpha);
        CFDictionaryAddValue(stringAttributes, kCTForegroundColorAttributeName, color);
        CGColorRelease(color);
    }

    matrix = CGAffineTransformIdentity;
    if (extend != 1.0 || slant != 0.0)
        matrix = CGAffineTransformMake(extend, 0, slant, 1.0, 0, 0);

    if (embolden != 0.0) {
        CFNumberRef emboldenNumber;
        embolden = embolden * Fix2D(scaled_size) / 100.0;
        emboldenNumber = CFNumberCreate(NULL, kCFNumberFloatType, &embolden);
        CFDictionaryAddValue(stringAttributes, kXeTeXEmboldenAttributeName, emboldenNumber);
        CFRelease(emboldenNumber);
    }

    if (letterspace != 0.0)
        loaded_font_letter_space = (letterspace / 100.0) * scaled_size;

    // Disable Core Text font fallback (cascading) with only the last resort font
    // in the cascade list.
    cascadeList = CFArrayCreateMutable(NULL, 1, &kCFTypeArrayCallBacks);
    lastResort = CTFontDescriptorCreateWithNameAndSize(CFSTR("LastResort"), 0);
    CFArrayAppendValue(cascadeList, lastResort);
    CFRelease(lastResort);
    CFDictionaryAddValue(attributes, kCTFontCascadeListAttribute, cascadeList);
    CFRelease(cascadeList);

    descriptor = CTFontDescriptorCreateWithAttributes(attributes);
    CFRelease(attributes);
    actualFont = CTFontCreateCopyWithAttributes(font, ctSize, &matrix, descriptor);
    CFRelease(font);
    CFDictionaryAddValue(stringAttributes, kCTFontAttributeName, actualFont);
    CFRelease(actualFont);

    native_font_type_flag = AAT_FONT_FLAG;
    return (void *) stringAttributes;
}
