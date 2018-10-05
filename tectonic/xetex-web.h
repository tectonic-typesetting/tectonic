/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009-2012 by Jonathan Kew
 Copyright (c) 2010-2014 by Han The Thanh
 Copyright (c) 2012-2015 by Khaled Hosny
 Copyright (c) 2014 by Peter Breitenlohner

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

#ifndef __XETEX_WEB_H
#define __XETEX_WEB_H

#include "xetex-core.h"
#include "xetex-ext.h"

#ifndef M_PI
#define M_PI 3.14159265358979323846264338327950288
#endif

BEGIN_EXTERN_C

void print_nl(int s);
void print_char(int c);
void begin_diagnostic(void);
void end_diagnostic(int nl);
int get_tracing_fonts_state(void);

extern Fixed loaded_font_design_size;
extern void **font_layout_engine;
extern int32_t *font_area;
extern int32_t *font_size;

END_EXTERN_C

#endif /* __XETEX_WEB_H */
