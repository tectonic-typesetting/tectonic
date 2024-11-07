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

#ifndef _CID_H_
#define _CID_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>

#include "dpx-pdffont.h"

extern CIDSysInfo CSI_IDENTITY;
extern CIDSysInfo CSI_UNICODE;

extern int opt_flags_cidfont;
extern void CIDFont_set_flags (int flags);

#define CIDFONT_FORCE_FIXEDPITCH (1 << 1)

extern char *CIDFont_get_usedchars   (pdf_font *font);
extern char *CIDFont_get_usedchars_v (pdf_font *font);

extern int   CIDFont_is_ACCFont  (pdf_font *font);
extern int   CIDFont_is_UCSFont  (pdf_font *font);

#include "dpx-fontmap.h"
extern int   pdf_font_cidfont_lookup_cache (pdf_font *fonts, int count, const char *map_name,
                                            CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt);
extern int   pdf_font_open_cidfont (pdf_font *font, const char *map_name,
                                    CIDSysInfo *cmap_csi, const fontmap_opt *fmap_opt);
extern void  pdf_font_load_cidfont (pdf_font *font);

#endif /* _CID_H_ */
