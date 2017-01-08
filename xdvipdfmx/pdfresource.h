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

#ifndef _PDF_RESOURCE_H_
#define _PDF_RESOURCE_H_

#include "pdfobj.h"

#define PDF_RES_FLUSH_IMMEDIATE 1

extern void     pdf_init_resources  (void);
extern void     pdf_close_resources (void);

extern int      pdf_defineresource (const char *category,
				    const char *resname,  pdf_obj *object, int flags);
extern int      pdf_findresource   (const char *category, const char *resname);
#if 0
extern int      pdf_resource_exist (const char *category, const char *resname);
#endif

extern pdf_obj *pdf_get_resource_reference (int res_id);
#if 0
extern pdf_obj *pdf_get_resource           (int res_id);
#endif

#endif /* _PDF_RESOURCE_H_ */
