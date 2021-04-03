/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2018 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _CID_P_H_
#define _CID_P_H_

#include "tectonic_bridge_core.h"

#define FONT_FLAG_NONE        0
#define FONT_FLAG_BASEFONT    (1 << 0)
#define FONT_FLAG_ACCFONT     (1 << 1)
#define FONT_FLAG_UCSFONT     (1 << 2)

#include "dpx-fontmap.h"
#define FONT_STYLE_NONE       FONTMAP_STYLE_NONE
#define FONT_STYLE_BOLD       FONTMAP_STYLE_BOLD
#define FONT_STYLE_ITALIC     FONTMAP_STYLE_ITALIC
#define FONT_STYLE_BOLDITALIC FONTMAP_STYLE_BOLDITALIC

typedef struct
{
  char       *name;  /* Unused */
  CIDSysInfo *csi;
  int         index;
  int         style;
  int         embed;
  int         stemv;
} cid_opt;

struct CIDFont
{
  char       *ident;      /* Map record entry */
  char       *name;       /* Fontname or filename */
  char       *fontname;   /* PostScript font name */
  /*
   * CIDFont Specific
   */
  int         subtype;    /* CIDFONT_TYPE0 or CIDFONT_TYPE2 */
  int         flags;      /* BASEFONT */
  int         parent[2];  /* Parent type0 font of this CID-keyed font: H, V */
  CIDSysInfo *csi;        /* Character collection */
  cid_opt    *options;    /* Options from map record */
  /*
   * PDF Font Resource
   */
  pdf_obj *indirect;   /* Indirect reference to CIDFont dictionary */
  pdf_obj *fontdict;   /* CIDFont dictionary */
  pdf_obj *descriptor; /* FontDescriptor */
};

#endif /* _CID_P_H_ */
