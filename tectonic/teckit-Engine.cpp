/*------------------------------------------------------------------------
Copyright (C) 2002-2016 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: Engine.cp
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Implements the TECkit conversion engine.
-------------------------------------------------------------------------*/

/*
	2008-01-23  jk  revised endian-ness stuff to allow Universal build
	2006-06-02	jk	added support for extended string rules (>255 per initial char)
	2006-06-02	jk	fixed bug handling passes with no mapping rules
	2006-01-12	jk	remove multi-char constants, use kTableType_XXX from TECkit_Format.h
	2005-07-19	jk	revised to use WORDS_BIGENDIAN conditional, config.h
	2005-05-06	jk	patched match() to forget matches within groups if we backtrack out
	2004-03-19	jk	rewrote match() to fix group/repeat bugs and be more efficient
	2004-03-12	jk	finished updating for version 2.1 with ...Opt APIs
*/

#include "tectonic_bridge_core.h"

//#define TRACING	1
#define VERSION "2.5.4"

#if	(defined(__dest_os) && (__dest_os == __win32_os)) || defined(WIN32)	/* Windows target: little-endian */
#	undef WORDS_BIGENDIAN
#endif

#ifdef __APPLE__
#include <TargetConditionals.h>
#endif

#if defined(TARGET_RT_BIG_ENDIAN)	/* the CodeWarrior prefix files or Apple TargetConditionals.h sets this */
#	if TARGET_RT_BIG_ENDIAN
#		undef WORDS_BIGENDIAN
#		define WORDS_BIGENDIAN 1
#	else
#		undef WORDS_BIGENDIAN
#	endif
#endif

#if	(defined(__dest_os) && (__dest_os == __win32_os)) || defined(WIN32)
#	define WIN32_LEAN_AND_MEAN
#	define NOSERVICE
#	define NOMCX
#	include <windows.h>
#endif

#include "teckit-cxx-Engine.h"

#include "tectonic_bridge_flate.h"

#ifdef TRACING
#include <iostream>

int	traceLevel = 1;
#endif

#include <cstdlib>
#include <cstring>
#include <algorithm>

static const UInt32 kNeedMoreInput = 0xfffffffeUL;
static const UInt32 kInvalidChar   = 0xfffffffdUL;
static const UInt32 kUnmappedChar  = 0xfffffffcUL;

using namespace std;

/* we apply READ to values read from the compiled table, to provide byte-swapping where needed */
inline UInt8
READ(const UInt8 p)
{
	return p;
}

inline UInt16
READ(const UInt16 p)
{
#ifdef WORDS_BIGENDIAN
	return p;
#else
	return (p >> 8) + (p << 8);
#endif
}

inline UInt32
READ(const UInt32 p)
{
#ifdef WORDS_BIGENDIAN
	return p;
#else
	return (p >> 24) + ((p >> 8) & 0x0000ff00) + ((p << 8) & 0x00ff0000) + (p << 24);
#endif
}

Stage::Stage()
	: oBuffer(0)
	, oBufSize(0)
	, oBufEnd(0)
	, oBufPtr(0)
	, prevStage(0)
{
}

Stage::~Stage()
{
	if (prevStage && prevStage->prevStage)
		delete prevStage;
}

UInt32
Stage::lookaheadCount() const
{
	return 0;
}

#include "teckit-NormalizationData.c"

Normalizer::Normalizer(bool compose)
	: prevCombClass(0)
	, oBufSafe(0)
	, bCompose(compose)
{
	oBufSize = 256;
	oBuffer = new UInt32[oBufSize];
}

Normalizer::~Normalizer()
{
	delete[] oBuffer;
}

/* constants for algorithmic Hangul decomposition */
#define	SBase	0xAC00
#define	LBase	0x1100
#define	VBase	0x1161
#define	TBase	0x11A7
#define	LCount	19
#define	VCount	21
#define	TCount	28
#define	NCount	(VCount * TCount)
#define	SCount	(LCount * NCount)

UInt32
Normalizer::process()
{
	UInt32	inChar = prevStage->getChar();
	if (inChar == kNeedMoreInput || inChar == kInvalidChar || inChar == kUnmappedChar)
		return inChar;
	if (inChar == kEndOfText) {
		generateChar(kEndOfText);
		return inChar;
	}

	UInt32	SIndex = inChar - SBase;
	if (SIndex >= SCount)
		decompose(inChar);
	else {
		generateChar(LBase + SIndex / NCount);
		generateChar(VBase + (SIndex % NCount) / TCount);
		UInt32 T = SIndex % TCount;
		if (T != 0)
			generateChar(TBase + T);
	}

	return 0;
}

void
Normalizer::Reset()
{
	oBufPtr = oBufEnd = 0;
	prevCombClass = 0;
	oBufSafe = 0;
}

void
Normalizer::decompose(UInt32 c)
{
	UInt32	prefix = decomposeOne(c);
	if (prefix != 0xffff)
		decompose(prefix);
	if (c != 0xffff)
		generateChar(c);
}

UInt32
Normalizer::decomposeOne(UInt32& c)
{
	UInt32	plane = c >> 16;
	UInt32	page = (c >> 8) & 0xff;
	UInt32	ch = c & 0xff;

	UInt16	charIndex = dcCharIndex[dcPageMaps[dcPlaneMap[plane]][page]][ch];
	if (charIndex == 0)
		return 0xffff;
	c = dcDecomposition[charIndex][1];
	return dcDecomposition[charIndex][0];
}

void
Normalizer::generateChar(UInt32 c)
{
	int	combClass = 0;
	if (c != kEndOfText) {
		UInt32	plane = c >> 16;
		UInt32	page = (c >> 8) & 0xff;
		UInt32	ch = c & 0xff;
		combClass = ccCharClass[ccPageMaps[ccPlaneMap[plane]][page]][ch];
	}

	if (combClass != 0) {
		// combiners are always buffered for sorting and possible composition
		if (prevCombClass <= combClass) {
			appendChar(c);
			prevCombClass = combClass;
		}
		else
			insertChar(c, combClass);
	}
	else {
		if (bCompose) {
			if (oBufEnd > 0) {
				// check whether last buffered char and current char should form Hangul syllable
				UInt32	last = oBuffer[oBufEnd - 1];

				// 1. check to see if two current characters are L and V
				UInt32	LIndex = last - LBase;
				if (LIndex < LCount) {
					UInt32	VIndex = c - VBase;
					if (VIndex < VCount) {
						// make syllable of form LV
						last = SBase + (LIndex * VCount + VIndex) * TCount;
						oBuffer[oBufEnd - 1] = last; // reset last
						return; // don't append c, and don't update oBufSafe as a following V would compose
					}
				}

				// 2. check to see if two current characters are LV and T
				UInt32	SIndex = last - SBase;
				if (SIndex < SCount && (SIndex % TCount) == 0) {
					UInt32	TIndex = c - TBase;
					if (TIndex <= TCount) {
						// make syllable of form LVT
						last += TIndex;
						oBuffer[oBufEnd - 1] = last; // reset last
						oBufSafe = oBufEnd;	// no more composition will be possible now
						return; // don't append c
					}
				}
			}

			// search for canonical compositions in the buffered text, and update oBufSafe if possible
			compose();
		}
		else
			oBufSafe = oBufEnd;
		appendChar(c);
		if (c == kEndOfText)
			oBufSafe = oBufEnd;
		prevCombClass = 0;
	}
}

void
Normalizer::appendChar(UInt32 c)
{
	/* unlikely that we'd ever need to do this--it would take a long string of non-spacing marks! */
	if (oBufEnd == oBufSize)
		growOutBuf();

	oBuffer[oBufEnd++] = c;
}

void
Normalizer::insertChar(UInt32 insCh, int insCombClass)
{
	if (oBufEnd == oBufSize)
		growOutBuf();

	UInt32 i;
	for (i = oBufEnd - 1; i > 0; --i) {
		UInt32	c = oBuffer[i];
		UInt32	plane = c >> 16;
		UInt32	page = (c >> 8) & 0xff;
		UInt32	ch = c & 0xff;
		int	combClass = ccCharClass[ccPageMaps[ccPlaneMap[plane]][page]][ch];
		if (insCombClass >= combClass)
			break;
	}
	++i;

	for (UInt32 j = oBufEnd; j > i; --j)
		oBuffer[j] = oBuffer[j - 1];

	oBuffer[i] = insCh;
	oBufEnd++;
}

