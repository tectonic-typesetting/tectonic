/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _CIDTYPE2_H_
#define _CIDTYPE2_H_

#include "tectonic_bridge_core.h"

#include "dpx-cid.h"
#include "dpx-cid_p.h"

void CIDFont_type2_set_flags   (int flags);

int  CIDFont_type2_open    (CIDFont *font, const char *name,
                                   CIDSysInfo *cmap_csi, cid_opt *opt);
void CIDFont_type2_dofont  (CIDFont *font);

#endif /* _CIDTYPE2_H_ */
