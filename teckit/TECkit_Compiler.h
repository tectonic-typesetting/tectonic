/*
	TECkit_Compiler.h
	
	Public API to the TECkit compiler library.

	18-Jan-2008		jk	added EXPORTED to declarations, for mingw32 cross-build
	
	Jonathan Kew	22-Dec-2001
					14-May-2002		added WINAPI to function declarations
					 5-Jul-2002		corrected placement of WINAPI/CALLBACK to keep MS compiler happy
					18-Mar-2005		added option to generate XML representation
					21-May-2005		changes based on Ulrik Petersen's patch for MS VC++ 6
					16-Sep-2006		added APIs to convert USVs to names and vice versa
*/

/*
	TECkit_Compiler.h
	Copyright (c) 2002-2014 SIL International.
*/

#ifndef __TECkit_Compiler_H__
#define __TECkit_Compiler_H__

#include "TECkit_Common.h"

#ifdef __cplusplus
extern "C" {
#endif

#ifdef _WIN32
/* MS compiler has predefined _WIN32, so assume Windows target  */
#include <windows.h>	// apparently just using windef.h fails on VC++6
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

#ifndef EXPORTED
#ifdef _WIN32
#define EXPORTED __declspec(dllexport)
#else
#define EXPORTED
#endif
#endif

#define kCompilerOpts_FormMask	0x0000000F	/* see TECkit_Common.h for encoding form constants */
#define kCompilerOpts_Compress	0x00000010	/* generate compressed mapping table */
#define kCompilerOpts_XML		0x00000020	/* instead of a compiled binary table, generate an XML representation of the mapping */

typedef void (CALLBACK *TECkit_ErrorFn)(void* userData, const char* msg, const char* param, UInt32 line);

TECkit_Status
WINAPI EXPORTED
TECkit_Compile(char* txt, UInt32 len, Byte doCompression, TECkit_ErrorFn errFunc, void* userData, Byte** outTable, UInt32* outLen);

TECkit_Status
WINAPI EXPORTED
TECkit_CompileOpt(char* txt, UInt32 len, TECkit_ErrorFn errFunc, void* userData, Byte** outTable, UInt32* outLen, UInt32 opts);

void
WINAPI EXPORTED
TECkit_DisposeCompiled(Byte* table);

UInt32
WINAPI EXPORTED
TECkit_GetCompilerVersion();

/* new APIs for looking up Unicode names (as NUL-terminated C strings) */
const char*
WINAPI EXPORTED
TECkit_GetUnicodeName(UInt32 usv);
	/* returns the Unicode name of usv, if available, else NULL */

char*
WINAPI EXPORTED
TECkit_GetTECkitName(UInt32 usv);
	/* returns the TECkit form of the name of usv, or "U+xxxx" if no name */
	/* NB: returns value is a pointer to a static string, which will be
	   overwritten by subsequent calls */

int
WINAPI EXPORTED
TECkit_GetUnicodeValue(char* name);
	/* returns Unicode value for a Unicode or TECkit name, or -1 if not known */

#ifdef __cplusplus
}
#endif

#endif
