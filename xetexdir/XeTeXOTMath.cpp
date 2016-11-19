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

#include <w2c/config.h>

#include <assert.h>
#include <algorithm>

#include "XeTeXOTMath.h"

#include "XeTeX_web.h"
#include "XeTeXLayoutInterface.h"
#include "XeTeXFontInst.h"
#include "XeTeXswap.h"

static int32_t getCoverage(const Coverage* coverage, GlyphID g)
{
    if (SWAP(coverage->format) == 1) {
        const CoverageFormat1 *table = (const CoverageFormat1 *) coverage;
        for (int i = 0; i < SWAP(table->glyphCount); i++) {
            if (SWAP(table->glyphArray[i]) == g)
                return i;
        }
    } else if (SWAP(coverage->format) == 2) {
        const CoverageFormat2 *table = (const CoverageFormat2 *) coverage;
        for (int i = 0; i < SWAP(table->rangeCount); i++) {
            if (SWAP(table->rangeArray[i].start) <= g && SWAP(table->rangeArray[i].end) >= g)
                return SWAP(table->rangeArray[i].startCoverageIndex) + (g - SWAP(table->rangeArray[i].start));
        }
    }

    return -1;
}

static int16_t getMathConstant(XeTeXFontInst* fontInst, mathConstantIndex whichConstant)
{
    const char* table = fontInst->getMathTable();
    if (table == NULL)
        return 0;

    const uint16_t* constants = (const uint16_t*)(table + SWAP(((const MathTableHeader*)table)->mathConstants));

    if (whichConstant < firstMathValueRecord) {
        /* it's a simple 16-bit value */
        return SWAP(constants[whichConstant]);
    }
    else if (whichConstant <= lastMathValueRecord) {
        const MathValueRecord* valueRecords = (const MathValueRecord*)
            ((char*)constants + firstMathValueRecord * sizeof(uint16_t) - firstMathValueRecord * sizeof(MathValueRecord));
        return SWAP(valueRecords[whichConstant].value);
    }
    else if (whichConstant <= lastMathConstant) {
        return SWAP(constants[whichConstant + (lastMathValueRecord - firstMathValueRecord + 1)]);
    }
    else
        return 0; /* or abort, with "internal error" or something */
}

int
get_ot_math_constant(int f, int n)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);
        rval = getMathConstant(font, (mathConstantIndex)n);
        /* scale according to font size, except the ones that are percentages */
        if (n > scriptScriptPercentScaleDown && n < radicalDegreeBottomRaisePercent)
            rval = D2Fix(font->unitsToPoints(rval));
    }
    return rval;
}

/* fontdimen IDs for math symbols font (family 2) */
#define math_x_height   5
#define math_quad       6
#define num1            8   /* numerator shift-up in display styles */
#define num2            9   /* numerator shift-up in non-display, non-\.{\\atop} */
#define num3            10  /* numerator shift-up in non-display \.{\\atop} */
#define denom1          11  /* denominator shift-down in display styles */
#define denom2          12  /* denominator shift-down in non-display styles */
#define sup1            13  /* superscript shift-up in uncramped display style */
#define sup2            14  /* superscript shift-up in uncramped non-display */
#define sup3            15  /* superscript shift-up in cramped styles */
#define sub1            16  /* subscript shift-down if superscript is absent */
#define sub2            17  /* subscript shift-down if superscript is present */
#define sup_drop        18  /* superscript baseline below top of large box */
#define sub_drop        19  /* subscript baseline below bottom of large box */
#define delim1          20  /* size of \.{\\atopwithdelims} delimiters */
#define delim2          21  /* size of \.{\\atopwithdelims} delimiters in non-displays */
#define axis_height     22  /* height of fraction lines above the baseline */

const mathConstantIndex TeX_sym_to_OT_map[] = {
    unknown,
    unknown,
    unknown,
    unknown,
    unknown,
    accentBaseHeight, // x-height
    unknown, // quad
    unknown,
    fractionNumeratorDisplayStyleShiftUp,
    fractionNumeratorShiftUp,
    stackTopShiftUp,
    fractionDenominatorDisplayStyleShiftDown,
    fractionDenominatorShiftDown,
    superscriptShiftUp, // ??
    superscriptShiftUp, // ??
    superscriptShiftUpCramped,
    subscriptShiftDown, // ??
    subscriptShiftDown, // ??
    superscriptBaselineDropMax, // ??
    subscriptBaselineDropMin, // ??
    delimitedSubFormulaMinHeight,
    unknown, // using quad instead for now
    axisHeight
};

