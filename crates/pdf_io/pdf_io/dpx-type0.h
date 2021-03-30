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

#ifndef _TYPE0_H_
#define _TYPE0_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdfobj.h"

#define add_to_used_chars2(b,c) {(b)[(c)/8] |= (1 << (7-((c)%8)));}
#define is_used_char2(b,c) (((b)[(c)/8]) & (1 << (7-((c)%8))))

typedef struct Type0Font Type0Font;

int        Type0Font_get_wmode     (Type0Font *font);
char      *Type0Font_get_usedchars (Type0Font *font);

pdf_obj   *Type0Font_get_resource  (Type0Font *font);

void       Type0Font_set_ToUnicode (Type0Font *font, pdf_obj *cmap_ref);

#include "dpx-fontmap.h"

/******************************** CACHE ********************************/

void       Type0Font_cache_init  (void);
Type0Font *Type0Font_cache_get   (int id);
int        Type0Font_cache_find  (const char *map_name, int cmap_id, fontmap_opt *fmap_opt);
void       Type0Font_cache_close (void);

#endif /* _TYPE0_H_ */
