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

#ifndef __XETEX_OT_MATH__
#define __XETEX_OT_MATH__

#include "XeTeX_ext.h"
#include "MathTable.h"

/* public "C" APIs for calling from Web(-to-C) code */
#ifdef __cplusplus
extern "C" {
#endif
    int get_native_mathsy_param(int f, int n);
    int get_native_mathex_param(int f, int n);
    int get_ot_math_constant(int f, int n);
    int get_ot_math_variant(int f, int g, int v, integer* adv, int horiz);
    void* get_ot_assembly_ptr(int f, int g, int horiz);
    int get_ot_math_ital_corr(int f, int g);
    int get_ot_math_accent_pos(int f, int g);
    int get_ot_math_kern(int f, int g, int sf, int sg, int cmd, int shift);
    int ot_part_count(const GlyphAssembly* a);
    int ot_part_glyph(const GlyphAssembly* a, int i);
    int ot_part_is_extender(const GlyphAssembly* a, int i);
    int ot_part_start_connector(int f, const GlyphAssembly* a, int i);
    int ot_part_end_connector(int f, const GlyphAssembly* a, int i);
    int ot_part_full_advance(int f, const GlyphAssembly* a, int i);
    int ot_min_connector_overlap(int f);
#ifdef __cplusplus
};
#endif

#endif
