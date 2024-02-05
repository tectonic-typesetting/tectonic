/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009-2012 by Jonathan Kew
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

#include "tectonic_bridge_core.h"

#include <unicode/ubidi.h>  /* Barely needed in this file. */

#include <graphite2/Font.h>
#include <graphite2/Segment.h>
#include <harfbuzz/hb.h>
#include <harfbuzz/hb-graphite2.h>
#if !HB_VERSION_ATLEAST(2,5,0)
/* Note: this configuration is no longer actively tested */
#include <harfbuzz/hb-icu.h>
#endif
#include <harfbuzz/hb-ot.h>

#include "tectonic_xetex_layout.h"

#include "xetex-XeTeXFontInst.h"
#ifdef XETEX_MAC
#include "xetex-XeTeXFontInst_Mac.h"
#endif
#include "xetex-XeTeXFontMgr.h"

#include <iostream>

/*******************************************************************/
/* Glyph bounding box cache to speed up \XeTeXuseglyphmetrics mode */
/*******************************************************************/

/*******************************************************************/

//void
//terminate_font_manager()
//{
//    XeTeXFontMgr::Terminate();
//}
//
//void
//destroy_font_manager()
//{
//    XeTeXFontMgr::Destroy();
//}
//
//PlatformFontRef
//findFontByName(const char* name, char* var, double size)
//{
//    return XeTeXFontMgr::GetFontManager()->findFont(name, var, size);
//}
//
//char
//getReqEngine()
//{
//    return XeTeXFontMgr::GetFontManager()->getReqEngine();
//}
//
//void
//setReqEngine(char reqEngine)
//{
//    XeTeXFontMgr::GetFontManager()->setReqEngine(reqEngine);
//}
//
//const char*
//getFullName(PlatformFontRef fontRef)
//{
//    return XeTeXFontMgr::GetFontManager()->getFullName(fontRef);
//}
//
//double
//getDesignSize(XeTeXFont font)
//{
//    return XeTeXFontMgr::GetFontManager()->getDesignSize(font);
//}

/* New Tectonic APIs for crate encapsulation */

//const char *
//ttxl_platfont_get_desc(PlatformFontRef fontRef)
//{
//    return XeTeXFontMgr::GetFontManager()->getPlatformFontDesc(fontRef).c_str();
//}