void
Normalizer::growOutBuf()
{
	UInt32	newSize = oBufSize + 256;
	UInt32*	newBuf = new UInt32[newSize];
	for (long i = 0; i < oBufSize; ++i)
		newBuf[i] = oBuffer[i];
	delete[] oBuffer;
	oBuffer = newBuf;
	oBufSize = newSize;
}

void
Normalizer::compose()
{
	// search for compositions in oBuffer up to oBufEnd
	UInt32	starterPos = 0;

	UInt32	c = oBuffer[0];
	UInt32	plane = c >> 16;
	UInt32	page = (c >> 8) & 0xff;
	UInt32	ch = c & 0xff;
	int		lastClass = ccCharClass[ccPageMaps[ccPlaneMap[plane]][page]][ch];
	if (lastClass != 0)
		lastClass = 256;

	if (oBufEnd > 1) {
		UInt32	compPos = 1;
		UInt16	li = cLCharIndex[cLPageMaps[cLPlaneMap[plane]][page]][ch];

	    for (long decompPos = 1; decompPos < oBufEnd; ++decompPos) {
			c = oBuffer[decompPos];
			plane = c >> 16;
			page = (c >> 8) & 0xff;
			ch = c & 0xff;
			int		chClass = ccCharClass[ccPageMaps[ccPlaneMap[plane]][page]][ch];
			UInt16	ri = cRCharIndex[cRPageMaps[cRPlaneMap[plane]][page]][ch];
	        UInt32	cmp = cComposites[li][ri];
			if (cmp != 0 && (lastClass < chClass || lastClass == 0)) {
	            oBuffer[starterPos] = cmp;
				plane = cmp >> 16;
				page = (cmp >> 8) & 0xff;
				ch = cmp & 0xff;
				li = cLCharIndex[cLPageMaps[cLPlaneMap[plane]][page]][ch];
	        }
	        else {
	            if (chClass == 0) {
	                starterPos = compPos;
					plane = c >> 16;
					page = (c >> 8) & 0xff;
					ch = c & 0xff;
					li = cLCharIndex[cLPageMaps[cLPlaneMap[plane]][page]][ch];
	            }
	            lastClass = chClass;
	            oBuffer[compPos++] = c;
	        }
	    }
	    oBufEnd = compPos;
	}

    // update oBufSafe to pass any chars that definitely can't compose
	if (lastClass != 0)
		oBufSafe = oBufEnd;
	else
		oBufSafe = starterPos;
}

UInt32
Normalizer::getChar()
{
	UInt32	c;
	while (oBufSafe == 0) {
		c = process();
		if (c == kNeedMoreInput || c == kInvalidChar || c == kUnmappedChar)
			return c;
	}
	c = oBuffer[oBufPtr++];
	if (oBufPtr == oBufSafe) {
		for (long i = oBufPtr; i < oBufEnd; ++i)
			oBuffer[i - oBufPtr] = oBuffer[i];
		oBufEnd -= oBufPtr;
		oBufSafe = oBufPtr = 0;
	}
	return c;
}

Pass::Pass(const TableHeader* inTable, Converter* cnv)
	: converter(cnv)
	, tableHeader(inTable)
	, iBuffer(0)
	, iBufSize(0)
	, iBufStart(0)
	, iBufEnd(0)
	, iBufPtr(0)
{
	bInputIsUnicode 	= ((READ(tableHeader->type) & 0xFF000000) >> 24) == 'U';
	bOutputIsUnicode	= (READ(tableHeader->type) & 0x000000FF) == 'U';
	bSupplementaryChars	= (READ(tableHeader->flags) & kTableFlags_Supplementary) != 0;

	numPageMaps = 1;
	pageBase		= reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->pageBase);
	lookupBase		= reinterpret_cast<const Lookup*>(reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->lookupBase));
	matchClassBase	= reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->matchClassBase);
	repClassBase	= reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->repClassBase);
	stringListBase	= reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->stringListBase);
	stringRuleData	= reinterpret_cast<const Byte*>(tableHeader) + READ(tableHeader->stringRuleData);

	if (bInputIsUnicode && bSupplementaryChars) {
		// support supplementary plane chars
		planeMap = pageBase;
		pageBase += 20;
		numPageMaps = READ(*(planeMap + 17));
	}

	iBufSize = (READ(inTable->maxMatch) + READ(inTable->maxPre) + READ(inTable->maxPost) + 7) & ~0x0003;
	iBuffer = new UInt32[iBufSize];

	oBufSize = (READ(inTable->maxOutput) + 7) & ~0x0003;
	oBuffer = new UInt32[oBufSize];
}

Pass::~Pass()
{
	delete[] oBuffer;
	delete[] iBuffer;
}

void
Pass::Reset()
{
	iBufStart = iBufEnd = iBufPtr = 0;
	oBufPtr = oBufEnd = 0;
}

UInt32
Pass::getChar()
	// called by next Pass when it wants the next character from us
{
	while (oBufPtr == oBufEnd) {
		oBufPtr = oBufEnd = 0;
		UInt32	c = DoMapping();
		if (c == kNeedMoreInput || c == kInvalidChar || c == kUnmappedChar)
			return c;
	}
	return oBuffer[oBufPtr++];
}

void
Pass::outputChar(UInt32 c)
	// Called by DoMapping to generate a character in the output stream
{
	if (oBufEnd < oBufSize)
		oBuffer[oBufEnd++] = c;
			// Cannot overflow provided the table correctly declares maxOutput
			// (so the compiler had better get it right!)
}

UInt32
Pass::lookaheadCount() const
	// return how many characters of lookahead this pass has in its input buffer
{
	return iBufEnd < iBufPtr
		? // iBufEnd has wrapped but iBufPtr hasn't
			iBufEnd + (iBufSize - iBufPtr)
		: // pointers are in the "normal" order
			iBufEnd - iBufPtr;
}

UInt32
Pass::inputChar(long inIndex)
	// Called by DoMapping or match to read the character at a given location
	// relative to the current input stream location
{
	long	target = iBufPtr + inIndex;
	if (inIndex < 0) {
		// look back
		if (target < 0)
			target += iBufSize;
		if (iBufPtr < iBufStart) {
			// iBufPtr has wrapped back to beginning of buffer, leaving iBufStart beyond it
			// so the valid pre-context is from iBufStart to iBufSize-1 and 0 to iBufPtr-1
			if (target >= iBufStart || target < iBufPtr)
				return iBuffer[target];
		}
		else {
			// iBufPtr points beyond iBufStart
			// so the valid pre-context is from iBufStart to iBufPtr-1
			if (target >= iBufStart && target < iBufPtr)
				return iBuffer[target];
		}
		return kEndOfText;
	}
	else {
		// look ahead
		if (target >= iBufSize)
			target -= iBufSize;
		if (iBufPtr == iBufEnd) {
			// ensure that current character is actually available
			UInt32	ch = prevStage->getChar();
			if (ch == kNeedMoreInput || ch == kInvalidChar || ch == kUnmappedChar)
				return ch;	// don't put this into iBuffer!
			iBuffer[iBufEnd++] = ch;
			if (iBufEnd == iBufSize)
				iBufEnd = 0;
			if (iBufEnd == iBufStart) {
				++iBufStart;
				if (iBufStart == iBufSize)
					iBufStart = 0;
			}
		}
		long	index = iBufPtr;
		while (index != target) {
			// scan forward as far as necessary, reading in required chars
			if (index == iBufSize - 1)
				index = 0;
			else
				++index;
			if (index == iBufEnd) {
				UInt32	ch = prevStage->getChar();
				if (ch == kNeedMoreInput || ch == kInvalidChar || ch == kUnmappedChar)
					return ch;
				iBuffer[iBufEnd++] = ch;
				if (iBufEnd == iBufSize)
					iBufEnd = 0;
				if (iBufEnd == iBufStart) {
					++iBufStart;
					if (iBufStart == iBufSize)
						iBufStart = 0;
				}
			}
		}
		return iBuffer[index];
	}
	return kEndOfText;
}

void
Pass::advanceInput(unsigned int numChars)
	// Called by DoMapping to move forward in the input stream
	// Will only move forward over chars already examined by a rule;
	//	therefore, getChar() can't return kEndOfText, kNeedMoreInput, etc.
{
	for (unsigned int i = 0; i < numChars; ++i) {
		if (iBufPtr == iBufEnd) {
			iBuffer[iBufEnd++] = prevStage->getChar();
			if (iBufEnd == iBufStart) {
				++iBufStart;
				if (iBufStart == iBufSize)
					iBufStart = 0;
			}
			if (iBufEnd == iBufSize)
				iBufEnd = 0;
		}
		iBufPtr++;
		if (iBufPtr == iBufSize)
			iBufPtr = 0;
	}
}

