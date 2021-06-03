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

#ifndef _SPC_UTIL_H_
#define _SPC_UTIL_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"

#include "dpx-specials.h"

/* syntax 1: ((rgb|cmyk|hsb|gray) colorvalues)|colorname
 * syntax 0: pdf_number|pdf_array
 *
 * This is for reading *single* color specification.
 */
int  spc_util_read_colorspec (struct spc_env *spe,
                                     pdf_color *colorspec,
                                     struct spc_arg *args, int syntax);
int  spc_util_read_dimtrns   (struct spc_env *spe,
                                     transform_info *dimtrns,
                                     struct spc_arg *args, int syntax);

int  spc_util_read_blahblah  (struct spc_env *spe,
                                     transform_info *dimtrns,
                                     int            *page_no,
                                     enum pdf_page_boundary *bbox_type,
                                     struct spc_arg *args);


int  spc_util_read_numbers   (double *values, int num_values,
                                     struct spc_arg *args);
int  spc_util_read_pdfcolor  (struct spc_env *spe,
                                     pdf_color *colorspec,
                                     struct spc_arg *args,
                                     pdf_color *defaultcolor);

#endif /* _SPC_UTIL_H_ */
