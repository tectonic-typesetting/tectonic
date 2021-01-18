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

#ifndef _TT_GLYF_H_
#define _TT_GLYF_H_

#include "tectonic_bridge_core.h"

#include "dpx-sfnt.h"

struct tt_glyph_desc
{
  USHORT gid;
  USHORT ogid; /* GID in original font */
  USHORT advw, advh;
  SHORT  lsb, tsb;
  SHORT  llx, lly, urx, ury;
  ULONG  length;
  BYTE  *data;
};

struct tt_glyphs
{
  USHORT num_glyphs;
  USHORT max_glyphs;
  USHORT last_gid;
  USHORT emsize;
  USHORT dw;           /* optimal value for DW */
  USHORT default_advh; /* default value */
  SHORT  default_tsb;  /* default value */
  struct tt_glyph_desc *gd;
  unsigned char *used_slot;
};

struct tt_glyphs *tt_build_init (void);
void   tt_build_finish (struct tt_glyphs *g);

USHORT tt_add_glyph  (struct tt_glyphs *g, USHORT gid, USHORT new_gid);
USHORT tt_get_index  (struct tt_glyphs *g, USHORT gid);
USHORT tt_find_glyph (struct tt_glyphs *g, USHORT gid);

int    tt_build_tables (sfnt *sfont, struct tt_glyphs *g);
int    tt_get_metrics  (sfnt *sfont, struct tt_glyphs *g);

#endif /* _TT_GLYF_H_ */
