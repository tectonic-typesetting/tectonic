/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew

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
 *   file name:  XeTeXFontInst.cpp
 *
 *   created on: 2005-10-22
 *   created by: Jonathan Kew
 *
 *     originally based on PortableFontInstance.cpp from ICU
 */

#include "tectonic_xetex_layout.h"
#include "xetex-XeTeXFontInst.h"

#include <string.h>

FT_Library gFreeTypeLibrary = 0;

XeTeXFontInst::XeTeXFontInst(const char* pathname, int index, float pointSize, int &status)
    : m_unitsPerEM(0)
    , m_pointSize(pointSize)
    , m_ascent(0)
    , m_descent(0)
    , m_capHeight(0)
    , m_xHeight(0)
    , m_italicAngle(0)
    , m_vertical(false)
    , m_filename(NULL)
    , m_index(0)
    , m_ftFace(0)
    , m_backingData(NULL)
    , m_backingData2(NULL)
    , m_hbFont(NULL)
{
    if (pathname != NULL)
        initializeFont((XeTeXFontBase*)this, pathname, index, &status);
}

XeTeXFontInst::~XeTeXFontInst()
{
    if (m_ftFace != 0) {
        FT_Done_Face(m_ftFace);
        m_ftFace = 0;
    }
    hb_font_destroy(m_hbFont);
    free(m_backingData);
    free(m_backingData2);
    free(m_filename);
}

void *
XeTeXFontInst::getFontTable(FT_Sfnt_Tag tag) const
{
    return FT_Get_Sfnt_Table(m_ftFace, tag);
}

void
XeTeXFontInst::getGlyphBounds(GlyphID gid, GlyphBBox* bbox)
{
    bbox->xMin = bbox->yMin = bbox->xMax = bbox->yMax = 0.0;

    FT_Error error = FT_Load_Glyph(m_ftFace, gid, FT_LOAD_NO_SCALE);
    if (error)
        return;

    FT_Glyph glyph;
    error = FT_Get_Glyph(m_ftFace->glyph, &glyph);
    if (error == 0) {
        FT_BBox ft_bbox;
        FT_Glyph_Get_CBox(glyph, FT_GLYPH_BBOX_UNSCALED, &ft_bbox);
        bbox->xMin = unitsToPoints(ft_bbox.xMin);
        bbox->yMin = unitsToPoints(ft_bbox.yMin);
        bbox->xMax = unitsToPoints(ft_bbox.xMax);
        bbox->yMax = unitsToPoints(ft_bbox.yMax);
        FT_Done_Glyph(glyph);
    }
}

GlyphID
XeTeXFontInst::mapCharToGlyph(UChar32 ch) const
{
    return FT_Get_Char_Index(m_ftFace, ch);
}

void
XeTeXFontInst::getGlyphHeightDepth(GlyphID gid, float* ht, float* dp)
{
    GlyphBBox bbox;
    getGlyphBounds(gid, &bbox);

    if (ht)
        *ht = bbox.yMax;
    if (dp)
        *dp = -bbox.yMin;
}

void
XeTeXFontInst::getGlyphSidebearings(GlyphID gid, float* lsb, float* rsb)
{
    float width = getGlyphWidth((XeTeXFont)this, gid);

    GlyphBBox bbox;
    getGlyphBounds(gid, &bbox);

    if (lsb)
        *lsb = bbox.xMin;
    if (rsb)
        *rsb = width - bbox.xMax;
}

float
XeTeXFontInst::getGlyphItalCorr(GlyphID gid)
{
    float rval = 0.0;

    float width = getGlyphWidth((XeTeXFont)this, gid);

    GlyphBBox bbox;
    getGlyphBounds(gid, &bbox);

    if (bbox.xMax > width)
        rval = bbox.xMax - width;

    return rval;
}

GlyphID
XeTeXFontInst::mapGlyphToIndex(const char* glyphName) const
{
    return FT_Get_Name_Index(m_ftFace, const_cast<char*>(glyphName));
}

const char*
XeTeXFontInst::getGlyphName(GlyphID gid, int& nameLen)
{
    if (FT_HAS_GLYPH_NAMES(m_ftFace)) {
        static char buffer[256];
        FT_Get_Glyph_Name(m_ftFace, gid, buffer, 256);
        nameLen = strlen(buffer);
        return &buffer[0];
    }
    else {
        nameLen = 0;
        return NULL;
    }
}

UChar32
XeTeXFontInst::getFirstCharCode()
{
    FT_UInt gindex;
    return FT_Get_First_Char(m_ftFace, &gindex);
}

UChar32
XeTeXFontInst::getLastCharCode()
{
    FT_UInt gindex;
    UChar32 ch = FT_Get_First_Char(m_ftFace, &gindex);
    UChar32 prev = ch;
    while (gindex != 0) {
        prev = ch;
        ch = FT_Get_Next_Char(m_ftFace, ch, &gindex);
    }
    return prev;
}
