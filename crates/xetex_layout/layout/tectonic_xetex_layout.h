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

/* Set up our types */

//#ifdef XETEX_MAC
//
//#include <ApplicationServices/ApplicationServices.h>
//typedef CTFontDescriptorRef PlatformFontRef;
//
//#else /* XETEX_MAC */
//
//#include <fontconfig/fontconfig.h>
//typedef FcPattern* PlatformFontRef;
//typedef int32_t Fixed; /* macOS defines Fixed in system headers */
//
//#endif /* XETEX_MAC */

typedef uint16_t GlyphID;

#endif /* XETEX_LAYOUT_INTERFACE_H */
