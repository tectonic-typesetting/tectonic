/*------------------------------------------------------------------------
Copyright (c) 2008-2014 SIL International. All rights reserved.

Distributable under the terms of either the Common Public License or the
GNU Lesser General Public License, as specified in the LICENSING.txt file.
------------------------------------------------------------------------*/

// version_defs.h -- common pieces included in all the version resources

#include <windows.h>

// version numbers that should be updated for each release
#define VER_FILEVERSION             2,5,4,0
#define VER_FILEVERSION_STR         "2.5.4.0\0"

#define VER_PRODUCTVERSION          2,5,4,0
#define VER_PRODUCTVERSION_STR      "2.5.4\0"

// constants used in all the binaries, shouldn't need changing (until next year)
#define VER_COMPANY_NAME_STR        "SIL International\0"
#define VER_COPYRIGHT_STR           "Copyright (c) 2002-2014\0"
#define VER_PRODUCT_NAME_STR        "TECkit core\0"

#ifndef DEBUG
#define VER_DEBUG                   0
#else
#define VER_DEBUG                   VS_FF_DEBUG
#endif

