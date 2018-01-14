/*------------------------------------------------------------------------
Copyright (C) 2002-2016 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.

File: TECkit_Common.h
Responsibility: Jonathan Kew
Last reviewed: Not yet.

Description:
    Public definitions used by TECkit engine and compiler
-------------------------------------------------------------------------*/

/*
	Common types and defines for the engine and compiler

History:
	16-Sep-2006		jk	updated version to 2.4 (adding new compiler APIs for Bob E)
	23-May-2005		jk	patch for 64-bit architectures (thanks to Ulrik P)
	18-Mar-2005		jk	updated minor version for 2.3 (engine unchanged, XML option in compiler)
	23-Sep-2003		jk	updated for version 2.1 - extended status values
	xx-xxx-2002		jk	version 2.0 initial release
*/

#ifndef __TECkit_Common_H__
#define __TECkit_Common_H__

#define	kCurrentTECkitVersion	0x00020004	/* 16.16 version number */

#ifndef __MACTYPES__
#ifndef MAC_TYPES	/* these are all predefined if using a Mac prefix */
typedef unsigned char			UInt8;
typedef unsigned short			UInt16;
typedef unsigned int			UInt32;	/* NB: assumes int is 4 bytes */
#ifndef ZCONF_H /* n.b. if also using zlib.h, it must precede TECkit headers */
typedef UInt8					Byte;
#endif
typedef Byte*					BytePtr;
typedef UInt16					UniChar;

typedef char*					Ptr;
typedef Byte*					TextPtr;
#endif
#endif

/*
	all public functions return a status code
*/
typedef long					TECkit_Status;

/*
	possible TECkit_Status return values
*/
#define	kStatus_NoError				0	/* this is usually the desired result! */

/* positive values are informational status values */
/* low byte is the basic status of the conversion process */
#define kStatusMask_Basic			0x000000FF
#define kStatus_OutputBufferFull	1	/* ConvertBuffer or Flush: output buffer full, so not all input was processed */
#define kStatus_NeedMoreInput		2	/* ConvertBuffer: processed all input data, ready for next chunk */

/* only returned in version 2.1 or later, with DontUseReplacementChar option */
#define kStatus_UnmappedChar		3	/* ConvertBuffer or Flush: stopped at unmapped character */

/* additional warning status in 2.1, only returned if 2.1-specific options are used */
/* one byte of the status value is used for warning flags */
#define kStatusMask_Warning			0x0000FF00
#define kStatus_UsedReplacement     0x00000100	/* ConvertBuffer or Flush: used default replacement character during mapping */

/* negative values are errors */
#define kStatus_InvalidForm			-1	/* inForm or outForm parameter doesn't match mapping (bytes/Unicode mismatch) */
#define kStatus_ConverterBusy		-2	/* can't initiate a conversion, as the converter is already in the midst of an operation */
#define kStatus_InvalidConverter	-3	/* converter object is corrupted (or not really a TECkit_Converter at all) */
#define kStatus_InvalidMapping		-4	/* compiled mapping data is not recognizable */
#define kStatus_BadMappingVersion	-5	/* compiled mapping is not a version we can handle */
#define kStatus_Exception			-6	/* an internal error has occurred */
#define kStatus_NameNotFound		-7	/* couldn't find the requested name in the compiled mapping */
#define kStatus_IncompleteChar		-8	/* bad input data (lone surrogate, incomplete UTF8 sequence) */
#define kStatus_CompilationFailed	-9	/* mapping compilation failed (syntax errors, etc) */
#define kStatus_OutOfMemory			-10	/* unable to allocate required memory */

/*
	encoding form constants for TECkit_CreateConverter and TECkit_Compile
*/
#define	kForm_EncodingFormMask		0x000F
#define kForm_Unspecified			0	/* invalid as argument to TECkit_CreateConverter */
#define	kForm_Bytes					1
#define kForm_UTF8					2
#define kForm_UTF16BE				3
#define kForm_UTF16LE				4
#define kForm_UTF32BE				5
#define kForm_UTF32LE				6

#endif
