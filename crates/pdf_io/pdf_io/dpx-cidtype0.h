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

#ifndef _CIDTYPE0_H_
#define _CIDTYPE0_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdffont.h"

extern int  CIDFont_type0_open    (pdf_font *font, const char *name,
                                   int index, cid_opt *opt);
extern int  CIDFont_type0_dofont  (pdf_font *font);

/* Type1 --> CFF CIDFont */
extern int  t1_load_UnicodeCMap  (const char *font_name, const char *otl_tags, int wmode);
extern int  CIDFont_type0_open_from_t1  (pdf_font *font, const char *name, int index, cid_opt *opt);
extern int  CIDFont_type0_open_from_t1c (pdf_font *font, const char *name, int index, cid_opt *opt);
extern int  CIDFont_type0_t1dofont  (pdf_font *font);
extern int  CIDFont_type0_t1cdofont (pdf_font *font);

extern pdf_obj *CIDFont_type0_t1create_ToUnicode_stream (const char *filename, const char *fontname, const char *used_chars);

#endif /* _CIDTYPE0_H_ */
