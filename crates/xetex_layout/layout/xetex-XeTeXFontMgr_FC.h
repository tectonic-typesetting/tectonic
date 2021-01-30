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

#ifndef __XETEX_FONT_MGR_FC_H
#define __XETEX_FONT_MGR_FC_H

#include "tectonic_bridge_core.h"
#include "xetex-XeTeXFontMgr.h"

class XeTeXFontMgr_FC
    : public XeTeXFontMgr
{
public:
                                    XeTeXFontMgr_FC()
                                        { }
    virtual                         ~XeTeXFontMgr_FC()
                                        { }

protected:

    virtual void                    initialize();
    virtual void                    terminate();

    virtual void                    getOpSizeRecAndStyleFlags(Font* theFont);
    virtual void                    searchForHostPlatformFonts(const std::string& name);

    virtual NameCollection*         readNames(FcPattern* pat);

    std::string                     getPlatformFontDesc(PlatformFontRef font) const;

    void                            cacheFamilyMembers(const std::list<std::string>& familyNames);

    FcFontSet*  allFonts;
    bool        cachedAll;
};

#endif  /* __XETEX_FONT_MGR_FC_H */