template<class T>
static const T*
binary_search(const T* array, UInt32 count, UInt32 value)
{
	while (count > 0) {
		const T*	i = array;
		UInt32	count2 = count / 2;
		i += count2;
		if (READ(*i) < value) {
			array = i + 1;
			count -= count2 + 1;
		}
		else
			count = count2;
	}
	return array;
}

long
Pass::classMatch(UInt32 classNumber, UInt32 inChar) const
{
	const UInt32*	classPtr = reinterpret_cast<const UInt32*>(matchClassBase + READ(*(reinterpret_cast<const UInt32*>(matchClassBase) + classNumber)));
	UInt32			memberCount = READ(*classPtr++);
	if (bInputIsUnicode) {
		if (bSupplementaryChars) {
			// classes are 32-bit
			const UInt32*	p = binary_search(classPtr, memberCount, inChar);
			if (READ(*p) == inChar)
				return p - classPtr;
		}
		else {
			// classes are 16-bit
			const UInt16*	p = binary_search(reinterpret_cast<const UInt16*>(classPtr), memberCount, inChar);
			if (READ(*p) == inChar)
				return p - reinterpret_cast<const UInt16*>(classPtr);
		}
	}
	else {
		// classes are 8-bit
		const UInt8*	p = binary_search(reinterpret_cast<const UInt8*>(classPtr), memberCount, inChar);
		if (READ(*p) == inChar)
			return p - reinterpret_cast<const UInt8*>(classPtr);
	}
	return -1;
}

UInt32
Pass::repClassMember(UInt32 classNumber, UInt32 index) const
{
	const UInt32*	classPtr = reinterpret_cast<const UInt32*>(repClassBase + READ(*(reinterpret_cast<const UInt32*>(repClassBase) + classNumber)));
	UInt32			memberCount = READ(*classPtr++);
	if (index < memberCount)
		if (bOutputIsUnicode)
			if (bSupplementaryChars)
				return READ(classPtr[index]);
			else
				return READ(reinterpret_cast<const UInt16*>(classPtr)[index]);
		else {
			return READ(reinterpret_cast<const UInt8*>(classPtr)[index]);
		}
	else
		return 0;	// this can't happen if the compiler is right!
}

#ifdef TRACING
static int _depth = 0;
#endif

#define RETURN(x)	do { _rval = (x); goto _return_label; } while (0)

#define matchYes	1
#define matchNo		0
UInt32
Pass::match(int index, int repeats, int textLoc)
{
/*
	attempt to match pattern starting at /index/
	initial repeat count is /repeats/
	text offset is /textLoc/

	recurses whenever we might need to backtrack

	returns
		matchYes	- succeeded
		matchNo		- can't match at this position
		other values, eg:
			kNeedMoreInput
			kInvalidChar
			kUnmappedChar
					- aborted without a definite decision
*/

#ifdef TRACING
cerr << "match(" << index << ", " << repeats << ", " << textLoc << ")\n";
#endif

	UInt32	_rval = matchNo;

	// we come back here to loop rather than recurse, with new values for the arguments
RESTART:

	// if this is the first attempt to match at this index, record where we are
	if (repeats == 0) {
		if (index == matchElems)
			matchedLength = textLoc;
		if (index < infoLimit) {
			info[index].matchedSpan.start = textLoc;
#ifdef TRACING
cerr << "info[" << index << "].matchedSpan.start = " << textLoc << "\n";
#endif
		}
	}

	// if we're at the end of the pattern, we have a match
	if (index >= patternLength)
		RETURN(matchYes);

	if (index == 0 && repeats == 0)
		sgrStack = 0;	// ensure this is cleared at start of pattern (shouldn't be necessary?)

	{	// gcc complains about jumping past initializers (from RETURN above) without this
		UInt32				mr;
		const MatchElem&	m = pattern[index];
		int					repeatMin = READ(m.flags.repeat) >> 4;
		int					repeatMax = READ(m.flags.repeat) & 0x0f;
		UInt8				type      = READ(m.flags.type);
		bool				negate    = ((type & kMatchElem_Negate) != 0);

		type = ((type & kMatchElem_NonLit) != 0)
			? type & kMatchElem_TypeMask
			: 0;

		int		classIndex;
		bool	matches;
		UInt32	inChar;

		// start of group: try each alternative in turn
		if (type == kMatchElem_Type_BGroup) {
			// try matching one of the alternatives in the group (again)
			info[index].groupRepeats = repeats;
			if (repeats < repeatMax) {
				int	altIndex = index;
				while (true) {
					mr = match(altIndex + 1, 0, textLoc);
					if (mr != matchNo)
						RETURN(mr);
					// failed, so step ahead to next alternative or end of group
					altIndex += READ(pattern[altIndex].value.bgroup.dNext);
					if ((READ(pattern[altIndex].flags.type) & kMatchElem_TypeMask) != kMatchElem_Type_OR)
						break;
				}
			}
			// if the group has matched enough times...
			if (repeats >= repeatMin) {
				// try to match following stuff
#ifdef TRACING
cerr << "repeats >= repeatMin\n";
#endif
				mr = match(index + READ(m.value.bgroup.dAfter), 0, textLoc);
				if (mr == matchYes) {
					if (index < infoLimit) {
						info[index].matchedSpan.limit = textLoc;
#ifdef TRACING
cerr << "group returning matchYes; info[" << index << "].matchedSpan.limit = " << textLoc << "\n";
#endif
						// don't allow elements within the group to indicate matches beyond the span of the group itself
						for (int i = index + READ(m.value.bgroup.dAfter) - 1; i > index; --i)
							if (i < infoLimit) {
								if (info[i].matchedSpan.start > textLoc)
									info[i].matchedSpan.start = textLoc;
								if (info[i].matchedSpan.limit > textLoc)
									info[i].matchedSpan.limit = textLoc;
							}
					}
				}
				RETURN(mr);
			}
			// otherwise just backtrack
			RETURN(matchNo);
		}

		// reached end of an alternative
		else if (type == kMatchElem_Type_OR || type == kMatchElem_Type_EGroup) {
			int	startIndex = index - READ(m.value.egroup.dStart);
			mr = match(startIndex, info[startIndex].groupRepeats + 1, textLoc);
			RETURN(mr);
		}

		// not a group, so we loop rather than recurse until optionality strikes
		else {
			// ensure that item matches at least repeatMin times
			while (repeats < repeatMin) {
				inChar = inputChar(textLoc);
				if (inChar == kInvalidChar || inChar == kNeedMoreInput || inChar == kUnmappedChar)
					RETURN(inChar);
				matches = false;
				switch (type) {
					case 0:	// literal
						matches = (READ(m.value.usv.data) & kUSVMask) == inChar;
						break;

					case kMatchElem_Type_Class:
						classIndex = classMatch(READ(m.value.cls.index), inChar);
						matches = (classIndex != -1);
						if (matches && repeats == 0 && index < infoLimit)
							info[index].classIndex = classIndex;
						break;

					case kMatchElem_Type_ANY:
						matches = (inChar != kEndOfText);
						break;

					case kMatchElem_Type_EOS:
						matches = (inChar == kEndOfText);
						break;
				}
				matches = (matches != negate);
				if (!matches)
					RETURN(matchNo);
				++repeats;
				textLoc += direction;
			}

			if (index < infoLimit) {
				info[index].matchedSpan.limit = textLoc;
#ifdef TRACING
cerr << "info[" << index << "].matchedSpan.limit = " << textLoc << "\n";
#endif
			}

			if (repeatMin == repeatMax) {
				// no need to recurse, as no optionality
				++index;
				repeats = 0;
				goto RESTART;
			}

			// try for another repeat if allowed
			if (repeats < repeatMax) {
				inChar = inputChar(textLoc);
				if (inChar == kInvalidChar || inChar == kNeedMoreInput || inChar == kUnmappedChar)
					RETURN(inChar);
				matches = false;
				switch (type) {
					case 0:	// literal
						matches = (READ(m.value.usv.data) & kUSVMask) == inChar;
						break;

					case kMatchElem_Type_Class:
						classIndex = classMatch(READ(m.value.cls.index), inChar);
						matches = (classIndex != -1);
						if (matches && repeats == 0 && index < infoLimit)
							info[index].classIndex = classIndex;
						break;

					case kMatchElem_Type_ANY:
						matches = (inChar != kEndOfText);
						break;

					case kMatchElem_Type_EOS:
						matches = (inChar == kEndOfText);
						break;
				}
				matches = (matches != negate);
				if (matches) {
					mr = match(index, repeats + 1, textLoc + direction);
					if (mr != matchNo)
						RETURN(mr);
				}
			}

			// otherwise try to match the remainder of the pattern
			mr = match(index + 1, 0, textLoc);
			RETURN(mr);
		}
	}

_return_label:

	if (_rval == matchNo)
		if (index < infoLimit) {
			info[index].matchedSpan.limit = textLoc;
#ifdef TRACING
cerr << "rval == matchNo; setting info[" << index << "].matchedSpan.limit = " << textLoc << "\n";
#endif
		}

#ifdef TRACING
cerr << "RETURN(" << (_rval == matchYes ? "matchYes" : "matchNo") << ")\n";
#endif
    return _rval;
}

