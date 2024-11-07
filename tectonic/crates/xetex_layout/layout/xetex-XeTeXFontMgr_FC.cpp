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

#include "tectonic_bridge_core.h"

#include "xetex-XeTeXFontMgr_FC.h"

/* allow compilation with old Fontconfig header */
#ifndef FC_FULLNAME
#define FC_FULLNAME "fullname"
#endif

#include <ft2build.h>
#include FT_SFNT_NAMES_H
#include FT_TRUETYPE_IDS_H

#include <unicode/ucnv.h>

#define kFontFamilyName 1
#define kFontStyleName  2
#define kFontFullName   4
#define kPreferredFamilyName    16
#define kPreferredSubfamilyName 17

extern FT_Library gFreeTypeLibrary;

static UConverter* macRomanConv = NULL;
static UConverter* utf16beConv = NULL;
static UConverter* utf8Conv = NULL;

static char*
convertToUtf8(UConverter* conv, const unsigned char* name, int len)
{
    char* buffer1 = NULL;
    char* buffer2 = NULL;
    int bufSize = -1;

    if (2 * (len + 1) > bufSize) {
        if (buffer1 != NULL) {
            delete[] buffer1;
            delete[] buffer2;
        }
        bufSize = 2 * len + 100;
        buffer1 = new char[bufSize];
        buffer2 = new char[bufSize];
    }

    UErrorCode status = U_ZERO_ERROR;
    len = ucnv_toUChars(conv, (UChar*)buffer1, bufSize, (const char*)name, len, &status);
    len = ucnv_fromUChars(utf8Conv, buffer2, bufSize, (UChar*)buffer1, len, &status);
    buffer2[len] = 0;

    delete[] buffer1;
    return buffer2;
}

XeTeXFontMgr::NameCollection*
XeTeXFontMgr_FC::readNames(FcPattern* pat)
{
    NameCollection* names = new NameCollection;

    char* pathname;
    if (FcPatternGetString(pat, FC_FILE, 0, (FcChar8**)&pathname) != FcResultMatch)
        return names;
    int index;
    if (FcPatternGetInteger(pat, FC_INDEX, 0, &index) != FcResultMatch)
        return names;

    FT_Face face;
    if (FT_New_Face(gFreeTypeLibrary, pathname, index, &face) != 0)
        return names;

    const char* name = FT_Get_Postscript_Name(face);
    if (name == NULL)
        return names;
    names->m_psName = name;

    // for sfnt containers, we'll read the name table ourselves, not rely on Fontconfig
    if (FT_IS_SFNT(face)) {
        unsigned int i;
        std::list<std::string>  familyNames;
        std::list<std::string>  subFamilyNames;
        FT_SfntName nameRec;
        for (i = 0; i < FT_Get_Sfnt_Name_Count(face); ++i) {
            char* utf8name = NULL;
            if (FT_Get_Sfnt_Name(face, i, &nameRec) != 0)
                continue;
            switch (nameRec.name_id) {
                case kFontFullName:
                case kFontFamilyName:
                case kFontStyleName:
                case kPreferredFamilyName:
                case kPreferredSubfamilyName:
                    {
                        bool preferredName = false;
                        /* Tectonic: macRomanConv may not be available; see comment below */
                        if (macRomanConv != NULL && nameRec.platform_id == TT_PLATFORM_MACINTOSH
                                && nameRec.encoding_id == TT_MAC_ID_ROMAN && nameRec.language_id == 0) {
                            utf8name = convertToUtf8(macRomanConv, nameRec.string, nameRec.string_len);
                            preferredName = true;
                        }
                        else if ((nameRec.platform_id == TT_PLATFORM_APPLE_UNICODE)
                                || (nameRec.platform_id == TT_PLATFORM_MICROSOFT))
                            utf8name = convertToUtf8(utf16beConv, nameRec.string, nameRec.string_len);

                        if (utf8name != NULL) {
                            std::list<std::string>* nameList = NULL;
                            switch (nameRec.name_id) {
                                case kFontFullName:
                                    nameList = &names->m_fullNames;
                                    break;
                                case kFontFamilyName:
                                    nameList = &names->m_familyNames;
                                    break;
                                case kFontStyleName:
                                    nameList = &names->m_styleNames;
                                    break;
                                case kPreferredFamilyName:
                                    nameList = &familyNames;
                                    break;
                                case kPreferredSubfamilyName:
                                    nameList = &subFamilyNames;
                                    break;
                            }
                            if (preferredName)
                                prependToList(nameList, utf8name);
                            else
                                appendToList(nameList, utf8name);
                            delete[] utf8name;
                        }
                    }
                    break;
            }
        }
        if (familyNames.size() > 0)
            names->m_familyNames = familyNames;
        if (subFamilyNames.size() > 0)
            names->m_styleNames = subFamilyNames;
    } else {
        index = 0;
        while (FcPatternGetString(pat, FC_FULLNAME, index++, (FcChar8**)&name) == FcResultMatch)
            appendToList(&names->m_fullNames, name);
        index = 0;
        while (FcPatternGetString(pat, FC_FAMILY, index++, (FcChar8**)&name) == FcResultMatch)
            appendToList(&names->m_familyNames, name);
        index = 0;
        while (FcPatternGetString(pat, FC_STYLE, index++, (FcChar8**)&name) == FcResultMatch)
            appendToList(&names->m_styleNames, name);

        if (names->m_fullNames.size() == 0) {
            std::string fullName(names->m_familyNames.front());
            if (names->m_styleNames.size() > 0) {
                fullName += " ";
                fullName += names->m_styleNames.front();
            }
            names->m_fullNames.push_back(fullName);
        }
    }

    FT_Done_Face(face);

    return names;
}

