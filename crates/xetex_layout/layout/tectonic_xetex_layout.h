/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
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

/* Formerly known as [xetex-]XeTeXLayoutInterface.h */

#ifndef XETEX_LAYOUT_INTERFACE_H
#define XETEX_LAYOUT_INTERFACE_H 1

#include "tectonic_bridge_core.h"
#include "layout_bindings.h"

/* harfbuzz: hb_tag_t and hb_font_t used below */
#include <harfbuzz/hb.h>


/* Set up our types */

#ifdef XETEX_MAC

#include <ApplicationServices/ApplicationServices.h>
typedef CTFontDescriptorRef PlatformFontRef;

#else /* XETEX_MAC */

#include <fontconfig/fontconfig.h>
typedef FcPattern* PlatformFontRef;
typedef int32_t Fixed; /* macOS defines Fixed in system headers */

#endif /* XETEX_MAC */

typedef uint16_t GlyphID;

typedef struct XeTeXFont_rec* XeTeXFont;
typedef struct XeTeXLayoutEngine_rec* XeTeXLayoutEngine;

/* Now we can defined our C APIs */

BEGIN_EXTERN_C

extern Fixed loaded_font_design_size;

void terminate_font_manager(void);
void destroy_font_manager(void);

PlatformFontRef findFontByName(const char* name, char* var, double size);

char getReqEngine(void);
void setReqEngine(char reqEngine);
const char* getFullName(PlatformFontRef fontRef);

double getDesignSize(XeTeXFont font);

/* Extra APIs needed to encapsulate across the crate boundaries */
const char *ttxl_platfont_get_desc(PlatformFontRef fontRef);

END_EXTERN_C

#endif /* XETEX_LAYOUT_INTERFACE_H */
