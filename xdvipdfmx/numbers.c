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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"	
#include "error.h"
#include "mfileio.h"
#include "numbers.h"

unsigned char get_unsigned_byte (FILE *file)
{
  int ch;
  if ((ch = fgetc (file)) < 0)
    ERROR ("File ended prematurely\n");
  return (unsigned char) ch;
}

void skip_bytes (unsigned int n, FILE *file)
{
  while (n-- > 0)
    get_unsigned_byte(file);
}

signed char get_signed_byte (FILE *file)
{
  int byte;
  byte = get_unsigned_byte(file);
  if (byte >= 0x80) 
    byte -= 0x100;
  return (signed char) byte;
}

unsigned short get_unsigned_pair (FILE *file)
{
  unsigned short pair = get_unsigned_byte(file);
  pair = (pair << 8) | get_unsigned_byte(file);
  return pair;
}

unsigned short sget_unsigned_pair (unsigned char *s)
{
  unsigned short pair = *(s++);
  pair = (pair << 8) | *(s++);
  return pair;
}

signed short get_signed_pair (FILE *file)
{
  signed short pair = get_signed_byte(file);
  pair = (pair << 8) | get_unsigned_byte(file);
  return pair;
}


unsigned int get_unsigned_triple(FILE *file)
{
  int i;
  unsigned int triple = 0;
  for (i=0; i<3; i++) {
    triple = (triple << 8) | get_unsigned_byte(file);
  }
  return triple;
}

signed int get_signed_triple(FILE *file)
{
  int i;
  signed int triple = get_signed_byte(file);
  for (i=0; i<2; i++) {
    triple = (triple << 8) | get_unsigned_byte(file);
  }
  return triple;
}

int32_t get_signed_quad(FILE *file)
{
  int i;
  int32_t quad = get_signed_byte(file);
  for (i=0; i<3; i++) {
    quad = (quad << 8) | get_unsigned_byte(file);
  }
  return quad;
}

uint32_t get_unsigned_quad(FILE *file)
{
  int i;
  uint32_t quad = 0;
  for (i=0; i<4; i++) {
    quad = (quad << 8) | get_unsigned_byte(file);
  }
  return quad;
}

int32_t get_unsigned_num (FILE *file, unsigned char num)
{
  int32_t val = get_unsigned_byte (file);
  switch (num) {
  case 3: if (val > 0x7f)
            val -= 0x100;
          val = (val << 8) | get_unsigned_byte (file);
  case 2: val = (val << 8) | get_unsigned_byte (file);
  case 1: val = (val << 8) | get_unsigned_byte (file);
  default: break;
  }
  return val;
}

/* Compute a signed quad that must be positive */
uint32_t get_positive_quad (FILE *file, const char *type, const char *name)
{
  int32_t val = get_signed_quad (file);
  if (val < 0)
    ERROR ("Bad %s: negative %s: %d", type, name, val);
  return (uint32_t)val;
}

int32_t sqxfw (int32_t sq, fixword fw)
{
  int sign = 1;
  uint32_t a, b, c, d, ad, bd, bc, ac;
  uint32_t e, f, g, h, i, j, k;
  int32_t result;
  /* Make positive. */
  if (sq < 0) {
    sign = -sign;
    sq = -sq;
  }
  if (fw < 0) {
    sign = -sign;
    fw = -fw;
  }
  a = ((uint32_t) sq) >> 16;
  b = ((uint32_t) sq) & 0xffffu;
  c = ((uint32_t) fw) >> 16;
  d = ((uint32_t) fw) & 0xffffu;
  ad = a*d; bd = b*d; bc = b*c; ac = a*c;
  e = bd >> 16;
  f = ad >> 16;
  g = ad & 0xffffu;
  h = bc >> 16;
  i = bc & 0xffffu;
  j = ac >> 16;
  k = ac & 0xffffu;
  result = (e+g+i + (1<<3)) >> 4;  /* 1<<3 is for rounding */
  result += (f+h+k) << 12;
  result += j << 28;
  return (sign > 0) ? result : -result;
}

