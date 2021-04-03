/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _CFF_DICT_H_
#define _CFF_DICT_H_

#include "tectonic_bridge_core.h"

#include "dpx-cff_types.h"
#include "dpx-cff.h"

#define CFF_NOMINALWIDTHX_DEFAULT 0.0
#define CFF_DEFAULTWIDTHX_DEFAULT 0.0

cff_dict *cff_new_dict (void);
void      cff_release_dict (cff_dict *dict);

void   cff_dict_set (cff_dict *dict, const char *key, int idx, double value);
double cff_dict_get (cff_dict *dict, const char *key, int idx);
void   cff_dict_add (cff_dict *dict, const char *key, int count);
void   cff_dict_remove (cff_dict *dict, const char *key);
int    cff_dict_known  (cff_dict *dict, const char *key);

/* decode/encode DICT */
cff_dict *cff_dict_unpack (card8 *data, card8 *endptr);
int       cff_dict_pack (cff_dict *dict, card8 *dest, int destlen);

void      cff_dict_update (cff_dict *dict, cff_font *cff);

#endif /* _CFF_DICT_H_ */