#undef RETURN

#ifdef TRACING
static void
printMatchElem(const MatchElem& m)
{
	string	rval;
	char	buf[20];
	if (m.flags.type & kMatchElem_Negate)
		rval += "!";
	if (m.flags.type & kMatchElem_NonLit) {
		switch (m.flags.type & kMatchElem_TypeMask) {
			case kMatchElem_Type_Class:
				sprintf(buf, "[%d]", m.value.cls.index);
				rval += buf;
				break;
			case kMatchElem_Type_BGroup:
				rval += "(";
				break;
			case kMatchElem_Type_EGroup:
				rval += ")";
				break;
			case kMatchElem_Type_OR:
				rval += "|";
				break;
			case kMatchElem_Type_ANY:
				rval += ".";
				break;
			case kMatchElem_Type_EOS:
				rval += "#";
				break;
			case kMatchElem_Type_Copy:
				rval += "@";
				break;
		}
	}
	else {
		UInt32	v = m.value.usv.data & kUSVMask;
		if (v >= ' ' && v < 0x7e) {
			sprintf(buf, "'%c'", (char)v);
			rval += buf;
		}
		else {
			sprintf(buf, "0x%04X", (UInt32)v);
			rval += buf;
		}
	}
	if (!(m.flags.type & kMatchElem_NonLit) || (m.flags.type & kMatchElem_TypeMask) != kMatchElem_Type_BGroup)
		switch (m.flags.repeat) {
			case 0x01:
				rval += "?";
				break;
			case 0x11:
				break;
			case 0x0F:
				rval += "*";
				break;
			case 0x1F:
				rval += "+";
				break;
			default:
				sprintf(buf, "{%d,%d}", m.flags.repeat >> 4, m.flags.repeat & 0x0F);
				rval += buf;
				break;
		}
	cerr << rval;
}

static void
printMatch(const StringRule* rule)
{
	for (int i = 0; i < READ(rule->matchLength); ++i) {
		cerr << " ";
		printMatchElem(((MatchElem*)(rule + 1))[i]);
//		cerr << "<" << i << ">";
	}
	if (READ(rule->preLength) > 0 || READ(rule->postLength) > 0) {
		cerr << " /";
		for (int i = READ(rule->preLength) - 1; i >= 0; --i) {
			cerr << " ";
			printMatchElem(((MatchElem*)(rule + 1))[READ(rule->matchLength) + READ(rule->postLength) + i]);
		}
		cerr << " _";
		for (int i = 0; i < READ(rule->postLength); ++i) {
			cerr << " ";
			printMatchElem(((MatchElem*)(rule + 1))[READ(rule->matchLength) + i]);
		}
	}
}

static void
printRep(const StringRule* rule)
{
	const RepElem*	r = (const RepElem*)((const MatchElem*)(rule + 1) + rule->matchLength + rule->preLength + rule->postLength);
	for (int i = 0; i < READ(rule->repLength); ++i, ++r) {
		cerr << " ";
		switch (READ(r->flags.type)) {
			case kRepElem_Literal:
				{
					UInt32	v;
					char	buf[20];
					v = READ(r->value);
					if (v >= ' ' && v <= 0x7e) {
						sprintf(buf, "'%c'", v);
						cerr << buf;
					}
					else {
						sprintf(buf, "0x%04X", v);
						cerr << buf;
					}
				}
				break;

			case kRepElem_Class:
				cerr << "[" << (int)READ(r->flags.repClass) << "," << (int)READ(r->flags.matchIndex) << "]";
				break;

			case kRepElem_Copy:
				cerr << "@" << (int)READ(r->flags.matchIndex);
				break;

			case kRepElem_Unmapped:
				cerr << "?";
				break;
		}
	}
}
#endif

