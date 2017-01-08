/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/

/*
 * Unicode related:
 *  Conversion between UTF-* and UCS-*.
 *  ToUnicode CMap
 *
 * Normalization?
 *
 * I made some unused functions here for completeness and FUTURE USE.
 */

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "unicode.h"

#define UC_DEBUG     3
#define UC_DEBUG_STR "UC"

#define UC_REPLACEMENT_CHAR 0x0000FFFD

#define UC_SUR_SHIFT      10
#define UC_SUR_MASK       0x03FFU
#define UC_SUR_LOW_START  0xDC00U
#define UC_SUR_HIGH_START 0xD800U
#define UC_SUR_END        0xE000U

int
UC_is_valid (int32_t ucv)
{
  if ( ucv < 0 || ucv > 0x10FFFFL ||
      (ucv >= 0x0000D800L && ucv <= 0x0000DFFFL))
    return 0;
  return 1;
}

int
UC_UTF16BE_is_valid_string (const unsigned char *p, const unsigned char *endptr)
{
  if (p + 1 >= endptr)
   return 0;
  while (p < endptr) {
    int32_t ucv = UC_UTF16BE_decode_char(&p, endptr);
    if (!UC_is_valid(ucv))
      return 0;
  }
  return 1;
}

int
UC_UTF8_is_valid_string (const unsigned char *p, const unsigned char *endptr)
{
  if (p + 1 >= endptr)
   return 0;
  while (p < endptr) {
    int32_t ucv = UC_UTF8_decode_char(&p, endptr);
    if (!UC_is_valid(ucv))
      return 0;
  }
  return 1;
}

int32_t
UC_UTF16BE_decode_char (const unsigned char **pp, const unsigned char *endptr)
{
  const unsigned char *p = *pp;
  int32_t  ucv = -1;
  uint16_t first, second;

  if (p + 1 >= endptr)
    return -1;

  first = ((p[0]) << 8|p[1]); p += 2;
  if (first >= UC_SUR_HIGH_START && first < UC_SUR_LOW_START) {
    if (p + 1 >= endptr)
      return -1;
    second = (p[0] << 8|p[1]); p += 2;
    ucv    =  second & UC_SUR_MASK;
    ucv   |= (first  & UC_SUR_MASK) << UC_SUR_SHIFT;
    ucv   += 0x00010000;
  } else if (first >= UC_SUR_LOW_START && first < UC_SUR_END) {
    return -1;
  } else {
    ucv = first;
  }

  *pp = p;
  return ucv;
}

size_t
UC_UTF16BE_encode_char (int32_t ucv, unsigned char **pp, unsigned char *endptr)
{
  int count = 0;
  unsigned char *p = *pp;

  if (ucv >= 0 && ucv <= 0xFFFF) {
    if (p + 2 >= endptr)
      return 0;
    p[0] = (ucv >> 8) & 0xff;
    p[1] = ucv & 0xff;
    count = 2;
  } else if (ucv >= 0x010000 && ucv <= 0x10FFFF) {
    unsigned short high, low;

    if (p + 4 >= endptr)
      return 0;
    ucv  -= 0x00010000;
    high = (ucv >> UC_SUR_SHIFT) + UC_SUR_HIGH_START;
    low  = (ucv &  UC_SUR_MASK)  + UC_SUR_LOW_START;
    p[0] = (high >> 8) & 0xff;
    p[1] = (high & 0xff);
    p[2] = (low >> 8) & 0xff;
    p[3] = (low & 0xff);
    count = 4;
  } else {
    if (p + 2 >= endptr)
      return 0;
    p[0] = (UC_REPLACEMENT_CHAR >> 8) & 0xff;
    p[1] = (UC_REPLACEMENT_CHAR & 0xff);
    count = 2;
  }

  *pp += count;
  return count;
}

