/****************************************************************************\
   xetexextra.h: banner etc. for XeTeX.

   This is included by XeTeX, from xetexextra.c.

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

/*
Copyright (C) 1995, 96 Karl Berry.
Copyright (C) 2004 Olaf Weber.
Copyright (C) 2004 Jonathan Kew/SIL International.

This program is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 2, or (at your option)
any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; if not, write to the Free Software
Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.  */

#include <xetexdir/etex_version.h> /* for ETEX_VERSION */
#include <xetexdir/xetex_version.h> /* for XETEX_VERSION */

#define BANNER "This is XeTeX, Version 3.14159265-" ETEX_VERSION "-" XETEX_VERSION
#define COPYRIGHT_HOLDER "SIL International, Jonathan Kew and Khaled Hosny"
#define AUTHOR "Jonathan Kew"
#define PROGRAM_HELP XETEXHELP
#define BUG_ADDRESS "xetex@tug.org"
#define DUMP_VAR TEX_format_default
#define DUMP_LENGTH_VAR format_default_length
#define DUMP_OPTION "fmt"
#define DUMP_EXT ".fmt"
#define INI_PROGRAM "xeinitex"
#define VIR_PROGRAM "xevirtex"
