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

#ifndef _PDFFONT_H_
#define _PDFFONT_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>

#include "dpx-fontmap.h"
#include "dpx-pdflimits.h"
#include "dpx-pdfobj.h"

#define PDF_FONT_FONTTYPE_TYPE1    0
#define PDF_FONT_FONTTYPE_TYPE1C   1
#define PDF_FONT_FONTTYPE_TYPE3    2
#define PDF_FONT_FONTTYPE_TRUETYPE 3

#define PDF_FONT_FONTTYPE_TYPE0    4

void pdf_font_set_dpi (int font_dpi);

#define PDF_FONT_FLAG_NOEMBED   (1 << 0)
#define PDF_FONT_FLAG_COMPOSITE (1 << 1)
#define PDF_FONT_FLAG_BASEFONT  (1 << 2)

#define PDF_FONT_PARAM_DESIGN_SIZE 1
#define PDF_FONT_PARAM_POINT_SIZE  2

typedef struct pdf_font pdf_font;

/* pdf_open_document() call them. */
void     pdf_init_fonts  (void);
void     pdf_close_fonts (void);

/* font_name is used when mrec is NULL.
 * font_scale (point size) used by PK font.
 * It might be necessary if dvipdfmx supports font format with
 * various optical sizes supported in the future.
 */
int      pdf_font_findresource  (const char *font_name,
                                        double font_scale, fontmap_rec *mrec);

int      pdf_get_font_subtype   (int font_id);
pdf_obj *pdf_get_font_reference (int font_id);
char    *pdf_get_font_usedchars (int font_id);

int      pdf_get_font_encoding  (int font_id);
int      pdf_get_font_wmode     (int font_id);

/* Each font drivers use the followings. */
bool     pdf_font_is_in_use      (pdf_font *font);

char    *pdf_font_get_ident      (pdf_font *font);
char    *pdf_font_get_mapname    (pdf_font *font);
char    *pdf_font_get_fontname   (pdf_font *font); /* without unique tag */
char    *pdf_font_get_uniqueTag  (pdf_font *font);

void pdf_font_reset_unique_tag_state(void);
void pdf_font_set_deterministic_unique_tags(int value);

pdf_obj *pdf_font_get_resource   (pdf_font *font);
pdf_obj *pdf_font_get_descriptor (pdf_font *font);

char    *pdf_font_get_usedchars  (pdf_font *font);
int      pdf_font_get_encoding   (pdf_font *font);

int      pdf_font_get_flag       (pdf_font *font, int mask);
double   pdf_font_get_param      (pdf_font *font, int type);

int      pdf_font_get_index      (pdf_font *font);

int      pdf_font_set_fontname   (pdf_font *font, const char *fontname);
int      pdf_font_set_flags      (pdf_font *font, int flags);
int      pdf_font_set_subtype    (pdf_font *font, int subtype);

void     pdf_font_make_uniqueTag (char *tag);

#endif /* _PDFFONT_H_ */
