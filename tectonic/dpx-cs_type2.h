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

#ifndef _CS_TYPE2_H_
#define _CS_TYPE2_H_

#include "tectonic_bridge_core.h"

#include "dpx-cff_types.h"

typedef struct {
  int flags; /* unused in Type 2 charstring */
  double wx, wy;
  struct {
    double llx, lly, urx, ury;
  } bbox;
  struct {
    double asb, adx, ady;
    card8  bchar, achar;
  } seac;   /* unused in Type 2 charstring */
} cs_ginfo;

int cs_copy_charstring (card8 *dest, int destlen,
                               card8 *src, int srclen,
                               cff_index *gsubr, cff_index *subr,
                               double default_width, double nominal_width, cs_ginfo *ginfo);

#endif /* _CS_TYPE2_H_ */