UInt32
Pass::DoMapping()
{
	UInt32	inChar = inputChar(0);
	if (inChar == kNeedMoreInput || inChar == kInvalidChar || inChar == kUnmappedChar)
		return inChar;
	if (inChar == kEndOfText) {
		outputChar(kEndOfText);
		return inChar;
	}
	matchedLength = 1;

	const Lookup*	lookup;
	if (bInputIsUnicode) {
		// Unicode lookup
		UInt16	charIndex = 0;
		if (reinterpret_cast<const UInt8*>(lookupBase) == pageBase) {
			// leave charIndex == 0 : pass with no rules
		}
		else {
			UInt8	plane = inChar >> 16;
			const UInt8*	pageMap = 0;
			if (bSupplementaryChars) {
				if ((plane < 17) && (READ(planeMap[plane]) != 0xff)) {
					pageMap = reinterpret_cast<const UInt8*>(pageBase + 256 * READ(planeMap[plane]));
					goto GOT_PAGE_MAP;
				}
			}
			else if (plane == 0) {
				pageMap = pageBase;
			GOT_PAGE_MAP:
				UInt8	page = (inChar >> 8) & 0xff;
				if (READ(pageMap[page]) != 0xff) {
					const UInt16*	charMapBase = reinterpret_cast<const UInt16*>(pageBase + 256 * numPageMaps);
					const UInt16*	charMap = charMapBase + 256 * READ(pageMap[page]);
					charIndex = READ(charMap[inChar & 0xff]);
				}
			}
		}
		lookup = lookupBase + charIndex;
	}
	else {
		// byte-oriented lookup
		if (pageBase != reinterpret_cast<const Byte*>(tableHeader)) {
			// dbcsPage present
			long	pageNumber = READ(pageBase[inChar]);
			if (pageNumber == 0)
				// not a valid DBCS lead byte
				lookup = lookupBase + inChar;
			else {
				UInt32	nextChar = inputChar(1);
				if (nextChar == kNeedMoreInput || nextChar == kInvalidChar || nextChar == kUnmappedChar)
					return nextChar;
				if (nextChar == kEndOfText)
					lookup = lookupBase + inChar;
				else {
					lookup = lookupBase + pageNumber * 256 + nextChar;
					if (READ(lookup->rules.type) == kLookupType_IllegalDBCS)
						// illegal DBCS sequence; map lead byte alone
						lookup = lookupBase + inChar;
					else
						matchedLength = 2;
				}
			}
		}
		else
			// single-byte only
			lookup = lookupBase + inChar;
	}

	UInt8	ruleType = READ(lookup->rules.type);
	if (ruleType == kLookupType_StringRules || (ruleType & kLookupType_RuleTypeMask) == kLookupType_ExtStringRules) {
		// process string rule list
		const UInt32*	ruleList = reinterpret_cast<const UInt32*>(stringListBase) + READ(lookup->rules.ruleIndex);
		bool			matched = false;
		bool			allowInsertion = true;
		int ruleCount = READ(lookup->rules.ruleCount);
		if ((ruleType & kLookupType_RuleTypeMask) == kLookupType_ExtStringRules)
			ruleCount += 256 * (ruleType & kLookupType_ExtRuleCountMask);
		for ( ; ruleCount > 0; --ruleCount) {
			const StringRule*	rule = reinterpret_cast<const StringRule*>(stringRuleData + READ(*ruleList));
#ifdef TRACING
if (traceLevel > 0) {
	cerr << "** trying match: ";
	printMatch(rule);
	cerr << "\n";
}
#endif
			ruleList++;

			matchElems = READ(rule->matchLength);
			if (matchElems == 0 && allowInsertion == false)
				continue;
			patternLength = matchElems + READ(rule->postLength);
			pattern = reinterpret_cast<const MatchElem*>(rule + 1);	// point past the defined struct for the rule header
			direction = 1;
			infoLimit = matchElems;

			// clear junk...
			for (int i = 0; i < infoLimit; ++i)
				info[i].matchedSpan.start = info[i].matchedSpan.limit = 0;

			UInt32	mr = match(0, 0, 0);
			if (mr == matchYes) {
				if (matchedLength == 0 && allowInsertion == false)
					continue;
				pattern += patternLength;
				patternLength = READ(rule->preLength);
				if (patternLength > 0) {
					direction = -1;
					infoLimit = 0;
					matchElems = -1;
					mr = match(0, 0, -1);
				}
				if (mr == matchYes) {
					// RULE MATCHED! execute it
#ifdef TRACING
if (traceLevel > 0) {
	cerr << "** MATCHED:";
	printMatch(rule);
	cerr << "\n";

	cerr << "** RANGES:";
	for (int i = 0; i < READ(rule->matchLength); ++i) {
		cerr << " <" << info[i].matchedSpan.start << ":" << info[i].matchedSpan.limit << ">";
	}
	cerr << "\n";

	cerr << "** REPLACEMENT:";
	printRep(rule);
	cerr << "\n";

	cerr << "** GENERATES:";
}
#endif
					const RepElem*	r = reinterpret_cast<const RepElem*>(pattern + patternLength);
					for (int i = 0; i < READ(rule->repLength); ++i, ++r) {
#ifdef TRACING
if (traceLevel > 0)
	cerr << " <";
#endif
						switch (READ(r->flags.type)) {
							case kRepElem_Literal:
								outputChar(READ(r->value));
#ifdef TRACING
if (traceLevel > 0)
	cerr << (int)READ(r->value);
#endif
								break;

							case kRepElem_Class:
								{
									const MatchInfo&	myInfo = info[READ(r->flags.matchIndex)];
									if (myInfo.matchedSpan.start < myInfo.matchedSpan.limit) {
										outputChar(repClassMember(READ(r->flags.repClass), myInfo.classIndex));
#ifdef TRACING
if (traceLevel > 0)
	cerr << (int)repClassMember(READ(r->flags.repClass), myInfo.classIndex);
#endif
									}
								}
								break;

							case kRepElem_Copy:
								{
									const MatchInfo*	myInfo = &info[READ(r->flags.matchIndex)];
									for (int j = myInfo->matchedSpan.start; j < myInfo->matchedSpan.limit; ++j) {
										outputChar(inputChar(j));
#ifdef TRACING
if (traceLevel > 0)
	cerr << (j > myInfo->matchedSpan.start ? "," : "") << (int)inputChar(j);
#endif
									}
								}
								break;

							case kRepElem_Unmapped:
								if (bOutputIsUnicode == bInputIsUnicode) {
									outputChar(inChar);
#ifdef TRACING
if (traceLevel > 0)
	cerr << (int)inChar;
#endif
								}
								else {
									switch (converter->unmappedBehavior) {
										case kOptionsUnmapped_DontUseReplacementChar:
											return kUnmappedChar;

										case kOptionsUnmapped_UseReplacementCharWithWarning:
											converter->warningStatus |= kStatus_UsedReplacement;
											// fall through

										default:	// case kOptionsUnmapped_UseReplacementCharSilently:
											outputChar(READ(tableHeader->replacementChar));
											break;
									}
#ifdef TRACING
if (traceLevel > 0)
	cerr << (int)READ(tableHeader->replacementChar);
#endif
								}
								break;
						}
#ifdef TRACING
if (traceLevel > 0)
	cerr << ">";
#endif
					}
#ifdef TRACING
if (traceLevel > 0)
	cerr << endl;
#endif
					if (matchedLength > 0) {
						// we've matched the current input character, so break the loop
						matched = true;
						break;
					}
					else {
						// must have been an insertion (or null!) rule, so skip any further insertion rules
						allowInsertion = false;
					}
				}
				else if (mr != matchNo) {
					return mr;
				}
			}
			else if (mr != matchNo) {
				return mr;
			}
		}
		if (!matched) {
			// no rule matched the current input char, so we simulate a default "Unmapped" lookup
			if (bOutputIsUnicode == bInputIsUnicode)
				// B->B or U->U simply copies the input to the output
				outputChar(inChar);
			else {
				// B->U or U->B uses the replacement char or fails, depending on options
				switch (converter->unmappedBehavior) {
					case kOptionsUnmapped_DontUseReplacementChar:
						return kUnmappedChar;

					case kOptionsUnmapped_UseReplacementCharWithWarning:
						converter->warningStatus |= kStatus_UsedReplacement;
						// fall through

					default:	// case kOptionsUnmapped_UseReplacementCharSilently:
						outputChar(READ(tableHeader->replacementChar));
						break;
				}
			}
			matchedLength = 1;
		}
	}
	else if (ruleType == kLookupType_Unmapped) {
		if (bOutputIsUnicode == bInputIsUnicode)
			outputChar(inChar);
		else {
			switch (converter->unmappedBehavior) {
				case kOptionsUnmapped_DontUseReplacementChar:
					return kUnmappedChar;

				case kOptionsUnmapped_UseReplacementCharWithWarning:
					converter->warningStatus |= kStatus_UsedReplacement;
					// fall through

				default:	// case kOptionsUnmapped_UseReplacementCharSilently:
					outputChar(READ(tableHeader->replacementChar));
					break;
			}
		}
	}
	else {
		// direct character output
		if (bOutputIsUnicode) {
			UInt32	usv = READ(lookup->usv);
			if (usv <= 0x0010ffff)
				outputChar(usv);
		}
		else {
			for (int i = 0; i < READ(lookup->bytes.count); ++i)
				outputChar(READ(lookup->bytes.data[i]));
		}
	}

	advanceInput(matchedLength);
	return 0;
}

