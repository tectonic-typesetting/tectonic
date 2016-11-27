/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012, 2013 by Jiang Jiang

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

#ifndef __XETEX_FONT_MANAGER_H
#define __XETEX_FONT_MANAGER_H

#ifdef XETEX_MAC
#include <ApplicationServices/ApplicationServices.h>
typedef CTFontDescriptorRef PlatformFontRef;
#else
#include <fontconfig/fontconfig.h>
#include <ft2build.h>
#include FT_FREETYPE_H
typedef FcPattern* PlatformFontRef;
#endif

#include "XeTeX_ext.h"

#include "XeTeXLayoutInterface.h"

#ifdef __cplusplus  /* allow inclusion in plain C files just to get the typedefs above */

#include <string>
#include <map>
#include <list>
#include <vector>

class XeTeXFontMgr
{
public:
    static XeTeXFontMgr*            GetFontManager();
        // returns the global fontmanager (creating it if necessary)
    static void                     Terminate();
        // clean up (may be required if using the cocoa implementation)

    PlatformFontRef                 findFont(const char* name, char* variant, double ptSize);
        // 1st arg is name as specified by user (C string, UTF-8)
        // 2nd is /B/I/AAT/OT/ICU/GR/S=## qualifiers
        // 1. try name given as "full name"
        // 2. if there's a hyphen, split and try "family-style"
        // 3. try as PostScript name
        // 4. try name as family with "Regular/Plain/Normal" style
        // apply style qualifiers and optical sizing if present

        // SIDE EFFECT: sets sReqEngine to 'A' or 'O' or 'G' if appropriate,
        //   else clears it to 0

        // SIDE EFFECT: updates TeX variables /nameoffile/ and /namelength/,
        //   to match the actual font found

        // SIDE EFFECT: edits /variant/ string in-place removing /B or /I

    const char*                     getFullName(PlatformFontRef font) const;
        // return the full name of the font, suitable for use in XeTeX source
        // without requiring style qualifiers

    double                          getDesignSize(XeTeXFont font);

    char                            getReqEngine() const { return sReqEngine; };
        // return the requested rendering technology for the most recent findFont
        // or 0 if no specific technology was requested

    void                            setReqEngine(char reqEngine) const { sReqEngine = reqEngine; };

protected:
    static XeTeXFontMgr*            sFontManager;
    static char                     sReqEngine;

                                    XeTeXFontMgr()
                                        { }
    virtual                         ~XeTeXFontMgr()
                                        { }

    virtual void                    initialize() = 0;
    virtual void                    terminate();

    virtual std::string             getPlatformFontDesc(PlatformFontRef font) const = 0;

    class Font;
    class Family;

    struct OpSizeRec {
        unsigned int    designSize;
        unsigned int    subFamilyID;
        unsigned int    nameCode;
        unsigned int    minSize;
        unsigned int    maxSize;
    };

    class Font {
        public:
                            Font(PlatformFontRef ref)
                                : m_fullName(NULL), m_psName(NULL), m_familyName(NULL), m_styleName(NULL)
                                , parent(NULL)
                                , fontRef(ref), weight(0), width(0), slant(0)
                                , isReg(false), isBold(false), isItalic(false)
                                { opSizeInfo.subFamilyID = 0;
                                  opSizeInfo.designSize = 100; } /* default to 10bp */
                            ~Font()
                                { delete m_fullName; delete m_psName; }

            std::string*    m_fullName;
            std::string*    m_psName;
            std::string*    m_familyName; // default family and style names that should locate this font
            std::string*    m_styleName;
            Family*         parent;
            PlatformFontRef fontRef;
            OpSizeRec       opSizeInfo;
            uint16_t        weight;
            uint16_t        width;
            int16_t         slant;
            bool            isReg;
            bool            isBold;
            bool            isItalic;
    };

    class Family {
        public:
                                            Family()
                                                : minWeight(0), maxWeight(0)
                                                , minWidth(0), maxWidth(0)
                                                , minSlant(0), maxSlant(0)
                                                {
                                                    styles = new std::map<std::string,Font*>;
                                                }
                                            ~Family()
                                                {
                                                    delete styles;
                                                }

            std::map<std::string,Font*>*    styles;
            uint16_t                        minWeight;
            uint16_t                        maxWeight;
            uint16_t                        minWidth;
            uint16_t                        maxWidth;
            int16_t                         minSlant;
            int16_t                         maxSlant;
    };

    class NameCollection {
    public:
        std::list<std::string>  m_familyNames;
        std::list<std::string>  m_styleNames;
        std::list<std::string>  m_fullNames;
        std::string             m_psName;
        std::string             m_subFamily;
    };

    std::map<std::string,Font*>                 m_nameToFont;                     // maps full name (as used in TeX source) to font record
    std::map<std::string,Family*>               m_nameToFamily;
    std::map<PlatformFontRef,Font*>             m_platformRefToFont;
    std::map<std::string,Font*>                 m_psNameToFont;                   // maps PS name (as used in .xdv) to font record

    int             weightAndWidthDiff(const Font* a, const Font* b) const;
    int             styleDiff(const Font* a, int wt, int wd, int slant) const;
    Font*           bestMatchFromFamily(const Family* fam, int wt, int wd, int slant) const;
    void            appendToList(std::list<std::string>* list, const char* str);
    void            prependToList(std::list<std::string>* list, const char* str);
    void            addToMaps(PlatformFontRef platformFont, const NameCollection* names);

    const OpSizeRec* getOpSize(XeTeXFont font);

    virtual void    getOpSizeRecAndStyleFlags(Font* theFont);
    virtual void    searchForHostPlatformFonts(const std::string& name) = 0;

    virtual NameCollection*     readNames(PlatformFontRef fontRef) = 0;

    void    die(const char*s, int i) const; /* for fatal internal errors! */
};

#endif  /* __cplusplus */


#endif  /* __XETEX_FONT_MANAGER_H */
