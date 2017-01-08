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

#ifndef _MPOST_H_
#define _MPOST_H_

#include  "mfileio.h"
#include  "pdfximage.h"
#include  "pdfdev.h"

extern int  mps_scan_bbox    (const char **pp, const char *endptr, pdf_rect *bbox);

/* returns xobj_id */
extern int  mps_include_page (const char *ident, FILE *fp);

extern int  mps_exec_inline  (const char **buffer, const char *endptr,
			      double x_user, double y_user);
extern int  mps_stack_depth  (void);

extern void mps_eop_cleanup  (void);

extern int  mps_do_page      (FILE *fp);

#endif /* _MPOST_H_ */