Converter::Converter(const Byte* inTable, UInt32 inTableSize, bool inForward,
						UInt16 inForm, UInt16 outForm)
	: table(0)
	, finalStage(0)
	, forward(inForward)
	, inputForm(inForm & kForm_EncodingFormMask)
	, outputForm(outForm & kForm_EncodingFormMask)
	, savedCount(0)
	, pendingOutputChar(kInvalidChar)
	, status(kStatus_NoError)
	, warningStatus(0)
{
	finalStage = this;
	UInt16	normForm = 0;
	if (inTable != 0) {
		const FileHeader*	fh = reinterpret_cast<const FileHeader*>(inTable);
		if (READ(fh->type) == kMagicNumberCmp) {
			// the table is compressed; allocate a new buffer and decompress
			uint64_t uncompressedLen = READ(fh->version);

			table = static_cast<Byte*>(malloc(uncompressedLen));
			if (table == 0) {
				status = kStatus_OutOfMemory;
				return;
			}

			if (tectonic_flate_decompress(
						table, &uncompressedLen,
						inTable + 2 * sizeof(UInt32), inTableSize - 2 * sizeof(UInt32)
			) < 0) {
				status = kStatus_InvalidMapping;
				return;
			}

			fh = reinterpret_cast<const FileHeader*>(table);
		}

		if (READ(fh->type) != kMagicNumber) {
			status = kStatus_InvalidMapping;
			return;
		}
		if ((READ(fh->version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000)) {
			status = kStatus_BadMappingVersion;
			return;
		}

		if (table == 0) {
			table = static_cast<Byte*>(malloc(inTableSize));
			if (table == 0) {
				status = kStatus_OutOfMemory;
				return;
			}
			memcpy(table, inTable, inTableSize);
		}

		fh = reinterpret_cast<const FileHeader*>(table);
		const UInt32*	nameOffsets = reinterpret_cast<const UInt32*>(table + sizeof(FileHeader));
		const UInt32*	tableBase = nameOffsets + READ(fh->numNames);
		UInt32			numTables = READ(fh->numFwdTables);
		if (!forward) {
			tableBase += numTables;
			numTables = READ(fh->numRevTables);
		}

		// check that the outputForm matches the output of the mapping
		UInt32	targetFlags = forward ? READ(fh->formFlagsRHS) : READ(fh->formFlagsLHS);
		if ((targetFlags & kFlags_Unicode) != 0) {
			if (outputForm < kForm_UTF8 || outputForm > kForm_UTF32LE) {
				status = kStatus_InvalidForm;
				return;
			}
		}
		else {
			if (outputForm != kForm_Bytes) {
				status = kStatus_InvalidForm;
				return;
			}
		}

		// if converting from Unicode, prefix a Normalizer if the mapping wants it
		UInt32	sourceFlags = forward ? READ(fh->formFlagsLHS) : READ(fh->formFlagsRHS);
		if ((sourceFlags & kFlags_Unicode) != 0) {
			// check that the inputForm is a Unicode form
			if (inputForm < kForm_UTF8 || inputForm > kForm_UTF32LE) {
				status = kStatus_InvalidForm;
				return;
			}
			Stage*	n = 0;
			if ((sourceFlags & kFlags_ExpectsNFD) != 0) {
				n = new Normalizer(false);
				normForm = kForm_NFD;
			}
			else if ((sourceFlags & kFlags_ExpectsNFC) != 0) {
				n = new Normalizer(true);
				normForm = kForm_NFC;
			}
			if (n != 0) {
				n->prevStage = finalStage;
				finalStage = n;
			}
		}
		else {
			// check that the inputForm is bytes
			if (inputForm != kForm_Bytes) {
				status = kStatus_InvalidForm;
				return;
			}
		}

		// create the processing pipeline
		for (UInt32 i = 0; i < numTables; ++i) {
			const TableHeader*	t = reinterpret_cast<const TableHeader*>(table + READ(tableBase[i]));
			Stage*	p = 0;
			switch (READ(t->type)) {
				case kTableType_BB:
				case kTableType_BU:
				case kTableType_UU:
				case kTableType_UB:
					p = new Pass(t, this);
					normForm = 0;
					break;
				case kTableType_NFC:
					p = new Normalizer(true);
					normForm = kForm_NFC;
					break;
				case kTableType_NFD:
					p = new Normalizer(false);
					normForm = kForm_NFD;
					break;
			}
			if (p == 0) {
				status = kStatus_InvalidMapping;
				return;
			}
			p->prevStage = finalStage;
			finalStage = p;
		}
	}
	else {
		// No mapping table provided, so we're mapping Unicode->Unicode,
		// possibly doing normalization and/or encoding form change.
		// Just check here that the input and output encoding forms are valid.
		if (inputForm < kForm_UTF8 || inputForm > kForm_UTF32LE || outputForm < kForm_UTF8 || outputForm > kForm_UTF32LE) {
			status = kStatus_InvalidForm;
			return;
		}
	}

	// if converting to Unicode, add a Normalizer pass at the end if requested
	if (outputForm >= kForm_UTF8 && outputForm <= kForm_UTF32LE) {
		Stage*	n = 0;
		if ((outForm & kForm_NormalizationMask) == kForm_NFD && normForm != kForm_NFD)
			n = new Normalizer(false);
		else if ((outForm & kForm_NormalizationMask) == kForm_NFC && normForm != kForm_NFC)
			n = new Normalizer(true);
		if (n != 0) {
			n->prevStage = finalStage;
			finalStage = n;
		}
	}
}

Converter::~Converter()
{
	if (finalStage != this)
		delete finalStage;

	if (table != 0)
		free(table);

	table = 0;
}

static UInt32
offsetsFromUTF8[6] =	{
	0x00000000UL,
	0x00003080UL,
	0x000E2080UL,
	0x03C82080UL,
	0xFA082080UL,
	0x82082080UL
};

static UInt8
bytesFromUTF8[256] = {
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,
	1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
	2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2, 3,3,3,3,3,3,3,3,4,4,4,4,5,5,5,5
};

static UInt8
firstByteMark[7] = {
	0x00, 0x00, 0xC0, 0xE0, 0xF0, 0xF8, 0xFC
};

static const int halfShift              = 10;
static const UInt32 halfBase            = 0x0010000UL;
static const UInt32 halfMask            = 0x3FFUL;
static const UInt32 kSurrogateHighStart = 0xD800UL;
static const UInt32 kSurrogateHighEnd   = 0xDBFFUL;
static const UInt32 kSurrogateLowStart  = 0xDC00UL;
static const UInt32 byteMask            = 0x000000BFUL;
static const UInt32 byteMark            = 0x00000080UL;

UInt32
Converter::getChar()
{
	if (dataPtr >= savedCount + dataLen)
		return inputComplete ? kEndOfText : kNeedMoreInput;
	if (inputForm == kForm_Bytes)
		return data[dataPtr++];
	return _getCharFn();
}

UInt32
Converter::_getCharFn()
{
//	This is ONLY called from the public getChar() function, which has already done these tests:
//
//	if (dataPtr >= dataLen)
//		return inputComplete ? kEndOfText : kNeedMoreInput;
//
//	if (inputForm == kForm_Bytes)
//		return data[dataPtr++];

	UInt32	rval = 0;

	if (savedCount > 0) {	// the less efficient version is only called if really needed
		rval = _getCharWithSavedBytes();
		return rval;
	}

#define CHECK_AVAIL(x)				\
	if (dataPtr + (x) > dataLen) {	\
		if (inputComplete)			\
			return kInvalidChar;	\
		else {						\
			_savePendingBytes();	\
			return kNeedMoreInput;	\
		}							\
	}

	switch (inputForm) {
		case kForm_UTF8:
			{
				UInt16 extraBytes = bytesFromUTF8[data[dataPtr]];
				CHECK_AVAIL(extraBytes + 1);
				switch (extraBytes) {	// note: code falls through cases!
					case 5:	rval += data[dataPtr++]; rval <<= 6;
					case 4:	rval += data[dataPtr++]; rval <<= 6;
					case 3:	rval += data[dataPtr++]; rval <<= 6;
					case 2:	rval += data[dataPtr++]; rval <<= 6;
					case 1:	rval += data[dataPtr++]; rval <<= 6;
					case 0:	rval += data[dataPtr++];
				};
				rval -= offsetsFromUTF8[extraBytes];
			}
			break;

		case kForm_UTF16BE:
			CHECK_AVAIL(2);
			rval = data[dataPtr++] << 8;
			rval += data[dataPtr++];
			if (rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd) {
				// check that 2 more bytes are available
				dataPtr -= 2;
				CHECK_AVAIL(4);	// if we don't have 4 bytes available, this will return with kNeedMoreInput,
								// and we'll retry from the beginning of the high surrogate once more is available
				dataPtr += 2;
				UInt32	low = data[dataPtr++] << 8;
				low += data[dataPtr++];
				rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
			}
			break;

		case kForm_UTF16LE:
			CHECK_AVAIL(2);
			rval = data[dataPtr++];
			rval += data[dataPtr++] << 8;
			if (rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd) {
				dataPtr -= 2;
				CHECK_AVAIL(4);
				dataPtr += 2;
				UInt32	low = data[dataPtr++];
				low += data[dataPtr++] << 8;
				rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
			}
			break;

		case kForm_UTF32BE:
			CHECK_AVAIL(4);
			rval = data[dataPtr++] << 24;
			rval += data[dataPtr++] << 16;
			rval += data[dataPtr++] << 8;
			rval += data[dataPtr++];
			break;

		case kForm_UTF32LE:
			CHECK_AVAIL(4);
			rval = data[dataPtr++];
			rval += data[dataPtr++] << 8;
			rval += data[dataPtr++] << 16;
			rval += data[dataPtr++] << 24;
			break;
	}

	return rval;
}

UInt32
Converter::_getCharWithSavedBytes()
	// This is a version of _getCharFn() that respects "saved bytes";
	// only call this if (savedCount > 0) because it has additional overhead for every byte read
{
	UInt32	rval = 0;

#undef CHECK_AVAIL
#define CHECK_AVAIL(x)							\
	if (dataPtr + (x) > savedCount + dataLen) {	\
		if (inputComplete)						\
			return kInvalidChar;				\
		else {									\
			_savePendingBytes();				\
			return kNeedMoreInput;				\
		}										\
	}

#undef DATA
#define DATA(x)	(x < savedCount ? savedBytes[x] : data[x - savedCount])

	switch (inputForm) {
		case kForm_UTF8:
			{
				UInt16 extraBytes = bytesFromUTF8[DATA(dataPtr)];
				CHECK_AVAIL(extraBytes + 1);
				switch (extraBytes) {	// note: code falls through cases!
					case 5:	rval += DATA(dataPtr); dataPtr++; rval <<= 6;
					case 4:	rval += DATA(dataPtr); dataPtr++; rval <<= 6;
					case 3:	rval += DATA(dataPtr); dataPtr++; rval <<= 6;
					case 2:	rval += DATA(dataPtr); dataPtr++; rval <<= 6;
					case 1:	rval += DATA(dataPtr); dataPtr++; rval <<= 6;
					case 0:	rval += DATA(dataPtr); dataPtr++;
				};
				rval -= offsetsFromUTF8[extraBytes];
			}
			break;

		case kForm_UTF16BE:
			CHECK_AVAIL(2);
			rval = DATA(dataPtr) << 8; dataPtr++;
			rval += DATA(dataPtr); dataPtr++;
			if (rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd) {
				dataPtr -= 2;
				CHECK_AVAIL(4);
				dataPtr += 2;
				UInt32	low = DATA(dataPtr) << 8; dataPtr++;
				low += DATA(dataPtr); dataPtr++;
				rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
			}
			break;

		case kForm_UTF16LE:
			CHECK_AVAIL(2);
			rval = DATA(dataPtr); dataPtr++;
			rval += DATA(dataPtr) << 8; dataPtr++;
			if (rval >= kSurrogateHighStart && rval <= kSurrogateHighEnd) {
				dataPtr -= 2;
				CHECK_AVAIL(4);
				dataPtr += 2;
				UInt32	low = DATA(dataPtr); dataPtr++;
				low += DATA(dataPtr) << 8; dataPtr++;
				rval = ((rval - kSurrogateHighStart) << halfShift) + (low - kSurrogateLowStart) + halfBase;
			}
			break;

		case kForm_UTF32BE:
			CHECK_AVAIL(4);
			rval = DATA(dataPtr) << 24; dataPtr++;
			rval += DATA(dataPtr) << 16; dataPtr++;
			rval += DATA(dataPtr) << 8; dataPtr++;
			rval += DATA(dataPtr); dataPtr++;
			break;

		case kForm_UTF32LE:
			CHECK_AVAIL(4);
			rval = DATA(dataPtr); dataPtr++;
			rval += DATA(dataPtr) << 8; dataPtr++;
			rval += DATA(dataPtr) << 16; dataPtr++;
			rval += DATA(dataPtr) << 24; dataPtr++;
			break;
	}

	if (dataPtr >= savedCount) {
		dataPtr -= savedCount;
		savedCount = 0;
	}

	return rval;
}

void
Converter::_savePendingBytes()
{
	dataPtr -= savedCount;
	while (dataPtr < dataLen)
		savedBytes[savedCount++] = data[dataPtr++];
}

bool
Converter::IsForward() const
{
	return forward;
}

void
Converter::GetFlags(UInt32& sourceFlags, UInt32& targetFlags) const
{
	const FileHeader*	fh = reinterpret_cast<const FileHeader*>(table);
	if (forward) {
		sourceFlags = READ(fh->formFlagsLHS);
		targetFlags = READ(fh->formFlagsRHS);
	}
	else {
		sourceFlags = READ(fh->formFlagsRHS);
		targetFlags = READ(fh->formFlagsLHS);
	}
}

static bool
getNamePtrFromTable(const Byte* table, UInt16 nameID, const Byte*& outNamePtr, UInt32& outNameLen)
{
	const FileHeader*	fh = reinterpret_cast<const FileHeader*>(table);
	const UInt32*		nameOffsets = reinterpret_cast<const UInt32*>(table + sizeof(FileHeader));
	for (UInt32 i = 0; i < READ(fh->numNames); ++i) {
		const NameRec*	n = reinterpret_cast<const NameRec*>(table + READ(nameOffsets[i]));
		if (READ(n->nameID) == nameID) {
			outNameLen = READ(n->nameLength);
			outNamePtr = reinterpret_cast<const Byte*>(n) + sizeof(NameRec);
			return true;
		}
	}
	return false;
}

bool
Converter::GetNamePtr(UInt16 nameID, const Byte*& outNamePtr, UInt32& outNameLen) const
{
	return getNamePtrFromTable(table, nameID, outNamePtr, outNameLen);
}

TECkit_Status
Converter::ConvertBufferOpt(
	const Byte* inBuffer, UInt32 inLength, UInt32* inUsed,
	Byte* outBuffer, UInt32 outLength, UInt32* outUsed,
	UInt32 inOptions, UInt32* lookaheadCount)
{
	TECkit_Status	rval;
#undef RETURN
#define RETURN(returnStatus)	rval = returnStatus; goto RETURN_LABEL

	UInt32	outPtr = 0;

	data = inBuffer;
	dataLen = inLength;
	dataPtr = 0;
	inputComplete = ((inOptions & kOptionsMask_InputComplete) == kOptionsComplete_InputIsComplete);
	unmappedBehavior = (inOptions & kOptionsMask_UnmappedBehavior);

	UInt32	c;
	if (pendingOutputChar != kInvalidChar) {
		c = pendingOutputChar;
		pendingOutputChar = kInvalidChar;
		goto GOT_CHAR;
	}
	while (1) {
		c = finalStage->getChar();
	GOT_CHAR:
		switch (c) {
			case kEndOfText:
				RETURN(kStatus_NoError);

			case kNeedMoreInput:
				RETURN(kStatus_NeedMoreInput);

			case kInvalidChar:
				RETURN(kStatus_IncompleteChar);

			case kUnmappedChar:
				RETURN(kStatus_UnmappedChar);

			default:
				switch (outputForm) {
					case kForm_Bytes:
						if (outPtr == outLength) {
							pendingOutputChar = c;
							RETURN(kStatus_OutputBufferFull);
						}
						outBuffer[outPtr++] = c;
						break;

					case kForm_UTF8:
						{
							int	bytesToWrite;
							if (c < 0x80) {				bytesToWrite = 1;
							} else if (c < 0x800) {		bytesToWrite = 2;
							} else if (c < 0x10000) {	bytesToWrite = 3;
							} else if (c < 0x200000) {	bytesToWrite = 4;
							} else {					bytesToWrite = 2;
														c = 0x0000fffd;
							};
							if (outPtr + bytesToWrite > outLength) {
								pendingOutputChar = c;
								RETURN (kStatus_OutputBufferFull);
							}
							outPtr += bytesToWrite;
							switch (bytesToWrite) {	/* note: code falls through cases! */
								case 4:	outBuffer[--outPtr] = (c | byteMark) & byteMask; c >>= 6;
								case 3:	outBuffer[--outPtr] = (c | byteMark) & byteMask; c >>= 6;
								case 2:	outBuffer[--outPtr] = (c | byteMark) & byteMask; c >>= 6;
								case 1:	outBuffer[--outPtr] =  c | firstByteMark[bytesToWrite];
							};
							outPtr += bytesToWrite;
						}
						break;

					case kForm_UTF16BE:
						if (c < 0x00010000) {
							if (outPtr + 2 > outLength) {
								pendingOutputChar = c;
								RETURN (kStatus_OutputBufferFull);
							}
							outBuffer[outPtr++] = c >> 8;
							outBuffer[outPtr++] = c;
						}
						else {
							if (outPtr + 4 > outLength) {
								pendingOutputChar = c;
								RETURN (kStatus_OutputBufferFull);
							}
							c -= halfBase;
							UInt32	hi = (c >> halfShift) + kSurrogateHighStart;
							UInt32	lo = (c & halfMask) + kSurrogateLowStart;
							outBuffer[outPtr++] = hi >> 8;
							outBuffer[outPtr++] = hi;
							outBuffer[outPtr++] = lo >> 8;
							outBuffer[outPtr++] = lo;
						}
						break;

					case kForm_UTF16LE:
						if (c < 0x00010000) {
							if (outPtr + 2 > outLength) {
								pendingOutputChar = c;
								RETURN (kStatus_OutputBufferFull);
							}
							outBuffer[outPtr++] = c;
							outBuffer[outPtr++] = c >> 8;
						}
						else {
							if (outPtr + 4 > outLength) {
								pendingOutputChar = c;
								RETURN (kStatus_OutputBufferFull);
							}
							c -= halfBase;
							UInt32	hi = (c >> halfShift) + kSurrogateHighStart;
							UInt32	lo = (c & halfMask) + kSurrogateLowStart;
							outBuffer[outPtr++] = hi;
							outBuffer[outPtr++] = hi >> 8;
							outBuffer[outPtr++] = lo;
							outBuffer[outPtr++] = lo >> 8;
						}
						break;

					case kForm_UTF32BE:
						if (outPtr + 4 > outLength) {
							pendingOutputChar = c;
							RETURN (kStatus_OutputBufferFull);
						}
						outBuffer[outPtr++] = c >> 24;
						outBuffer[outPtr++] = c >> 16;
						outBuffer[outPtr++] = c >> 8;
						outBuffer[outPtr++] = c;
						break;

					case kForm_UTF32LE:
						if (outPtr + 4 > outLength) {
							pendingOutputChar = c;
							RETURN (kStatus_OutputBufferFull);
						}
						outBuffer[outPtr++] = c;
						outBuffer[outPtr++] = c >> 8;
						outBuffer[outPtr++] = c >> 16;
						outBuffer[outPtr++] = c >> 24;
						break;
				}
				break;
		}
	}
	RETURN (kStatus_NoError);

RETURN_LABEL:
	if (inUsed)
		*inUsed = dataPtr;
	if (outUsed)
		*outUsed = outPtr;
	if (lookaheadCount) {
		*lookaheadCount = 0;
		Stage*	s = finalStage;
		while (s != this) {
			*lookaheadCount += s->lookaheadCount();
			s = s->prevStage;
		}
	}

	rval |= warningStatus;
	if ((rval & kStatusMask_Basic) == kStatus_NoError)
		Reset();

	return rval;
}

void
Converter::Reset()
{
	pendingOutputChar = kInvalidChar;
	savedCount = 0;
	dataPtr = 0;
	dataLen = 0;
	warningStatus = 0;
	Stage*	s = finalStage;
	while (s != this) {
		s->Reset();
		s = s->prevStage;
	}
}

bool
Converter::Validate(const Converter* cnv)
{
	if (!cnv)
		return false;
	if (cnv->status != kStatus_NoError)
		return false;
	if (cnv->table != 0) {
		const FileHeader*	fh = reinterpret_cast<const FileHeader*>(cnv->table);
		if (READ(fh->type) != kMagicNumber)
			return false;
	}
	return true;
}

TECkit_Status
WINAPI
TECkit_CreateConverter(
	Byte*				mapping,
	UInt32				mappingSize,
	Byte				mapForward,
	UInt16				inputForm,
	UInt16				outputForm,
	TECkit_Converter*	converter)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = 0;
	*converter = 0;
	cnv = new Converter(mapping, mappingSize, mapForward, inputForm, outputForm);
	status = cnv->creationStatus();
	if (status == kStatus_NoError)
		*converter = reinterpret_cast<TECkit_Converter>(cnv);
	else
		delete cnv;
	return status;
}