int
get_native_mathsy_param(int f, int n)
{
    int rval = 0;

    if (n == math_quad) {
        rval = font_size[f];
    }
    else if (n == delim2) { // XXX not sure what OT parameter we should use here;
                            // for now we use 1.5em, clamped to delim1 height
        rval = std::min<int>(1.5 * font_size[f], get_native_mathsy_param(f, delim1));
    }
    else {
        if (n < sizeof(TeX_sym_to_OT_map) / sizeof(mathConstantIndex)) {
            mathConstantIndex ot_index = TeX_sym_to_OT_map[n];
            if (ot_index != unknown)
                rval = get_ot_math_constant(f, (int)ot_index);
        }
    }
//  fprintf(stderr, " math_sy(%d, %d) returns %.3f\n", f, n, Fix2D(rval));

    return rval;
}

/* fontdimen IDs for math extension font (family 3) */
#define default_rule_thickness  8   /* thickness of \.{\\over} bars */
#define big_op_spacing1         9   /* minimum clearance above a displayed op */
#define big_op_spacing2         10  /* minimum clearance below a displayed op */
#define big_op_spacing3         11  /* minimum baselineskip above displayed op */
#define big_op_spacing4         12  /* minimum baselineskip below displayed op */
#define big_op_spacing5         13  /* padding above and below displayed limits */

const mathConstantIndex TeX_ext_to_OT_map[] = {
    unknown,
    unknown,
    unknown,
    unknown,
    unknown,
    accentBaseHeight, // x-height
    unknown, // quad
    unknown,
    fractionRuleThickness, // default_rule_thickness
    upperLimitGapMin, // big_op_spacing1
    lowerLimitGapMin, // big_op_spacing2
    upperLimitBaselineRiseMin, // big_op_spacing3
    lowerLimitBaselineDropMin, // big_op_spacing4
    stackGapMin // big_op_spacing5
};

int
get_native_mathex_param(int f, int n)
{
    int rval = 0;

    if (n == math_quad)
        rval = font_size[f];
    else {
        if (n < sizeof(TeX_ext_to_OT_map) / sizeof(mathConstantIndex)) {
            mathConstantIndex ot_index = TeX_ext_to_OT_map[n];
            if (ot_index != unknown)
                rval = get_ot_math_constant(f, (int)ot_index);
        }
    }
//  fprintf(stderr, " math_ex(%d, %d) returns %.3f\n", f, n, Fix2D(rval));

    return rval;
}

int
get_ot_math_variant(int f, int g, int v, integer* adv, int horiz)
{
    int rval = g;
    *adv = -1;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathVariants);
        if (offset == 0)
            return rval;
        const MathVariants* variants = (const MathVariants*)(table + offset);

        offset = horiz ? SWAP(variants->horizGlyphCoverage) : SWAP(variants->vertGlyphCoverage);
        if (offset == 0)
            return rval;
        const Coverage* coverage = (const Coverage*)(((const char*)variants) + offset);

        int32_t index = getCoverage(coverage, g);
        if (index >= 0) {
            if (horiz)
                index += SWAP(variants->vertGlyphCount);
            const MathGlyphConstruction*    construction = (const MathGlyphConstruction*)(((const char*)variants)
                                                            + SWAP(variants->vertGlyphConstruction[index]));
            if (v < SWAP(construction->variantCount)) {
                rval = SWAP(construction->mathGlyphVariantRecord[v].variantGlyph);
                *adv = D2Fix(font->unitsToPoints(SWAP(construction->mathGlyphVariantRecord[v].advanceMeasurement)));
            }
        }
    }

    return rval;
}

