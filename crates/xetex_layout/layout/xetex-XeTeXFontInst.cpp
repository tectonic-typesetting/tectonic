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

//void
//XeTeXFontInst::initialize(const char* pathname, int index, int &status)
//{
//    TT_Postscript *postTable;
//    TT_OS2* os2Table;
//    FT_Error error;
//    hb_face_t *hbFace;
//
//    if (!gFreeTypeLibrary) {
//        error = FT_Init_FreeType(&gFreeTypeLibrary);
//        if (error)
//            _tt_abort("FreeType initialization failed, error %d", error);
//    }
//
//    // Here we emulate some logic that was originally in find_native_font();
//    rust_input_handle_t handle = ttstub_input_open (pathname, TTBC_FILE_FORMAT_OPEN_TYPE, 0);
//    if (handle == NULL)
//        handle = ttstub_input_open (pathname, TTBC_FILE_FORMAT_TRUE_TYPE, 0);
//    if (handle == NULL)
//        handle = ttstub_input_open (pathname, TTBC_FILE_FORMAT_TYPE1, 0);
//    if (handle == NULL) {
//        status = 1;
//        return;
//    }
//
//    size_t sz = ttstub_input_get_size (handle);
//    m_backingData = (FT_Byte *) xmalloc (sz);
//    ssize_t r = ttstub_input_read (handle, (char *) m_backingData, sz);
//    if (r < 0 || (size_t) r != sz)
//        _tt_abort("failed to read font file");
//    ttstub_input_close(handle);
//
//    error = FT_New_Memory_Face(gFreeTypeLibrary, m_backingData, sz, index, &m_ftFace);
//
//    if (error || !FT_IS_SCALABLE(m_ftFace)) {
//        status = 1;
//        return;
//    }
//
//    /* for non-sfnt-packaged fonts (presumably Type 1), see if there is an AFM file we can attach */
//    if (index == 0 && !FT_IS_SFNT(m_ftFace)) {
//        // Tectonic: this code used to use kpse_find_file and FT_Attach_File
//        // to try to find metrics for this font. Thanks to the existence of
//        // FT_Attach_Stream we can emulate this behavior while going through
//        // the Rust I/O layer.
//
//        char *afm = xstrdup (xbasename (pathname));
//        char *p = strrchr (afm, '.');
//        if (p != NULL && strlen(p) == 4 && tolower(*(p+1)) == 'p' && tolower(*(p+2)) == 'f')
//            strcpy(p, ".afm");
//
//        rust_input_handle_t afm_handle = ttstub_input_open (afm, TTBC_FILE_FORMAT_AFM, 0);
//        free (afm);
//
//        if (afm_handle != NULL) {
//            sz = ttstub_input_get_size (afm_handle);
//            m_backingData2 = (FT_Byte *) xmalloc (sz);
//            r = ttstub_input_read (afm_handle, (char *) m_backingData2, sz);
//            if (r < 0 || (size_t) r != sz)
//                _tt_abort("failed to read AFM file");
//            ttstub_input_close(afm_handle);
//
//            FT_Open_Args open_args;
//            open_args.flags = FT_OPEN_MEMORY;
//            open_args.memory_base = m_backingData2;
//            open_args.memory_size = sz;
//
//            FT_Attach_Stream(m_ftFace, &open_args);
//        }
//    }
//
//    m_filename = xstrdup(pathname);
//    m_index = index;
//    m_unitsPerEM = m_ftFace->units_per_EM;
//    m_ascent = unitsToPoints(m_ftFace->ascender);
//    m_descent = unitsToPoints(m_ftFace->descender);
//
//    postTable = (TT_Postscript *) getFontTable(ft_sfnt_post);
//    if (postTable != NULL) {
//        m_italicAngle = RsFix2D(postTable->italicAngle);
//    }
//
//    os2Table = (TT_OS2*) getFontTable(ft_sfnt_os2);
//    if (os2Table) {
//        m_capHeight = unitsToPoints(os2Table->sCapHeight);
//        m_xHeight = unitsToPoints(os2Table->sxHeight);
//    }
//
//    // Set up HarfBuzz font
//    hbFace = hb_face_create_for_tables(_get_table, m_ftFace, NULL);
//    hb_face_set_index(hbFace, index);
//    hb_face_set_upem(hbFace, m_unitsPerEM);
//    m_hbFont = hb_font_create(hbFace);
//    hb_face_destroy(hbFace);
//
//    hb_font_set_funcs(m_hbFont, _get_font_funcs(), m_ftFace, NULL);
//    hb_font_set_scale(m_hbFont, m_unitsPerEM, m_unitsPerEM);
//    // We don’t want device tables adjustments
//    hb_font_set_ppem(m_hbFont, 0, 0);
//
//    return;
//}

void
XeTeXFontInst::setLayoutDirVertical(bool vertical)
{
    m_vertical = vertical;
}

void *
XeTeXFontInst::getFontTable(OTTag tag) const
{
    FT_ULong tmpLength = 0;
    FT_Error error = FT_Load_Sfnt_Table(m_ftFace, tag, 0, NULL, &tmpLength);
    if (error)
        return NULL;

    void* table = xmalloc(tmpLength * sizeof(char));
    if (table != NULL) {
        error = FT_Load_Sfnt_Table(m_ftFace, tag, 0, (FT_Byte*)table, &tmpLength);
        if (error) {
            free((void *) table);
            return NULL;
        }
    }

    return table;
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

uint16_t
XeTeXFontInst::getNumGlyphs() const
{
    return m_ftFace->num_glyphs;
}

float
XeTeXFontInst::getGlyphWidth(GlyphID gid)
{
    return unitsToPoints(_get_glyph_advance(m_ftFace, gid, false));
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
    float width = getGlyphWidth(gid);

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

    float width = getGlyphWidth(gid);

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