TECkit_Status
WINAPI
TECkit_DisposeConverter(
	TECkit_Converter	converter)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else
		delete cnv;
	return status;
}

TECkit_Status
WINAPI
TECkit_GetConverterName(
	TECkit_Converter	converter,
	UInt16				nameID,
	Byte*				nameBuffer,
	UInt32				bufferSize,
	UInt32*				nameLength)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else {
		const Byte*	namePtr;
		if (cnv->GetNamePtr(nameID, namePtr, *nameLength)) {
			UInt16	copyBytes = *nameLength < bufferSize ? *nameLength : bufferSize;
			if (copyBytes > 0)
				memcpy(nameBuffer, namePtr, copyBytes);
		}
		else
			status = kStatus_NameNotFound;
	}
	return status;
}

TECkit_Status
WINAPI
TECkit_GetConverterFlags(
	TECkit_Converter	converter,
	UInt32*				sourceFlags,
	UInt32*				targetFlags)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else
		cnv->GetFlags(*sourceFlags, *targetFlags);
	return status;
}

TECkit_Status
WINAPI
TECkit_ResetConverter(
	TECkit_Converter	converter)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else
		cnv->Reset();
	return status;
}

TECkit_Status
WINAPI
TECkit_ConvertBufferOpt(
	TECkit_Converter	converter,
	const Byte*			inBuffer,
	UInt32				inLength,
	UInt32*				inUsed,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	UInt32				inOptions,
	UInt32*				lookaheadCount)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else
		status = cnv->ConvertBufferOpt(inBuffer, inLength, inUsed, outBuffer, outLength, outUsed, inOptions, lookaheadCount);
	return status;
}