void*
get_ot_assembly_ptr(int f, int g, int horiz)
{
    void*   rval = NULL;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathVariants);
        if (offset == 0)
            return rval;
        const MathVariants* variants = (const MathVariants*)(table + offset);

        offset = horiz ? SWAP(variants->horizGlyphCoverage) : SWAP(variants->vertGlyphCoverage);
        if (offset == 0)
            return rval;
        const Coverage* coverage = (const Coverage*)(((const char*)variants) + offset);

        int32_t index = getCoverage(coverage, g);
        if (index >= 0) {
            if (horiz)
                index += SWAP(variants->vertGlyphCount);
            const MathGlyphConstruction*    construction = (const MathGlyphConstruction*)(((const char*)variants)
                                                            + SWAP(variants->vertGlyphConstruction[index]));
            offset = SWAP(construction->glyphAssembly);
            if (offset != 0)
                rval = (void*)(((const char*)construction) + offset);
        }
    }

    return rval;
}

int
get_ot_math_ital_corr(int f, int g)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathGlyphInfo);
        if (offset == 0)
            return rval;
        const MathGlyphInfo* glyphInfo = (const MathGlyphInfo*)(table + offset);

        offset = SWAP(glyphInfo->mathItalicsCorrectionInfo);
        if (offset == 0)
            return rval;
        const MathItalicsCorrectionInfo* italCorrInfo = (const MathItalicsCorrectionInfo*)(((const char*)glyphInfo) + offset);

        offset = SWAP(italCorrInfo->coverage);
        if (offset == 0)
            return rval;
        const Coverage* coverage = (const Coverage*)(((const char*)italCorrInfo) + offset);

        int32_t index = getCoverage(coverage, g);
        if (index >= 0 && index < SWAP(italCorrInfo->italicsCorrectionCount))
            rval = D2Fix(font->unitsToPoints(SWAP(italCorrInfo->italicsCorrection[index].value)));
    }

    return rval;
}

int
get_ot_math_accent_pos(int f, int g)
{
    int rval = 0x7fffffffUL;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathGlyphInfo);
        if (offset == 0)
            return rval;
        const MathGlyphInfo* glyphInfo = (const MathGlyphInfo*)(table + offset);

        offset = SWAP(glyphInfo->mathTopAccentAttachment);
        if (offset == 0)
            return rval;
        const MathTopAccentAttachment* accentAttachment = (const MathTopAccentAttachment*)(((const char*)glyphInfo) + offset);

        offset = SWAP(accentAttachment->coverage);
        if (offset == 0)
            return rval;
        const Coverage* coverage = (const Coverage*)(((const char*)accentAttachment) + offset);

        int32_t index = getCoverage(coverage, g);
        if (index >= 0 && index < SWAP(accentAttachment->topAccentAttachmentCount)) {
            rval = (int16_t)SWAP(accentAttachment->topAccentAttachment[index].value);
            rval = D2Fix(font->unitsToPoints(rval));
        }
    }

    return rval;
}

int
ot_min_connector_overlap(int f)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathVariants);
        if (offset == 0)
            return rval;
        const MathVariants* variants = (const MathVariants*)(table + offset);

        rval = D2Fix(font->unitsToPoints(SWAP(variants->minConnectorOverlap)));
    }

    return rval;
}

typedef enum {
    topRight,
    topLeft,
    bottomRight,
    bottomLeft,
} MathKernSide;

static int
getMathKernAt(int f, int g, MathKernSide side, int height)
{
    int rval = 0;
    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst* font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);

        const char* table = font->getMathTable();
        if (table == NULL)
            return rval;

        uint16_t    offset = SWAP(((const MathTableHeader*)table)->mathGlyphInfo);
        if (offset == 0)
            return rval;

        const MathGlyphInfo* glyphInfo = (const MathGlyphInfo*)(table + offset);

        offset = SWAP(glyphInfo->mathKernInfo);
        if (offset == 0)
            return rval;

        const MathKernInfo* mathKernInfo = (const MathKernInfo*)(((const char*)glyphInfo) + offset);

        offset = SWAP(mathKernInfo->coverage);
        if (offset == 0)
            return rval;

        const Coverage* coverage = (const Coverage*)(((const char*)mathKernInfo) + offset);

        int32_t index = getCoverage(coverage, g);
        if (index >= 0 && index < SWAP(mathKernInfo->kernInfoCount)) {
            if (side == topRight)
                offset = SWAP(mathKernInfo->kernInfo[index].topRight);
            else if (side == bottomRight)
                offset = SWAP(mathKernInfo->kernInfo[index].bottomRight);
            else if (side == topLeft)
                offset = SWAP(mathKernInfo->kernInfo[index].topLeft);
            else if (side == bottomLeft)
                offset = SWAP(mathKernInfo->kernInfo[index].bottomLeft);
            else
                assert(0); // we should not reach here

            if (offset == 0)
                return rval;

            const MathKernTable* kernTable = (const MathKernTable*)(((const char*)mathKernInfo) + offset);

            uint16_t count = SWAP(kernTable->heightCount);

            // kern[] array immediately follows the height[] array with |count| elements
            const MathValueRecord* kern = &kernTable->height[0] + count;

            if (count == 0 || height < SWAP(kernTable->height[0].value))
                rval = SWAP(kern[0].value);
            else {
                rval = SWAP(kern[count].value);
                for (int i = 0; i < count; i++) {
                    if (height <= SWAP(kernTable->height[i].value)) {
                        rval = SWAP(kern[i].value);
                        break;
                    }
                }
            }
        }
    }

    return rval;
}

