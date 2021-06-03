/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009-2014 by Jonathan Kew

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

#include "tectonic_xetex_layout.h"

#ifdef XETEX_MAC
#include "xetex-XeTeXFontMgr_Mac.h"
#else
#include "xetex-XeTeXFontMgr_FC.h"
#endif
#include "xetex-XeTeXFontInst.h"

#include <harfbuzz/hb-ot.h>

// see cpascal.h
#define printcstring(STR)        \
  do {                           \
    const char* ch_ptr = (STR);  \
    while (*ch_ptr)              \
      print_char(*(ch_ptr++));    \
  } while (0)

XeTeXFontMgr* XeTeXFontMgr::sFontManager = NULL;
char XeTeXFontMgr::sReqEngine = 0;
Fixed loaded_font_design_size = 0; /* explicitly *not* static */

/* use our own fmax function because it seems to be missing on certain platforms
   (solaris2.9, at least) */
static inline double
my_fmax(double x, double y)
{
    return (x > y) ? x : y;
}

XeTeXFontMgr*
XeTeXFontMgr::GetFontManager()
{
    if (sFontManager == NULL) {
#ifdef XETEX_MAC
        sFontManager = new XeTeXFontMgr_Mac;
#else
        sFontManager = new XeTeXFontMgr_FC;
#endif
        sFontManager->initialize();
    }

    return sFontManager;
}

void
XeTeXFontMgr::Terminate()
{
    if (sFontManager != NULL) {
        sFontManager->terminate();
        // we don't actually deallocate the manager, just ask it to clean up
        // any auxiliary data such as the cocoa pool or freetype/fontconfig stuff
        // as we still need to access font names after this is called
    }
}

void
XeTeXFontMgr::Destroy()
{
    // Here we actually fully destroy the font manager.

    if (sFontManager != NULL) {
        delete sFontManager;
        sFontManager = NULL;
    }
}

