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
#define PDF_FONT_FONTTYPE_CIDTYPE0 5
#define PDF_FONT_FONTTYPE_CIDTYPE2 6

void pdf_font_set_dpi (int font_dpi);

#define PDF_FONT_FLAG_NOEMBED   (1 << 0)
#define PDF_FONT_FLAG_COMPOSITE (1 << 1)
#define PDF_FONT_FLAG_BASEFONT  (1 << 2)
#define PDF_FONT_FLAG_USEDCHAR_SHARED  (1 << 3)
#define PDF_FONT_FLAG_IS_ALIAS  (1 << 4)
#define PDF_FONT_FLAG_IS_REENCODE  (1 << 5)
#define PDF_FONT_FLAG_ACCFONT  (1 << 6)
#define PDF_FONT_FLAG_UCSFONT  (1 << 7)

#define CIDFONT_FLAG_TYPE1    (1 << 8)
#define CIDFONT_FLAG_TYPE1C   (1 << 9)
#define CIDFONT_FLAG_TRUETYPE (1 << 10)

#define PDF_FONT_PARAM_DESIGN_SIZE 1
#define PDF_FONT_PARAM_POINT_SIZE  2

#include "dpx-fontmap.h"

#define FONT_STYLE_NONE       FONTMAP_STYLE_NONE
#define FONT_STYLE_BOLD       FONTMAP_STYLE_BOLD
#define FONT_STYLE_ITALIC     FONTMAP_STYLE_ITALIC
#define FONT_STYLE_BOLDITALIC FONTMAP_STYLE_BOLDITALIC

typedef struct {
  char *registry;
  char *ordering;
  int   supplement;
} CIDSysInfo;

typedef struct
{
  CIDSysInfo csi;
  int        style;
  int        embed;
  int        stemv;
} cid_opt;

struct pdf_font
{
  char    *ident;   /* map name */
  int      font_id; /* ID of this font */
  int      subtype;

  char    *filename;

  int      encoding_id; /* encoding or CMap */

  uint32_t index;
  char    *fontname;
  char     uniqueID[7];

  /*
   * PDF font resource objects
   */
  pdf_obj *reference;
  pdf_obj *resource;
  pdf_obj *descriptor;

  /*
   * Font format specific data
   */
  char    *usedchars;
  int      flags;

  /* PK font */
  double   point_size;
  double   design_size;

  /* Type0 font */
  struct {
    int  descendant; /* Only single descendant is allowed. */
    int  wmode;
  } type0;

  /* CIDFont */
  struct {
    CIDSysInfo csi;     /* Character collection */
    cid_opt    options; /* Options from map record */
    int        need_vmetrics;
    char      *usedchars_v;
  } cid;
};


typedef struct pdf_font pdf_font;

/* pdf_open_document() call them. */
void     pdf_init_fonts  (void);
void     pdf_close_fonts (void);

/* tex_name is used when mrec is NULL.
 * font_scale (point size) used by PK font.
 * It might be necessary if dvipdfmx supports font format with
 * various optical sizes supported in the future.
 */
extern int      pdf_font_findresource  (const char *tex_name, double font_scale);
extern int      pdf_font_load_font     (const char *tex_name, double font_scale, const fontmap_rec *mrec);

extern pdf_font *pdf_get_font_data      (int font_id);

extern char     *pdf_get_font_ident     (int font_id);
extern int       pdf_get_font_subtype   (int font_id);
extern pdf_obj  *pdf_get_font_reference (int font_id);
extern pdf_obj  *pdf_get_font_resource  (int font_id);
extern char     *pdf_get_font_usedchars (int font_id);

extern int       pdf_get_font_encoding  (int font_id);
extern int       pdf_get_font_wmode     (int font_id);

extern int       pdf_font_resource_name (int font_id, char *buf);

extern char     *pdf_font_get_uniqueTag  (pdf_font *font);

extern pdf_obj  *pdf_font_get_resource   (pdf_font *font);
extern pdf_obj  *pdf_font_get_descriptor (pdf_font *font);

extern void      pdf_font_make_uniqueTag (char *tag);

#define add_to_used_chars2(b,c) {(b)[(c)/8] |= (1 << (7-((c)%8)));}
#define is_used_char2(b,c) (((b)[(c)/8]) & (1 << (7-((c)%8))))

extern int pdf_check_tfm_widths (const char *ident, double *widths, int firstchar, int lastchar, const char *usedchars);

/* Tectonic: */
void pdf_font_reset_unique_tag_state(void);
void pdf_font_set_deterministic_unique_tags(int value);

#endif /* _PDFFONT_H_ */
