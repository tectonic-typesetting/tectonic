/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.
    
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

#ifndef _TT_POST_H_
#define _TT_POST_H_

#include "sfnt.h"

struct tt_post_table
{
  Fixed    Version;
  Fixed    italicAngle;
  FWord    underlinePosition;
  FWord    underlineThickness;
  ULONG    isFixedPitch;
  ULONG    minMemType42;
  ULONG    maxMemType42;
  ULONG    minMemType1;
  ULONG    maxMemType1; 

  USHORT   numberOfGlyphs;

  const char **glyphNamePtr; /* Glyph names (pointer to C string) */
  char   **names;        /* Non-standard glyph names */

  USHORT   count;        /* Number of glyph names in names[] */
};

extern struct tt_post_table  *tt_read_post_table (sfnt *sfont);
extern void   tt_release_post_table (struct tt_post_table *post);

extern USHORT tt_lookup_post_table  (struct tt_post_table *post, const char *glyphname);
extern char*  tt_get_glyphname      (struct tt_post_table *post, USHORT gid);

#endif /* _TT_POST_H_ */