void
XeTeXFontMgr_FC::getOpSizeRecAndStyleFlags(Font* theFont)
{
    XeTeXFontMgr::getOpSizeRecAndStyleFlags(theFont);

    if (theFont->weight == 0 && theFont->width == 0) {
        // try to get values from FontConfig, as it apparently wasn't an sfnt
        FcPattern* pat = theFont->fontRef;
        int value;
        if (FcPatternGetInteger(pat, FC_WEIGHT, 0, &value) == FcResultMatch)
            theFont->weight = value;
        if (FcPatternGetInteger(pat, FC_WIDTH, 0, &value) == FcResultMatch)
            theFont->width = value;
        if (FcPatternGetInteger(pat, FC_SLANT, 0, &value) == FcResultMatch)
            theFont->slant = value;
    }
}

void
XeTeXFontMgr_FC::cacheFamilyMembers(const std::list<std::string>& familyNames)
{
    if (familyNames.size() == 0)
        return;
    for (int f = 0; f < allFonts->nfont; ++f) {
        FcPattern* pat = allFonts->fonts[f];
        if (m_platformRefToFont.find(pat) != m_platformRefToFont.end())
            continue;
        char* s;
        for (int i = 0; FcPatternGetString(pat, FC_FAMILY, i, (FcChar8**)&s) == FcResultMatch; ++i) {
            for (std::list<std::string>::const_iterator j = familyNames.begin(); j != familyNames.end(); ++j) {
                if (*j == s) {
                    NameCollection* names = readNames(pat);
                    addToMaps(pat, names);
                    delete names;
                    goto cached;
                }
            }
        }
    cached:
        ;
    }
}

