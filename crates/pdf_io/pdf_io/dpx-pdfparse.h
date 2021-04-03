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

#ifndef _PDFPARSE_H_
#define _PDFPARSE_H_

#include "tectonic_bridge_core.h"

#include "dpx-numbers.h"
#include "dpx-pdfobj.h"

/* Please remove this */
void dump (const char *start, const char *end);

void pdfparse_skip_line (const char **start, const char *end);
void skip_white (const char **start, const char *end);

char *parse_number   (const char **start, const char *end);
char *parse_unsigned (const char **start, const char *end);

char *parse_ident     (const char **start, const char *end);
char *parse_val_ident (const char **start, const char *end);
char *parse_opt_ident (const char **start, const char *end);

pdf_obj *parse_pdf_name    (const char **pp, const char *endptr);
pdf_obj *parse_pdf_boolean (const char **pp, const char *endptr);
pdf_obj *parse_pdf_number  (const char **pp, const char *endptr);
pdf_obj *parse_pdf_null    (const char **pp, const char *endptr);
pdf_obj *parse_pdf_string  (const char **pp, const char *endptr);
pdf_obj *parse_pdf_dict    (const char **pp, const char *endptr, pdf_file *pf);
pdf_obj *parse_pdf_array   (const char **pp, const char *endptr, pdf_file *pf);
pdf_obj *parse_pdf_object  (const char **pp, const char *endptr, pdf_file *pf);

pdf_obj *parse_pdf_tainted_dict (const char **pp, const char *endptr);

#endif /* _PDFPARSE_H_ */
