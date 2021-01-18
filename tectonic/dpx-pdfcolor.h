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

#ifndef _PDF_COLOR_H_
#define _PDF_COLOR_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>

#include "dpx-pdfobj.h"

#define PDF_COLORSPACE_TYPE_DEVICECMYK -4
#define PDF_COLORSPACE_TYPE_DEVICERGB  -3
#define PDF_COLORSPACE_TYPE_SPOT       -2
#define PDF_COLORSPACE_TYPE_DEVICEGRAY -1
#define PDF_COLORSPACE_TYPE_INVALID     0
#define PDF_COLORSPACE_TYPE_CALGRAY     1
#define PDF_COLORSPACE_TYPE_CIELAB      2
#define PDF_COLORSPACE_TYPE_CALRGB      3
#define PDF_COLORSPACE_TYPE_ICCBASED    4

#define PDF_COLORSPACE_TYPE_CMYK  PDF_COLORSPACE_TYPE_DEVICECMYK
#define PDF_COLORSPACE_TYPE_RGB   PDF_COLORSPACE_TYPE_DEVICERGB
#define PDF_COLORSPACE_TYPE_GRAY  PDF_COLORSPACE_TYPE_DEVICEGRAY


#define PDF_COLOR_COMPONENT_MAX 4

typedef struct
{
  int    num_components;
  char*  spot_color_name;
  double values[PDF_COLOR_COMPONENT_MAX];
} pdf_color;

int        pdf_color_rgbcolor      (pdf_color *color,
                                           double r, double g, double b);
int        pdf_color_cmykcolor     (pdf_color *color,
                                           double c, double m, double y, double k);
int        pdf_color_graycolor     (pdf_color *color, double g);

int        pdf_color_spotcolor     (pdf_color *color, char* color_name, double c);

void       pdf_color_copycolor     (pdf_color *color1, const pdf_color *color2);

#define pdf_color_black(c)   pdf_color_graycolor(c, 0.0);
#define pdf_color_white(c)   pdf_color_graycolor(c, 1.0);

void       pdf_color_brighten_color (pdf_color *dst, const pdf_color *src, double f);

int        pdf_color_type          (const pdf_color *color);
int        pdf_color_compare       (const pdf_color *color1, const pdf_color *color2);
int        pdf_color_to_string     (const pdf_color *color, char *buffer, char mask);

bool       pdf_color_is_white      (const pdf_color *color);
bool       pdf_color_is_valid      (const pdf_color *color);

/* Not check size */
pdf_obj *iccp_get_rendering_intent (const void *profile, int proflen);
int      iccp_check_colorspace     (int colortype,
                                           const void *profile, int proflen);

/* returns colorspace ID */
int      iccp_load_profile (const char *ident,
                                   const void *profile, int proflen);

void     pdf_init_colors  (void);
void     pdf_close_colors (void);

pdf_obj *pdf_get_colorspace_reference      (int cspc_id);

/* Color special
 * See remark in spc_color.c.
 */
void     pdf_color_set   (pdf_color *sc, pdf_color *fc);
void     pdf_color_push  (pdf_color *sc, pdf_color *fc);
void     pdf_color_pop   (void);

/* Color stack
 */
void     pdf_color_clear_stack (void);
void     pdf_color_get_current (pdf_color **sc, pdf_color **fc);

#endif /* _PDF_COLOR_H_ */
