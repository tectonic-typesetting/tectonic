/*------------------------------------------------------------------------
Copyright (C) 2002-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Engine.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public API to the TECkit conversion engine.
-------------------------------------------------------------------------*/

/*
	TECkit_Engine.h
	
	Public API to the TECkit encoding conversion library.
	
	18-Jan-2008		jk	added EXPORTED to declarations, for mingw32 cross-build
	18-Mar-2005		jk	moved version number to TECkit_Common.h as it is shared with the compiler
	19-Mar-2004		jk	updated minor version for 2.2 engine (improved matching functionality)
	23-Sep-2003		jk	updated for version 2.1 - new "...Opt" APIs
	 5-Jul-2002		jk	corrected placement of WINAPI to keep MS compiler happy
	14-May-2002		jk	added WINAPI to function declarations
	22-Dec-2001		jk	initial version
*/

#ifndef __TECkit_Engine_H__
#define __TECkit_Engine_H__

#include "TECkit_Common.h"

/* formFlags bits for normalization; if none are set, then this side of the mapping is normalization-form-agnostic on input, and may generate an unspecified mixture */
#define kFlags_ExpectsNFC		0x00000001	/* expects fully composed text (NC) */
#define kFlags_ExpectsNFD		0x00000002	/* expects fully decomposed text (NCD) */
#define kFlags_GeneratesNFC		0x00000004	/* generates fully composed text (NC) */
#define kFlags_GeneratesNFD		0x00000008	/* generates fully decomposed text (NCD) */

/* if VisualOrder is set, this side of the mapping deals with visual-order rather than logical-order text (only relevant for bidi scripts) */
#define kFlags_VisualOrder		0x00008000	/* visual rather than logical order */

/* if Unicode is set, the encoding is Unicode on this side of the mapping */
#define kFlags_Unicode			0x00010000	/* this is Unicode rather than a byte encoding */

/* required names */
#define kNameID_LHS_Name		0		/* "source" or LHS encoding name, e.g. "SIL-EEG_URDU-2001" */
#define kNameID_RHS_Name		1		/* "destination" or RHS encoding name, e.g. "UNICODE-3-1" */
#define kNameID_LHS_Description	2		/* source encoding description, e.g. "SIL East Eurasia Group Extended Urdu (Mac OS)" */
#define kNameID_RHS_Description	3		/* destination description, e.g. "Unicode 3.1" */
/* additional recommended names (parallel to UTR-22) */
#define kNameID_Version			4		/* "1.0b1" */
#define kNameID_Contact			5		/* "mailto:nrsi@sil.org" */
#define kNameID_RegAuthority	6		/* "SIL International" */
#define kNameID_RegName			7		/* "Greek (Galatia)" */
#define kNameID_Copyright		8		/* "(c)2002 SIL International" */
/* additional name IDs may be defined in the future */

/*
	encoding form options for TECkit_CreateConverter
*/
#define	kForm_NormalizationMask		0x0F00
#define	kForm_NFC					0x0100
#define	kForm_NFD					0x0200

/*
	end of text value for TECkit_DataSource functions to return
*/
#define	kEndOfText					0xffffffffUL

/*
	A converter object is an opaque pointer
*/
typedef struct Opaque_TECkit_Converter*		TECkit_Converter;

