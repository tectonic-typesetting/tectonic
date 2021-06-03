/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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

#ifndef _OTL_CONF_H_
#define _OTL_CONF_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdfobj.h"

void     otl_conf_set_verbose (int level);

pdf_obj *otl_find_conf  (const char *conf_name);
void     otl_init_conf  (void);
void     otl_close_conf (void);

pdf_obj *otl_conf_get_class (pdf_obj *conf, const char *ident);

char *otl_conf_get_script   (pdf_obj *conf);
char *otl_conf_get_language (pdf_obj *conf);
pdf_obj *otl_conf_get_rule  (pdf_obj *conf);
pdf_obj *otl_conf_find_opt  (pdf_obj *conf, const char *opt_tag);

#endif /* _OTL_CONF_H_ */
