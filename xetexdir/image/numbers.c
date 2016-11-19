/*  $Header: /home/cvsroot/dvipdfmx/src/numbers.c,v 1.8 2004/03/03 13:19:00 hirata Exp $

    This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2015 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team <dvipdfmx@project.ktug.or.kr>
    
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

#if HAVE_CONFIG_H
#include <w2c/config.h>
#endif

#include "mfileio.h"
#include "numbers.h"

unsigned char get_unsigned_byte (FILE *file)
{
  int ch;
  if ((ch = fgetc (file)) < 0) {
    fprintf (stderr, "File ended prematurely\n");
    exit(-1);
  }
  return (unsigned char) ch;
}

unsigned short get_unsigned_pair (FILE *file)
{
  unsigned short pair = get_unsigned_byte(file);
  pair = pair*0x100u + get_unsigned_byte(file);
  return pair;
}