static float
glyph_height(int f, int g)
{
    float rval = 0.0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[f];
        getGlyphHeightDepth(engine, g, &rval, NULL);
    }

    return rval;
}

static float
glyph_depth(int f, int g)
{
    float rval = 0.0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXLayoutEngine engine = (XeTeXLayoutEngine)font_layout_engine[f];
        getGlyphHeightDepth(engine, g, NULL, &rval);
    }

    return rval;
}

// keep in sync with xetex.web
#define sup_cmd 0
#define sub_cmd 1

int
get_ot_math_kern(int f, int g, int sf, int sg, int cmd, int shift)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst* font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);
        int kern = 0, skern = 0;
        float corr_height_top = 0.0, corr_height_bot = 0.0;

        if (cmd == sup_cmd) { // superscript
            corr_height_top =  font->pointsToUnits(glyph_height(f, g));
            corr_height_bot = -font->pointsToUnits(glyph_depth(sf, sg) + Fix2D(shift));

            kern = getMathKernAt(f, g, topRight, corr_height_top);
            skern = getMathKernAt(sf, sg, bottomLeft, corr_height_top);
            rval = kern + skern;

            kern = getMathKernAt(f, g, topRight, corr_height_bot);
            skern = getMathKernAt(sf, sg, bottomLeft, corr_height_bot);
            if ((kern + skern) < rval)
                rval = kern + skern;

        } else if (cmd == sub_cmd) { // subscript
            corr_height_top =  font->pointsToUnits(glyph_height(sf, sg) - Fix2D(shift));
            corr_height_bot = -font->pointsToUnits(glyph_depth(f, g));

            kern = getMathKernAt(f, g, bottomRight, corr_height_top);
            skern = getMathKernAt(sf, sg, topLeft, corr_height_top);
            rval = kern + skern;

            kern = getMathKernAt(f, g, bottomRight, corr_height_bot);
            skern = getMathKernAt(sf, sg, topLeft, corr_height_bot);
            if ((kern + skern) < rval)
                rval = kern + skern;

        } else {
            assert(0); // we should not reach here
        }

        return D2Fix(font->unitsToPoints(rval));
    }

    return 0;
}

int
ot_part_count(const GlyphAssembly* a)
{
    return SWAP(a->partCount);
}

int
ot_part_glyph(const GlyphAssembly* a, int i)
{
    return SWAP(a->partRecords[i].glyph);
}

int
ot_part_is_extender(const GlyphAssembly* a, int i)
{
    return (SWAP(a->partRecords[i].partFlags) & fExtender) != 0;
}

int
ot_part_start_connector(int f, const GlyphAssembly* a, int i)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);
        rval = D2Fix(font->unitsToPoints(SWAP(a->partRecords[i].startConnectorLength)));
    }

    return rval;
}

int
ot_part_end_connector(int f, const GlyphAssembly* a, int i)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);
        rval = D2Fix(font->unitsToPoints(SWAP(a->partRecords[i].endConnectorLength)));
    }

    return rval;
}

int
ot_part_full_advance(int f, const GlyphAssembly* a, int i)
{
    int rval = 0;

    if (font_area[f] == OTGR_FONT_FLAG) {
        XeTeXFontInst*  font = (XeTeXFontInst*)getFont((XeTeXLayoutEngine)font_layout_engine[f]);
        rval = D2Fix(font->unitsToPoints(SWAP(a->partRecords[i].fullAdvance)));
    }

    return rval;
}
