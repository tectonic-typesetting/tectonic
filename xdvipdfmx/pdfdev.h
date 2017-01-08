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

#ifndef _PDFDEV_H_
#define _PDFDEV_H_

#include "numbers.h"
#include "pdfobj.h"
#include "pdfcolor.h"

typedef int spt_t;

typedef struct pdf_tmatrix
{
  double a, b, c, d, e, f;
} pdf_tmatrix;

typedef struct pdf_rect
{
  double llx, lly, urx, ury;
} pdf_rect;

typedef struct pdf_coord
{
  double x, y;
} pdf_coord;

/* The name transform_info is misleading.
 * I'll put this here for a moment...
 */
typedef struct
{
  /* Physical dimensions
   *
   * If those values are given, images will be scaled
   * and/or shifted to fit within a box described by
   * those values.
   */
  double      width;
  double      height;
  double      depth;

  pdf_tmatrix matrix; /* transform matrix */
  pdf_rect    bbox;   /* user_bbox */

  int         flags;
} transform_info;
#define INFO_HAS_USER_BBOX (1 << 0)
#define INFO_HAS_WIDTH     (1 << 1)
#define INFO_HAS_HEIGHT    (1 << 2)
#define INFO_DO_CLIP       (1 << 3)
#define INFO_DO_HIDE       (1 << 4)
extern void   transform_info_clear (transform_info *info);


extern void   pdf_dev_set_verbose (void);

/* Not in spt_t. */
extern int    pdf_sprint_matrix (char *buf, const pdf_tmatrix *p);
extern int    pdf_sprint_rect   (char *buf, const pdf_rect    *p);
extern int    pdf_sprint_coord  (char *buf, const pdf_coord   *p);
extern int    pdf_sprint_length (char *buf, double value);
extern int    pdf_sprint_number (char *buf, double value);

/* unit_conv: multiplier for input unit (spt_t) to bp conversion.
 * precision: How many fractional digits preserved in output (not real
 *            accuracy control).
 * is_bw:     Ignore color related special instructions.
 */
extern void   pdf_init_device   (double unit_conv, int precision, int is_bw);
extern void   pdf_close_device  (void);

/* returns 1.0/unit_conv */
extern double dev_unit_dviunit  (void);

#if 0
/* DVI interpreter knows text positioning in relative motion.
 * However, pdf_dev_set_string() recieves text string with placement
 * in absolute position in user space, and it convert absolute
 * positioning back to relative positioning. It is quite wasteful.
 *
 * TeX using DVI register stack operation to do CR and then use down
 * command for LF. DVI interpreter knows hint for current leading
 * and others (raised or lowered), but they are mostly lost in
 * pdf_dev_set_string().
 */

typedef struct
{
  int      argc;

  struct {
    int    is_kern; /* kern or string */

    spt_t  kern;    /* negative kern means space */

    int    offset;  /* offset to sbuf   */
    int    length;  /* length of string */
  } args[];

  unsigned char sbuf[PDF_STRING_LEN_MAX];

} pdf_text_string;

/* Something for handling raise, leading, etc. here. */

#endif

/* Draw texts and rules:
 *
 * xpos, ypos, width, and height are all fixed-point numbers
 * converted to big-points by multiplying unit_conv (dvi2pts).
 * They must be position in the user space.
 *
 * ctype:
 *   0 - input string is in multi-byte encoding.
 *   1 - input string is in 8-bit encoding.
 *   2 - input string is in 16-bit encoding.
 */
extern void   pdf_dev_set_string (spt_t xpos, spt_t ypos,
				  const void *instr_ptr, int instr_len,
				  spt_t text_width,
				  int   font_id, int ctype);
extern void   pdf_dev_set_rule   (spt_t xpos, spt_t ypos,
				  spt_t width, spt_t height);

/* Place XObject */
extern int    pdf_dev_put_image  (int xobj_id,
				  transform_info *p, double ref_x, double ref_y);

/* The design_size and ptsize required by PK font support...
 */
extern int    pdf_dev_locate_font (const char *font_name, spt_t ptsize);

/* The following two routines are NOT WORKING.
 * Dvipdfmx doesn't manage gstate well..
 */
#if 0
/* pdf_dev_translate() or pdf_dev_concat() should be used. */
extern void   pdf_dev_set_origin (double orig_x, double orig_y);
#endif
/* Always returns 1.0, please rename this. */
extern double pdf_dev_scale      (void);

/* Access text state parameters. */
#if 0
extern int    pdf_dev_currentfont     (void); /* returns font_id */
extern double pdf_dev_get_font_ptsize (int font_id);
#endif
extern int    pdf_dev_get_font_wmode  (int font_id); /* ps: special support want this (pTeX). */

/* Text composition (direction) mode
 * This affects only when auto_rotate is enabled.
 */
extern int    pdf_dev_get_dirmode     (void);
extern void   pdf_dev_set_dirmode     (int dir_mode);

/* Set rect to rectangle in device space.
 * Unit conversion spt_t to bp and transformation applied within it.
 */
extern void   pdf_dev_set_rect   (pdf_rect *rect,
				  spt_t x_pos, spt_t y_pos,
				  spt_t width, spt_t height, spt_t depth);

/* Accessor to various device parameters.
 */
#define PDF_DEV_PARAM_AUTOROTATE  1
#define PDF_DEV_PARAM_COLORMODE   2

extern int    pdf_dev_get_param (int param_type);
extern void   pdf_dev_set_param (int param_type, int value);

/* Text composition mode is ignored (always same as font's
 * writing mode) and glyph rotation is not enabled if
 * auto_rotate is unset.
 */
#define pdf_dev_set_autorotate(v) pdf_dev_set_param(PDF_DEV_PARAM_AUTOROTATE, (v))

/*
 * For pdf_doc, pdf_draw and others.
 */

/* Force reselecting font and color:
 * XFrom (content grabbing) and Metapost support want them.
 */
extern void   pdf_dev_reset_fonts (int newpage);
extern void   pdf_dev_reset_color (int force);

/* Initialization of transformation matrix with M and others.
 * They are called within pdf_doc_begin_page() and pdf_doc_end_page().
 */
extern void   pdf_dev_bop (const pdf_tmatrix *M);
extern void   pdf_dev_eop (void);

/* Text is normal and line art is not normal in dvipdfmx. So we don't have
 * begin_text (BT in PDF) and end_text (ET), but instead we have graphics_mode()
 * to terminate text section. pdf_dev_flushpath() and others call this.
 */
extern void   graphics_mode (void);

extern void   pdf_dev_get_coord(double *xpos, double *ypos);
extern void   pdf_dev_push_coord(double xpos, double ypos);
extern void   pdf_dev_pop_coord(void);

extern void   pdf_dev_begin_actualtext (uint16_t *unicodes, int len);
extern void   pdf_dev_end_actualtext ();

#endif /* _PDFDEV_H_ */
