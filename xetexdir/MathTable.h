/****************************************************************************\
 Part of the XeTeX typesetting system
 Copyright (c) 1994-2008 by SIL International
 Copyright (c) 2009 by Jonathan Kew
 Copyright (c) 2012 by Khaled Hosny

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

#ifndef __MATHTABLE_H__
#define __MATHTABLE_H__

#ifndef ANY_NUMBER
#define ANY_NUMBER 1
#endif

typedef uint16_t Offset;

typedef struct {
	int16_t		value;
	Offset		deviceTable;
} MathValueRecord;

typedef struct {
	GlyphID		start;
	GlyphID		end;
	int16_t		startCoverageIndex;
} RangeRecord;

typedef struct {
	uint32_t	version;
	Offset		mathConstants;
	Offset		mathGlyphInfo;
	Offset		mathVariants;
} MathTableHeader;

typedef struct {
	uint16_t		scriptPercentScaleDown;
	uint16_t		scriptScriptPercentScaleDown;
	uint16_t		delimitedSubFormulaMinHeight;
	uint16_t		displayOperatorMinHeight;
	MathValueRecord mathLeading;
	MathValueRecord axisHeight;
	MathValueRecord accentBaseHeight;
	MathValueRecord flattenedAccentBaseHeight;
	MathValueRecord subscriptShiftDown;
	MathValueRecord subscriptTopMax;
	MathValueRecord subscriptBaselineDropMin;
	MathValueRecord superscriptShiftUp;
	MathValueRecord superscriptShiftUpCramped;
	MathValueRecord superscriptBottomMin;
	MathValueRecord superscriptBaselineDropMax;
	MathValueRecord subSuperscriptGapMin;
	MathValueRecord superscriptBottomMaxWithSubscript;
	MathValueRecord spaceAfterScript;
	MathValueRecord upperLimitGapMin;
	MathValueRecord upperLimitBaselineRiseMin;
	MathValueRecord lowerLimitGapMin;
	MathValueRecord lowerLimitBaselineDropMin;
	MathValueRecord stackTopShiftUp;
	MathValueRecord stackTopDisplayStyleShiftUp;
	MathValueRecord stackBottomShiftDown;
	MathValueRecord stackBottomDisplayStyleShiftDown;
	MathValueRecord stackGapMin;
	MathValueRecord stackDisplayStyleGapMin;
	MathValueRecord stretchStackTopShiftUp;
	MathValueRecord stretchStackBottomShiftDown;
	MathValueRecord stretchStackGapAboveMin;
	MathValueRecord stretchStackGapBelowMin;
	MathValueRecord fractionNumeratorShiftUp;
	MathValueRecord fractionNumeratorDisplayStyleShiftUp;
	MathValueRecord fractionDenominatorShiftDown;
	MathValueRecord fractionDenominatorDisplayStyleShiftDown;
	MathValueRecord fractionNumeratorGapMin;
	MathValueRecord fractionNumDiisplayStyleGapMin;
	MathValueRecord fractionRuleThickness;
	MathValueRecord fractionDenominatorGapMin;
	MathValueRecord fractionDenomDisplayStyleGapMin;
	MathValueRecord skewedFractionHorizontalGap;
	MathValueRecord skewedFractionVerticalGap;
	MathValueRecord overbarVerticalGap;
	MathValueRecord overbarRuleThickness;
	MathValueRecord overbarExtraAscender;
	MathValueRecord underbarVerticalGap;
	MathValueRecord underbarRuleThickness;
	MathValueRecord underbarExtraDescender;
	MathValueRecord radicalVerticalGap;
	MathValueRecord radicalDisplayStyleVerticalGap;
	MathValueRecord radicalRuleThickness;
	MathValueRecord radicalExtraAscender;
	MathValueRecord radicalKernBeforeDegree;
	MathValueRecord radicalKernAfterDegree;
	uint16_t 		radicalDegreeBottomRaisePercent;
} MathConstants;

typedef enum {
	unknown = -1,
	scriptPercentScaleDown = 0,
	scriptScriptPercentScaleDown,
	delimitedSubFormulaMinHeight,
	displayOperatorMinHeight,
	mathLeading,
	firstMathValueRecord = mathLeading,
	axisHeight,
	accentBaseHeight,
	flattenedAccentBaseHeight,
	subscriptShiftDown,
	subscriptTopMax,
	subscriptBaselineDropMin,
	superscriptShiftUp,
	superscriptShiftUpCramped,
	superscriptBottomMin,
	superscriptBaselineDropMax,
	subSuperscriptGapMin,
	superscriptBottomMaxWithSubscript,
	spaceAfterScript,
	upperLimitGapMin,
	upperLimitBaselineRiseMin,
	lowerLimitGapMin,
	lowerLimitBaselineDropMin,
	stackTopShiftUp,
	stackTopDisplayStyleShiftUp,
	stackBottomShiftDown,
	stackBottomDisplayStyleShiftDown,
	stackGapMin,
	stackDisplayStyleGapMin,
	stretchStackTopShiftUp,
	stretchStackBottomShiftDown,
	stretchStackGapAboveMin,
	stretchStackGapBelowMin,
	fractionNumeratorShiftUp,
	fractionNumeratorDisplayStyleShiftUp,
	fractionDenominatorShiftDown,
	fractionDenominatorDisplayStyleShiftDown,
	fractionNumeratorGapMin,
	fractionNumDisplayStyleGapMin,
	fractionRuleThickness,
	fractionDenominatorGapMin,
	fractionDenomDisplayStyleGapMin,
	skewedFractionHorizontalGap,
	skewedFractionVerticalGap,
	overbarVerticalGap,
	overbarRuleThickness,
	overbarExtraAscender,
	underbarVerticalGap,
	underbarRuleThickness,
	underbarExtraDescender,
	radicalVerticalGap,
	radicalDisplayStyleVerticalGap,
	radicalRuleThickness,
	radicalExtraAscender,
	radicalKernBeforeDegree,
	radicalKernAfterDegree,
	lastMathValueRecord = radicalKernAfterDegree,
	radicalDegreeBottomRaisePercent,
	lastMathConstant = radicalDegreeBottomRaisePercent
} mathConstantIndex;

typedef struct {
	uint16_t	minConnectorOverlap;
	Offset		vertGlyphCoverage;
	Offset		horizGlyphCoverage;
	uint16_t	vertGlyphCount;
	uint16_t	horizGlyphCount;
	Offset		vertGlyphConstruction[ANY_NUMBER];
	Offset		horizGlyphConstruction[ANY_NUMBER];
} MathVariants;

typedef struct {
	GlyphID		variantGlyph;
	uint16_t	advanceMeasurement;
} MathGlyphVariantRecord;

typedef struct {
	Offset		glyphAssembly;
	uint16_t	variantCount;
	MathGlyphVariantRecord mathGlyphVariantRecord[ANY_NUMBER];
} MathGlyphConstruction;

typedef struct {
	GlyphID		glyph;
	uint16_t	startConnectorLength;
	uint16_t	endConnectorLength;
	uint16_t	fullAdvance;
	uint16_t	partFlags;
} GlyphPartRecord;
#define fExtender	0x0001

typedef struct {
	MathValueRecord italicsCorrection;
	uint16_t		partCount;
	GlyphPartRecord partRecords[ANY_NUMBER];
} GlyphAssembly;

typedef struct {
	Offset		mathItalicsCorrectionInfo;
	Offset		mathTopAccentAttachment;
	Offset		extendedShapeCoverage;
	Offset		mathKernInfo;
} MathGlyphInfo;

typedef struct {
	Offset		coverage;
	uint16_t	italicsCorrectionCount;
	MathValueRecord	italicsCorrection[ANY_NUMBER];
} MathItalicsCorrectionInfo;

typedef struct {
	Offset		coverage;
	uint16_t	topAccentAttachmentCount;
	MathValueRecord	topAccentAttachment[ANY_NUMBER];
} MathTopAccentAttachment;

typedef struct {
	Offset	topRight;
	Offset	topLeft;
	Offset	bottomRight;
	Offset	bottomLeft;
} MathKernInfoRecord;

typedef struct {
	Offset		coverage;
	uint16_t	kernInfoCount;
	MathKernInfoRecord kernInfo[ANY_NUMBER];
} MathKernInfo;

typedef struct {
	uint16_t		heightCount;
	MathValueRecord	height[ANY_NUMBER];
/*
 * The offset of this will vary depending on the size of the height[] array,
 * so we have to compute it at runtime, not refer to a fixed field offset.
 *
	MathValueRecord	kern[ANY_NUMBER];
 */
} MathKernTable;

typedef struct {
    uint16_t	format;
} Coverage;

typedef struct {
    uint16_t	format;
    uint16_t	glyphCount;
    GlyphID		glyphArray[ANY_NUMBER];
} CoverageFormat1;

typedef struct {
    uint16_t	format;
    uint16_t	rangeCount;
    RangeRecord	rangeArray[ANY_NUMBER];
} CoverageFormat2;

#endif
