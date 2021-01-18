/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _PDF_NAMES_H_
#define _PDF_NAMES_H_

#include "tectonic_bridge_core.h"

/* Hash */
#include "dpx-dpxutil.h"
#include "dpx-pdfobj.h"

/* Not actually tree... */
struct ht_table *pdf_new_name_tree    (void);
void             pdf_delete_name_tree (struct ht_table **names);

int      pdf_names_add_object       (struct ht_table *names,
                                            const void *key, int keylen, pdf_obj *object);
pdf_obj *pdf_names_lookup_reference (struct ht_table *names,
                                            const void *key, int keylen);
pdf_obj *pdf_names_lookup_object    (struct ht_table *names,
                                            const void *key, int keylen);
int      pdf_names_close_object     (struct ht_table *names,
                                            const void *key, int keylen);

/* Really create name tree... */
pdf_obj *pdf_names_create_tree      (struct ht_table *names,
                                            int *count,
                                            struct ht_table *filter);

#endif /*  _PDF_NAMES_H_ */