void
XeTeXFontMgr_FC::searchForHostPlatformFonts(const std::string& name)
{
    if (cachedAll) // we've already loaded everything on an earlier search
        return;

    std::string famName;
    int hyph = name.find('-');
    if (hyph > 0 && hyph < (int) (name.length() - 1))
        famName.assign(name.begin(), name.begin() + hyph);
    else
        hyph = 0;

    bool found = false;
    while (1) {
        for (int f = 0; f < allFonts->nfont; ++f) {
            FcPattern* pat = allFonts->fonts[f];
            if (m_platformRefToFont.find(pat) != m_platformRefToFont.end())
                continue;

            if (cachedAll) {
                // failed to find it via FC; add everything to our maps (potentially slow) as a last resort
                NameCollection* names = readNames(pat);
                addToMaps(pat, names);
                delete names;
                continue;
            }

            char* s;
            int i;
            for (i = 0; FcPatternGetString(pat, FC_FULLNAME, i, (FcChar8**)&s) == FcResultMatch; ++i) {
                if (name == s) {
                    NameCollection* names = readNames(pat);
                    addToMaps(pat, names);
                    cacheFamilyMembers(names->m_familyNames);
                    delete names;
                    found = true;
                    goto next_font;
                }
            }

            for (i = 0; FcPatternGetString(pat, FC_FAMILY, i, (FcChar8**)&s) == FcResultMatch; ++i) {
                if (name == s || (hyph && famName == s)) {
                    NameCollection* names = readNames(pat);
                    addToMaps(pat, names);
                    cacheFamilyMembers(names->m_familyNames);
                    delete names;
                    found = true;
                    goto next_font;
                }
                char* t;
                for (int j = 0; FcPatternGetString(pat, FC_STYLE, j, (FcChar8**)&t) == FcResultMatch; ++j) {
                    std::string full(s);
                    full += " ";
                    full += t;
                    if (name == full) {
                        NameCollection* names = readNames(pat);
                        addToMaps(pat, names);
                        cacheFamilyMembers(names->m_familyNames);
                        delete names;
                        found = true;
                        goto next_font;
                    }
                }
            }

        next_font:
            ;
        }

        if (found || cachedAll)
            break;
        cachedAll = true;
    }
}

void
XeTeXFontMgr_FC::initialize()
{
    if (FcInit() == FcFalse)
        _tt_abort("fontconfig initialization failed");

    if (gFreeTypeLibrary == 0 && FT_Init_FreeType(&gFreeTypeLibrary) != 0)
        _tt_abort("FreeType initialization failed");

    UErrorCode err = U_ZERO_ERROR;

    /* Tectonic: Alpine >=3.16 splits the ICU data in a way that seems to make
     * the "macintosh" converter unavailable in our MUSL static builds. I don't
     * see a workaround: installing `icu-data-full` doesn't help, I think
     * because the static libraries don't access the data file that it provides.
     */
    macRomanConv = ucnv_open("macintosh", &err);
    if (!U_SUCCESS(err)) {
        err = U_ZERO_ERROR;
        macRomanConv = NULL;
    }

    utf16beConv = ucnv_open("UTF16BE", &err);
    utf8Conv = ucnv_open("UTF8", &err);
    if (!U_SUCCESS(err))
        _tt_abort("cannot read font names");

    FcPattern* pat = FcNameParse((const FcChar8*)":outline=true");
    FcObjectSet* os = FcObjectSetBuild(FC_FAMILY, FC_STYLE, FC_FILE, FC_INDEX,
                                       FC_FULLNAME, FC_WEIGHT, FC_WIDTH, FC_SLANT, FC_FONTFORMAT, NULL);
    allFonts = FcFontList(FcConfigGetCurrent(), pat, os);
    FcObjectSetDestroy(os);
    FcPatternDestroy(pat);

    cachedAll = false;
}

void
XeTeXFontMgr_FC::terminate()
{
    if (allFonts != NULL) {
        FcFontSetDestroy(allFonts);
        allFonts = NULL;
    }

    if (macRomanConv != NULL) {
        ucnv_close(macRomanConv);
        macRomanConv = NULL;
    }
    if (utf16beConv != NULL) {
        ucnv_close(utf16beConv);
        utf16beConv = NULL;
    }
    if (utf8Conv != NULL) {
        ucnv_close(utf8Conv);
        utf8Conv = NULL;
    }
}

std::string
XeTeXFontMgr_FC::getPlatformFontDesc(PlatformFontRef font) const
{
    std::string path;
    FcChar8* s;
    if (FcPatternGetString(font, FC_FILE, 0, (FcChar8**)&s) == FcResultMatch)
        path = (char*)s;
    else
        path = "[unknown]";
    return path;
}