TECkit_Status
WINAPI
TECkit_ConvertBuffer(
	TECkit_Converter	converter,
	const Byte*			inBuffer,
	UInt32				inLength,
	UInt32*				inUsed,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	Byte				inputIsComplete)
{
	return TECkit_ConvertBufferOpt(converter, inBuffer, inLength, inUsed, outBuffer, outLength, outUsed,
			kOptionsUnmapped_UseReplacementCharSilently + (inputIsComplete ? kOptionsComplete_InputIsComplete : 0), 0);
}

TECkit_Status
WINAPI
TECkit_FlushOpt(
	TECkit_Converter	converter,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	UInt32				inOptions,
	UInt32*				lookaheadCount)
{
	TECkit_Status	status = kStatus_NoError;
	Converter*	cnv = reinterpret_cast<Converter*>(converter);
	if (!Converter::Validate(cnv))
		status = kStatus_InvalidConverter;
	else
		status = cnv->ConvertBufferOpt(0, 0, 0, outBuffer, outLength, outUsed,
			inOptions | kOptionsComplete_InputIsComplete, lookaheadCount);
	return status;
}

TECkit_Status
WINAPI
TECkit_Flush(
	TECkit_Converter	converter,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed)
{
	return TECkit_FlushOpt(converter, outBuffer, outLength, outUsed, kOptionsUnmapped_UseReplacementCharSilently, 0);
}

TECkit_Status
WINAPI
TECkit_GetMappingFlags(
	Byte*				mapping,
	UInt32				mappingSize,
	UInt32*				lhsFlags,
	UInt32*				rhsFlags)
{
	TECkit_Status	status = kStatus_NoError;
	if (mapping == 0)
		status = kStatus_InvalidMapping;
	else {
		const FileHeader*	fh = reinterpret_cast<const FileHeader*>(mapping);
		FileHeader			header;
		if (READ(fh->type) == kMagicNumberCmp) {
			// compressed mapping, so we need to decompress enough of it to read the flags
			uint64_t uncompressedLen = sizeof(FileHeader);

			if (tectonic_flate_decompress(
						reinterpret_cast<Byte*>(&header), &uncompressedLen,
						mapping + 2 * sizeof(UInt32), mappingSize - 2 * sizeof(UInt32)
			) != FlateResult_BufError) {
				status = kStatus_InvalidMapping;
			}

			fh = &header;
		}
		if (status == kStatus_NoError && READ(fh->type) == kMagicNumber) {
			if ((READ(fh->version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000))
				status = kStatus_BadMappingVersion;
			else {
				*lhsFlags = READ(fh->formFlagsLHS);
				*rhsFlags = READ(fh->formFlagsRHS);
			}
		}
		else
			status = kStatus_InvalidMapping;
	}
	return status;
}

TECkit_Status
WINAPI
TECkit_GetMappingName(
	Byte*				mapping,
	UInt32				mappingSize,
	UInt16				nameID,
	Byte*				nameBuffer,
	UInt32				bufferSize,
	UInt32*				nameLength)
{
	void*	buf = 0;
	TECkit_Status	status = kStatus_NoError;
	if (mapping == 0)
		status = kStatus_InvalidMapping;
	else {
		const FileHeader*	fh = reinterpret_cast<const FileHeader*>(mapping);
		FileHeader			header;
		if (READ(fh->type) == kMagicNumberCmp) {
			// compressed mapping, so we need to decompress the fixed header to read the headerLength field,
			// and then decompress the complete header to get the names
			uint64_t uncompressedLen = sizeof(FileHeader);

			if (tectonic_flate_decompress(
						reinterpret_cast<Byte*>(&header), &uncompressedLen,
						mapping + 2 * sizeof(UInt32), mappingSize - 2 * sizeof(UInt32)
			) != FlateResult_BufError) {
				status = kStatus_InvalidMapping;
			} else {
				fh = &header;
				uncompressedLen = READ(fh->headerLength);
				buf = malloc(uncompressedLen);

				if (buf == 0)
					status = kStatus_OutOfMemory;
				else {
					if (tectonic_flate_decompress(
								static_cast<Byte*>(buf), &uncompressedLen,
								mapping + 2 * sizeof(UInt32), mappingSize - 2 * sizeof(UInt32)
					) != FlateResult_BufError) {
						status = kStatus_InvalidMapping;
					}

					fh = static_cast<const FileHeader*>(buf);
				}
			}
		}
		if (status == kStatus_NoError && READ(fh->type) == kMagicNumber) {
			if ((READ(fh->version) & 0xFFFF0000) > (kCurrentFileVersion & 0xFFFF0000))
				status = kStatus_BadMappingVersion;
			else {
				const Byte*	namePtr;
				if (getNamePtrFromTable(reinterpret_cast<const Byte*>(fh), nameID, namePtr, *nameLength)) {
					UInt16	copyBytes = *nameLength < bufferSize ? *nameLength : bufferSize;
					if (copyBytes > 0)
						memcpy(nameBuffer, namePtr, copyBytes);
				}
				else
					status = kStatus_NameNotFound;
			}
		}
		else
			status = kStatus_InvalidMapping;
		if (buf != 0)
			free(buf);
	}
	return status;
}

UInt32
WINAPI
TECkit_GetVersion()
{
	return kCurrentTECkitVersion;
}