PlatformFontRef
XeTeXFontMgr::findFont(const char* name, char* variant, double ptSize)
    // ptSize is in TeX points, or negative for 'scaled' factor
    // "variant" string will be shortened (in-place) by removal of /B and /I if present
{
    std::string nameStr(name);
    Font* font = NULL;
    double dsize = 10.0;
    loaded_font_design_size = 655360L;

    for (int pass = 0; pass < 2; ++pass) {
        // try full name as given
        std::map<std::string,Font*>::iterator i = m_nameToFont.find(nameStr);
        if (i != m_nameToFont.end()) {
            font = i->second;
            if (font->opSizeInfo.designSize != 0.0)
                dsize = font->opSizeInfo.designSize;
            break;
        }

        // if there's a hyphen, split there and try Family-Style
        int hyph = nameStr.find('-');
        if (hyph > 0 && hyph < (int) (nameStr.length() - 1)) {
            std::string family(nameStr.begin(), nameStr.begin() + hyph);
            std::map<std::string,Family*>::iterator f = m_nameToFamily.find(family);
            if (f != m_nameToFamily.end()) {
                std::string style(nameStr.begin() + hyph + 1, nameStr.end());
                i = f->second->styles->find(style);
                if (i != f->second->styles->end()) {
                    font = i->second;
                    if (font->opSizeInfo.designSize != 0.0)
                        dsize = font->opSizeInfo.designSize;
                    break;
                }
            }
        }

        // try as PostScript name
        i = m_psNameToFont.find(nameStr);
        if (i != m_psNameToFont.end()) {
            font = i->second;
            if (font->opSizeInfo.designSize != 0.0)
                dsize = font->opSizeInfo.designSize;
            break;
        }

        // try for the name as a family name
        std::map<std::string,Family*>::iterator f = m_nameToFamily.find(nameStr);

        if (f != m_nameToFamily.end()) {
            // look for a family member with the "regular" bit set in OS/2
            int regFonts = 0;
            for (i = f->second->styles->begin(); i != f->second->styles->end(); ++i)
                if (i->second->isReg) {
                    if (regFonts == 0)
                        font = i->second;
                    ++regFonts;
                }

            // families with Ornament or similar fonts may flag those as Regular,
            // which confuses the search above... so try some known names
            if (font == NULL || regFonts > 1) {
                // try for style "Regular", "Plain", "Normal", "Roman"
                i = f->second->styles->find("Regular");
                if (i != f->second->styles->end())
                    font = i->second;
                else {
                    i = f->second->styles->find("Plain");
                    if (i != f->second->styles->end())
                        font = i->second;
                    else {
                        i = f->second->styles->find("Normal");
                        if (i != f->second->styles->end())
                            font = i->second;
                        else {
                            i = f->second->styles->find("Roman");
                            if (i != f->second->styles->end())
                                font = i->second;
                        }
                    }
                }
            }

            if (font == NULL) {
                // look through the family for the (weight, width, slant) nearest to (80, 100, 0)
                font = bestMatchFromFamily(f->second, 80, 100, 0);
            }

            if (font != NULL)
                break;
        }

        if (pass == 0) {
            // didn't find it in our caches, so do a platform search (may be relatively expensive);
            // this will update the caches with any fonts that seem to match the name given,
            // so that the second pass might find it
            searchForHostPlatformFonts(nameStr);
        }
    }

    if (font == NULL)
        return 0;

    Family* parent = font->parent;

    // if there are variant requests, try to apply them
    // and delete B, I, and S=... codes from the string, just retain /engine option
    sReqEngine = 0;
    bool reqBold = false;
    bool reqItal = false;
    if (variant != NULL) {
        std::string varString;
        char* cp = variant;
        while (*cp) {
            if (strncmp(cp, "AAT", 3) == 0) {
                sReqEngine = 'A';
                cp += 3;
                if (varString.length() > 0 && *(varString.end() - 1) != '/')
                    varString.append("/");
                varString.append("AAT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "ICU", 3) == 0) { // for backword compatability
                sReqEngine = 'O';
                cp += 3;
                if (varString.length() > 0 && *(varString.end() - 1) != '/')
                    varString.append("/");
                varString.append("OT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "OT", 2) == 0) {
                sReqEngine = 'O';
                cp += 2;
                if (varString.length() > 0 && *(varString.end() - 1) != '/')
                    varString.append("/");
                varString.append("OT");
                goto skip_to_slash;
            }
            if (strncmp(cp, "GR", 2) == 0) {
                sReqEngine = 'G';
                cp += 2;
                if (varString.length() > 0 && *(varString.end() - 1) != '/')
                    varString.append("/");
                varString.append("GR");
                goto skip_to_slash;
            }
            if (*cp == 'S') {
                ++cp;
                if (*cp == '=')
                    ++cp;
                ptSize = 0.0;
                while (*cp >= '0' && *cp <= '9') {
                    ptSize = ptSize * 10 + *cp - '0';
                    ++cp;
                }
                if (*cp == '.') {
                    double dec = 1.0;
                    ++cp;
                    while (*cp >= '0' && *cp <= '9') {
                        dec = dec * 10.0;
                        ptSize = ptSize + (*cp - '0') / dec;
                        ++cp;
                    }
                }
                goto skip_to_slash;
            }

            /* if the code is "B" or "I", we skip putting it in varString */
            while (1) {
                if (*cp == 'B') {
                    reqBold = true;
                    ++cp;
                    continue;
                }
                if (*cp == 'I') {
                    reqItal = true;
                    ++cp;
                    continue;
                }
                break;
            }

        skip_to_slash:
            while (*cp && *cp != '/')
                ++cp;
            if (*cp == '/')
                ++cp;
        }
        strcpy(variant, varString.c_str());

        std::map<std::string,Font*>::iterator i;
        if (reqItal) {
            Font* bestMatch = font;
            if (font->slant < parent->maxSlant)
                // try for a face with more slant
                bestMatch = bestMatchFromFamily(parent, font->weight, font->width, parent->maxSlant);

            if (bestMatch == font && font->slant > parent->minSlant)
                // maybe the slant is negated, or maybe this was something like "Times-Italic/I"
                bestMatch = bestMatchFromFamily(parent, font->weight, font->width, parent->minSlant);

            if (parent->minWeight == parent->maxWeight && bestMatch->isBold != font->isBold) {
                // try again using the bold flag, as we can't trust weight values
                Font* newBest = NULL;
                for (i = parent->styles->begin(); i != parent->styles->end(); ++i) {
                    if (i->second->isBold == font->isBold) {
                        if (newBest == NULL && i->second->isItalic != font->isItalic) {
                            newBest = i->second;
                            break;
                        }
                    }
                }
                if (newBest != NULL)
                    bestMatch = newBest;
            }

            if (bestMatch == font) {
                // maybe slant values weren't present; try the style bits as a fallback
                bestMatch = NULL;
                for (i = parent->styles->begin(); i != parent->styles->end(); ++i) {
                    if (i->second->isItalic == !font->isItalic) {
                        if (parent->minWeight != parent->maxWeight) {
                            // weight info was available, so try to match that
                            if (bestMatch == NULL || weightAndWidthDiff(i->second, font) < weightAndWidthDiff(bestMatch, font))
                                bestMatch = i->second;
                        } else {
                            // no weight info, so try matching style bits
                            if (bestMatch == NULL && i->second->isBold == font->isBold) {
                                bestMatch = i->second;
                                break;  // found a match, no need to look further as we can't distinguish!
                            }
                        }
                    }
                }
            }
            if (bestMatch != NULL)
                font = bestMatch;
        }

        if (reqBold) {
            // try for more boldness, with the same width and slant
            Font* bestMatch = font;
            if (font->weight < parent->maxWeight) {
                // try to increase weight by 1/2 x (max - min), rounding up
                bestMatch = bestMatchFromFamily(parent,
                    font->weight + (parent->maxWeight - parent->minWeight) / 2 + 1,
                    font->width, font->slant);
                if (parent->minSlant == parent->maxSlant) {
                    // double-check the italic flag, as we can't trust slant values
                    Font* newBest = NULL;
                    for (i = parent->styles->begin(); i != parent->styles->end(); ++i) {
                        if (i->second->isItalic == font->isItalic) {
                            if (newBest == NULL || weightAndWidthDiff(i->second, bestMatch) < weightAndWidthDiff(newBest, bestMatch))
                                newBest = i->second;
                        }
                    }
                    if (newBest != NULL)
                        bestMatch = newBest;
                }
            }
            if (bestMatch == font && !font->isBold) {
                for (i = parent->styles->begin(); i != parent->styles->end(); ++i) {
                    if (i->second->isItalic == font->isItalic && i->second->isBold) {
                        bestMatch = i->second;
                        break;
                    }
                }
            }
            font = bestMatch;
        }
    }

    // if there's optical size info, try to apply it
    if (ptSize < 0.0)
        ptSize = dsize;
    if (font != NULL && font->opSizeInfo.subFamilyID != 0 && ptSize > 0.0) {
        double bestMismatch = my_fmax(font->opSizeInfo.minSize - ptSize, ptSize - font->opSizeInfo.maxSize);
        if (bestMismatch > 0.0) {
            Font* bestMatch = font;
            for (std::map<std::string,Font*>::iterator i = parent->styles->begin(); i != parent->styles->end(); ++i) {
                if (i->second->opSizeInfo.subFamilyID != font->opSizeInfo.subFamilyID)
                    continue;
                double mismatch = my_fmax(i->second->opSizeInfo.minSize - ptSize, ptSize - i->second->opSizeInfo.maxSize);
                if (mismatch < bestMismatch) {
                    bestMatch = i->second;
                    bestMismatch = mismatch;
                }
                if (bestMismatch <= 0.0)
                    break;
            }
            font = bestMatch;
        }
    }

    if (font != NULL && font->opSizeInfo.designSize != 0.0)
        loaded_font_design_size = unsigned(font->opSizeInfo.designSize * 65536.0 + 0.5);

    /* Tectonic: there used to be a bit of tracing code here, but we neede to
     * move it to find_native_font() to preserve encapsulation. */

    return font->fontRef;
}

const char*
XeTeXFontMgr::getFullName(PlatformFontRef font) const
{
    std::map<PlatformFontRef,Font*>::const_iterator i = m_platformRefToFont.find(font);
    if (i == m_platformRefToFont.end())
        _tt_abort("internal error %d in XeTeXFontMgr", 2);
    if (i->second->m_fullName != NULL)
        return i->second->m_fullName->c_str();
    else
        return i->second->m_psName->c_str();
}

int
XeTeXFontMgr::weightAndWidthDiff(const Font* a, const Font* b) const
{
    if (a->weight == 0 && a->width == 0) {
        // assume there was no OS/2 info
        if (a->isBold == b->isBold)
            return 0;
        else
            return 10000;
    }

    int widDiff = labs(a->width - b->width);
    if (widDiff < 10)
        widDiff *= 50;

    return labs(a->weight - b->weight) + widDiff;
}

int
XeTeXFontMgr::styleDiff(const Font* a, int wt, int wd, int slant) const
{
    int widDiff = labs(a->width - wd);
    if (widDiff < 10)
        widDiff *= 200;

    return labs(labs(a->slant) - labs(slant)) * 2 + labs(a->weight - wt) + widDiff;
}

XeTeXFontMgr::Font*
XeTeXFontMgr::bestMatchFromFamily(const Family* fam, int wt, int wd, int slant) const
{
    Font* bestMatch = NULL;
    for (std::map<std::string,Font*>::iterator s = fam->styles->begin(); s != fam->styles->end(); ++s)
        if (bestMatch == NULL || styleDiff(s->second, wt, wd, slant) < styleDiff(bestMatch, wt, wd, slant))
            bestMatch = s->second;
    return bestMatch;
}


XeTeXFontMgr::OpSizeRec*
XeTeXFontMgr::getOpSize(XeTeXFont font)
{
    hb_font_t *hbFont = ((XeTeXFontInst *) font)->getHbFont();

    if (hbFont == NULL)
        return NULL;

    hb_face_t *face = hb_font_get_face(hbFont);
    OpSizeRec *pSizeRec = (OpSizeRec*) xmalloc(sizeof(OpSizeRec));

    unsigned int designSize, minSize, maxSize;
    bool ok = hb_ot_layout_get_size_params(face,
                                           &designSize,
                                           &pSizeRec->subFamilyID,
                                           &pSizeRec->nameCode,
                                           &minSize,
                                           &maxSize);

    if (ok) {
        // Convert sizes from PostScript deci-points to TeX points
        pSizeRec->designSize = designSize * 72.27 / 72.0 / 10.0;
        pSizeRec->minSize = minSize * 72.27 / 72.0 / 10.0;
        pSizeRec->maxSize = maxSize * 72.27 / 72.0 / 10.0;
        return pSizeRec;
    }

    free(pSizeRec);
    return NULL;
}


double
XeTeXFontMgr::getDesignSize(XeTeXFont font)
{
    OpSizeRec* pSizeRec = getOpSize(font);

    if (pSizeRec == NULL)
        return 10.0;

    /* Tectonic: make sure not to leak pSizeRec */
    double result = pSizeRec->designSize;
    free(pSizeRec);
    return result;
}


void
XeTeXFontMgr::getOpSizeRecAndStyleFlags(Font* theFont)
{
    XeTeXFont font = createFont(theFont->fontRef, 655360);
    XeTeXFontInst* fontInst = (XeTeXFontInst*) font;
    if (font != 0) {
        OpSizeRec* pSizeRec = getOpSize(font);

        if (pSizeRec != NULL) {
            theFont->opSizeInfo.designSize = pSizeRec->designSize;
            if (pSizeRec->subFamilyID == 0
                && pSizeRec->nameCode == 0
                && pSizeRec->minSize == 0.0
                && pSizeRec->maxSize == 0.0) {
                /* Tectonic: make sure not to leak pSizeRec */
                free(pSizeRec);
                goto done_size; // feature is valid, but no 'size' range
            }

            theFont->opSizeInfo.subFamilyID = pSizeRec->subFamilyID;
            theFont->opSizeInfo.nameCode = pSizeRec->nameCode;
            theFont->opSizeInfo.minSize = pSizeRec->minSize;
            theFont->opSizeInfo.maxSize = pSizeRec->maxSize;
            free(pSizeRec);
        }

    done_size:

        const TT_OS2* os2Table = (TT_OS2*) fontInst->getFontTable(ft_sfnt_os2);
        if (os2Table != NULL) {
            theFont->weight = os2Table->usWeightClass;
            theFont->width = os2Table->usWidthClass;
            uint16_t sel = os2Table->fsSelection;
            theFont->isReg = (sel & (1 << 6)) != 0;
            theFont->isBold = (sel & (1 << 5)) != 0;
            theFont->isItalic = (sel & (1 << 0)) != 0;
        }

        const TT_Header* headTable = (TT_Header*) fontInst->getFontTable(ft_sfnt_head);
        if (headTable != NULL) {
            uint16_t ms = headTable->Mac_Style;
            if ((ms & (1 << 0)) != 0)
                theFont->isBold = true;
            if ((ms & (1 << 1)) != 0)
                theFont->isItalic = true;
        }

        const TT_Postscript* postTable = (const TT_Postscript*) fontInst->getFontTable(ft_sfnt_post);
        if (postTable != NULL) {
            theFont->slant = (int)(1000 * (tan(Fix2D(-postTable->italicAngle) * M_PI / 180.0)));
        }
        deleteFont(font);
    }
}

// append a name but only if it's not already in the list
void
XeTeXFontMgr::appendToList(std::list<std::string>* list, const char* str)
{
    for (std::list<std::string>::const_iterator i = list->begin(); i != list->end(); ++i)
        if (*i == str)
            return;
    list->push_back(str);
}

// prepend a name, removing it from later in the list if present
void
XeTeXFontMgr::prependToList(std::list<std::string>* list, const char* str)
{
    for (std::list<std::string>::iterator i = list->begin(); i != list->end(); ++i)
        if (*i == str) {
            list->erase(i);
            break;
        }
    list->push_front(str);
}

void
XeTeXFontMgr::addToMaps(PlatformFontRef platformFont, const NameCollection* names)
{
    if (m_platformRefToFont.find(platformFont) != m_platformRefToFont.end())
        return; // this font has already been cached

    if (names->m_psName.length() == 0)
        return; // can't use a font that lacks a PostScript name

    if (m_psNameToFont.find(names->m_psName) != m_psNameToFont.end())
        return; // duplicates an earlier PS name, so skip

    Font* thisFont = new Font(platformFont);
    thisFont->m_psName = new std::string(names->m_psName);
    getOpSizeRecAndStyleFlags(thisFont);

    m_psNameToFont[names->m_psName] = thisFont;
    m_platformRefToFont[platformFont] = thisFont;

    if (names->m_fullNames.size() > 0)
        thisFont->m_fullName = new std::string(*(names->m_fullNames.begin()));

    if (names->m_familyNames.size() > 0)
        thisFont->m_familyName = new std::string(*(names->m_familyNames.begin()));
    else
        thisFont->m_familyName = new std::string(names->m_psName);

    if (names->m_styleNames.size() > 0)
        thisFont->m_styleName = new std::string(*(names->m_styleNames.begin()));
    else
        thisFont->m_styleName = new std::string;

    std::list<std::string>::const_iterator i;
    for (i = names->m_familyNames.begin(); i != names->m_familyNames.end(); ++i) {
        std::map<std::string,Family*>::iterator iFam = m_nameToFamily.find(*i);
        Family* family;
        if (iFam == m_nameToFamily.end()) {
            family = new Family;
            m_nameToFamily[*i] = family;
            family->minWeight = thisFont->weight;
            family->maxWeight = thisFont->weight;
            family->minWidth = thisFont->width;
            family->maxWidth = thisFont->width;
            family->minSlant = thisFont->slant;
            family->maxSlant = thisFont->slant;
        } else {
            family = iFam->second;
            if (thisFont->weight < family->minWeight)
                family->minWeight = thisFont->weight;
            if (thisFont->weight > family->maxWeight)
                family->maxWeight = thisFont->weight;
            if (thisFont->width < family->minWidth)
                family->minWidth = thisFont->width;
            if (thisFont->width > family->maxWidth)
                family->maxWidth = thisFont->width;
            if (thisFont->slant < family->minSlant)
                family->minSlant = thisFont->slant;
            if (thisFont->slant > family->maxSlant)
                family->maxSlant = thisFont->slant;
        }

        if (thisFont->parent == NULL)
            thisFont->parent = family;

        // ensure all style names in the family point to thisFont
        for (std::list<std::string>::const_iterator j = names->m_styleNames.begin(); j != names->m_styleNames.end(); ++j) {
            std::map<std::string,Font*>::iterator iFont = family->styles->find(*j);
            if (iFont == family->styles->end())
                (*family->styles)[*j] = thisFont;
/*
            else if (iFont->second != thisFont)
                fprintf(stderr, "# Font name warning: ambiguous Style \"%s\" in Family \"%s\" (PSNames \"%s\" and \"%s\")\n",
                            j->c_str(), i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
*/
        }
    }

    for (i = names->m_fullNames.begin(); i != names->m_fullNames.end(); ++i) {
        std::map<std::string,Font*>::iterator iFont = m_nameToFont.find(*i);
        if (iFont == m_nameToFont.end())
            m_nameToFont[*i] = thisFont;
/*
        else if (iFont->second != thisFont)
            fprintf(stderr, "# Font name warning: ambiguous FullName \"%s\" (PSNames \"%s\" and \"%s\")\n",
                        i->c_str(), iFont->second->m_psName->c_str(), thisFont->m_psName->c_str());
*/
    }
}

void
XeTeXFontMgr::terminate()
{
}