int32_t
UC_UTF8_decode_char (const unsigned char **pp, const unsigned char *endptr)
{
  const unsigned char *p = *pp;
  int32_t  ucv;
  unsigned char c = *p++;
  int      nbytes;

  if (c <= 0x7f) {
    ucv    = c;
    nbytes = 0;
  } else if ((c & 0xe0) == 0xc0) { /* 110x xxxx */
    ucv    = c & 31;
    nbytes = 1;
  } else if ((c & 0xf0) == 0xe0) { /* 1110 xxxx */
    ucv    = c & 0x0f;
    nbytes = 2;
  } else if ((c & 0xf8) == 0xf0) { /* 1111 0xxx */
    ucv    = c & 0x07;
    nbytes = 3;
  } else if ((c & 0xfc) == 0xf8) { /* 1111 10xx */
    ucv    = c & 0x03;
    nbytes = 4;
  } else if ((c & 0xfe) == 0xfc) { /* 1111 110x */
    ucv    = c & 0x01;
    nbytes = 5;
  } else {
    return -1;
  }
  if (p + nbytes > endptr)
    return -1;
  while (nbytes-- > 0) {
    c = *p++;
    if ((c & 0xc0) != 0x80)
      return -1;
    ucv = (ucv << 6) | (c & 0x3f);
  }

  *pp = p;
  return ucv;
}

size_t
UC_UTF8_encode_char (int32_t ucv, unsigned char **pp, unsigned char *endptr)
{
  int  count = 0;
  unsigned char *p = *pp;

  ASSERT( pp && *pp && endptr );

  if (!UC_is_valid(ucv))
    return 0;

  if (ucv < 0x7f) {
    if (p >= endptr - 1)
      return 0;
    p[0]  = (unsigned char) ucv;
    count = 1;
  } else if (ucv <= 0x7ff) {
    if (p >= endptr -2)
      return 0;
    p[0] = (unsigned char) (0xc0 | (ucv >> 6));
    p[1] = (unsigned char) (0x80 | (ucv & 0x3f));
    count = 2;
  } else if (ucv <= 0xffff) {
    if (p >= endptr - 3)
      return 0;
    p[0] = (unsigned char) (0xe0 | (ucv >> 12));
    p[1] = (unsigned char) (0x80 | ((ucv >> 6) & 0x3f));
    p[2] = (unsigned char) (0x80 | (ucv & 0x3f));
    count = 3;
  } else if (ucv <= 0x1fffff) {
    if (p >= endptr - 4)
      return 0;
    p[0] = (unsigned char) (0xf0 | (ucv >> 18));
    p[1] = (unsigned char) (0x80 | ((ucv >> 12) & 0x3f));
    p[2] = (unsigned char) (0x80 | ((ucv >>  6) & 0x3f));
    p[3] = (unsigned char) (0x80 | (ucv & 0x3f));
    count = 4;
  } else if (ucv <= 0x3ffffff) {
    if (p >= endptr - 5)
      return 0;
    p[0] = (unsigned char) (0xf8 | (ucv >> 24));
    p[1] = (unsigned char) (0x80 | ((ucv >> 18) & 0x3f));
    p[2] = (unsigned char) (0x80 | ((ucv >> 12) & 0x3f));
    p[3] = (unsigned char) (0x80 | ((ucv >>  6) & 0x3f));
    p[4] = (unsigned char) (0x80 | (ucv & 0x3f));
    count = 5;
  } else if (ucv <= 0x7fffffff) {
     if (p >= endptr - 6)
      return 0;
    p[0] = (unsigned char) (0xfc | (ucv >> 30));
    p[1] = (unsigned char) (0x80 | ((ucv >> 24) & 0x3f));
    p[2] = (unsigned char) (0x80 | ((ucv >> 18) & 0x3f));
    p[3] = (unsigned char) (0x80 | ((ucv >> 12) & 0x3f));
    p[4] = (unsigned char) (0x80 | ((ucv >>  6) & 0x3f));
    p[5] = (unsigned char) (0x80 | (ucv & 0x3f));
    count = 6;
  }

  *pp += count;
  return count;
}
