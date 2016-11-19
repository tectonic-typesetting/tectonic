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

#ifndef __XeTeXswap_H
#define __XeTeXswap_H

#include <unicode/platform.h>	// ICU's platform.h defines U_IS_BIG_ENDIAN for us

static inline uint16_t
SWAP16(const uint16_t p)
{
#if U_IS_BIG_ENDIAN
	return p;
#else
	return (p >> 8) + (p << 8);
#endif
}

static inline uint32_t
SWAP32(const uint32_t p)
{
#if U_IS_BIG_ENDIAN
	return p;
#else
	return (p >> 24) + ((p >> 8) & 0x0000ff00) + ((p << 8) & 0x00ff0000) + (p << 24);
#endif
}

#ifdef __cplusplus
static inline uint16_t
SWAP(uint16_t p)
{
	return SWAP16(p);
}

static inline uint32_t
SWAP(uint32_t p)
{
	return SWAP32(p);
}

static inline int16_t
SWAP(int16_t p)
{
	return (int16_t)SWAP16((uint16_t)p);
}

static inline int32_t
SWAP(int32_t p)
{
	return (int32_t)SWAP32((uint32_t)p);
}
#endif

#endif