#if defined(__cplusplus)
extern "C" {
#endif

#ifdef _WIN32
/* MS compiler has predefined _WIN32, so assume Windows target  */
#include <windows.h>
#undef WINAPI
#define WINAPI
#define EXPORTED
#else
/* not the MS compiler, so try Metrowerks' platform macros */
#ifndef __APPLE__
#if defined __dest_os && (__dest_os == __win32_os)
#include <windef.h>
#endif
#endif
#endif

#ifndef WINAPI
#define WINAPI
#define CALLBACK
#endif

/* this seems to be needed for a gcc-mingw32 build to work... */
#ifndef EXPORTED
#ifdef _WIN32
#define EXPORTED __declspec(dllexport)
#else
#define EXPORTED
#endif
#endif

/*
	Create a converter object from a compiled mapping
*/
TECkit_Status
WINAPI EXPORTED
TECkit_CreateConverter(
	Byte*				mapping,
	UInt32				mappingSize,
	Byte				mapForward,
	UInt16				sourceForm,
	UInt16				targetForm,
	TECkit_Converter*	converter);

/*
	Dispose of a converter object
*/
TECkit_Status
WINAPI EXPORTED
TECkit_DisposeConverter(
	TECkit_Converter	converter);

/*
	Read a name record or the flags from a converter object
*/
TECkit_Status
WINAPI EXPORTED
TECkit_GetConverterName(
	TECkit_Converter	converter,
	UInt16				nameID,
	Byte*				nameBuffer,
	UInt32				bufferSize,
	UInt32*				nameLength);

TECkit_Status
WINAPI EXPORTED
TECkit_GetConverterFlags(
	TECkit_Converter	converter,
	UInt32*				sourceFlags,
	UInt32*				targetFlags);

/*
	Reset a converter object, forgetting any buffered context/state
*/
TECkit_Status
WINAPI EXPORTED
TECkit_ResetConverter(
	TECkit_Converter	converter);

/*
	Convert text from a buffer in memory
*/
TECkit_Status
WINAPI EXPORTED
TECkit_ConvertBuffer(
	TECkit_Converter	converter,
	const Byte*			inBuffer,
	UInt32				inLength,
	UInt32*				inUsed,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	Byte				inputIsComplete);

/*
	Flush any buffered text from a converter object
	(at end of input, if inputIsComplete flag not set for ConvertBuffer)
*/
TECkit_Status
WINAPI EXPORTED
TECkit_Flush(
	TECkit_Converter	converter,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed);


/*
	Read name and flags directly from a compiled mapping, before making a converter object
*/
TECkit_Status
WINAPI EXPORTED
TECkit_GetMappingName(
	Byte*				mapping,
	UInt32				mappingSize,
	UInt16				nameID,
	Byte*				nameBuffer,
	UInt32				bufferSize,
	UInt32*				nameLength);

TECkit_Status
WINAPI EXPORTED
TECkit_GetMappingFlags(
	Byte*				mapping,
	UInt32				mappingSize,
	UInt32*				lhsFlags,
	UInt32*				rhsFlags);

/*
	Return the version number of the TECkit library
*/
UInt32
WINAPI EXPORTED
TECkit_GetVersion();

/*
	***** New APIs for version 2.1 of the engine *****

	A converter object now has options to control behavior when "unmappable" characters
	occur in the input text.
	Choices are:
		UseReplacementCharSilently
			- original behavior, just uses "replacement character" in the mapping
		UseReplacementCharWithWarning
			- do the same mapping, but return a warning in the status value
		DontUseReplacementChar
			- stop conversion, returning immediately on encountering an unmapped character
*/

#define kOptionsMask_UnmappedBehavior					0x000F
#define kOptionsUnmapped_UseReplacementCharSilently		  0x00
#define kOptionsUnmapped_UseReplacementCharWithWarning	  0x01
#define kOptionsUnmapped_DontUseReplacementChar			  0x02

#define kOptionsMask_InputComplete						0x0100
#define kOptionsComplete_InputIsComplete				0x0100

/*
	Convert text from a buffer in memory, with options
	(note that former inputIsComplete flag is now a bit in the options parameter)
*/
TECkit_Status
WINAPI EXPORTED
TECkit_ConvertBufferOpt(
	TECkit_Converter	converter,
	const Byte*			inBuffer,
	UInt32				inLength,
	UInt32*				inUsed,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	UInt32				inOptions,
	UInt32*				lookaheadCount);

/*
	Flush any buffered text from a converter object, with options
	(at end of input, if inputIsComplete flag not set for ConvertBuffer)
*/
TECkit_Status
WINAPI EXPORTED
TECkit_FlushOpt(
	TECkit_Converter	converter,
	Byte*				outBuffer,
	UInt32				outLength,
	UInt32*				outUsed,
	UInt32				inOptions,
	UInt32*				lookaheadCount);


#if defined(__cplusplus)
}	/* extern "C" */
#endif

#endif /* __TECkit_Engine_H__ */
