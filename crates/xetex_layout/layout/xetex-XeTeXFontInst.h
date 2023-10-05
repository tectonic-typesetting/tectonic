/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009, 2011 by Jonathan Kew

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

/*
 *   file name:  XeTeXFontInst.h
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 *
 *  originally based on PortableFontInstance.h from ICU
 */

#ifndef __XeTeXFontInst_H
#define __XeTeXFontInst_H

#include "tectonic_bridge_core.h"
#include "xetex-XeTeXFontMgr.h"

#include <ft2build.h>
#include FT_GLYPH_H
#include FT_ADVANCES_H
#include FT_TRUETYPE_TABLES_H

#include <unicode/umachine.h>

// create specific subclasses for each supported platform

struct XeTeXFontInst
{
    unsigned short m_unitsPerEM;
    float m_pointSize;
    float m_ascent;
    float m_descent;
    float m_capHeight;
    float m_xHeight;
    float m_italicAngle;

    bool m_vertical; // false = horizontal, true = vertical

    char *m_filename; // font filename
    uint32_t m_index; // face index

    FT_Face m_ftFace;
    FT_Byte *m_backingData, *m_backingData2;
    hb_font_t* m_hbFont;

    XeTeXFontInst(const char* filename, int index, float pointSize, int &status);

    virtual ~XeTeXFontInst();

    void *getFontTable(FT_Sfnt_Tag tableTag) const;

    const char *getFilename(uint32_t* index) const
    {
        *index = m_index;
        return m_filename;
    }
    hb_font_t *getHbFont() const { return m_hbFont; }
    bool getLayoutDirVertical() const { return m_vertical; }

    float getPointSize() const { return m_pointSize; }
    float getAscent() const { return m_ascent; }
    float getDescent() const { return m_descent; }
    float getCapHeight() const { return m_capHeight; }
    float getXHeight() const { return m_xHeight; }

    GlyphID mapCharToGlyph(UChar32 ch) const;
    GlyphID mapGlyphToIndex(const char* glyphName) const;

    void getGlyphBounds(GlyphID glyph, GlyphBBox* bbox);

    void getGlyphHeightDepth(GlyphID glyph, float *ht, float* dp);
    void getGlyphSidebearings(GlyphID glyph, float* lsb, float* rsb);
    float getGlyphItalCorr(GlyphID glyph);

    const char* getGlyphName(GlyphID gid, int& nameLen);

    UChar32 getFirstCharCode();
    UChar32 getLastCharCode();

    /* Tectonic: these are modified from the base XeTeX code to use doubles;
     * otherwise roundoff errors can accumulate leading to differences in the
     * XDV outputs. */
    float unitsToPoints(double units) const
    {
        return (units * m_pointSize) / m_unitsPerEM;
    }

    float pointsToUnits(double points) const
    {
        return (points * m_unitsPerEM) / m_pointSize;
    }
};

#endif
