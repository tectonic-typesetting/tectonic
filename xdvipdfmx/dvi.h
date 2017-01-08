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

#ifndef _DVI_H_
#define _DVI_H_	

#include "error.h"
#include "numbers.h"
/* spt_t */
#include "pdfdev.h"

/* instantiated in dvipdfmx.c */
extern double paper_width, paper_height;
extern int    landscape_mode;

extern double get_origin (int x);

extern void  dvi_set_verbose (void);

/* returns scale (dvi2pts) */
extern double dvi_init  (char *dvi_filename, double mag); /* may append .dvi or .xdv to filename */
extern void   dvi_close (void);  /* Closes data structures created by dvi_open */

extern double       dvi_tell_mag  (void);
extern double       dvi_unit_size (void);
extern double       dvi_dev_xpos  (void);
extern double       dvi_dev_ypos  (void);
extern unsigned int dvi_npages    (void);
extern const char  *dvi_comment   (void);

extern void dvi_vf_init   (int dev_font_id);
extern void dvi_vf_finish (void);

extern void dvi_set_font (int font_id);
extern void dvi_set      (int32_t ch);
extern void dvi_rule     (int32_t width, int32_t height);

extern void dvi_right (int32_t x);
extern void dvi_put   (int32_t ch);
extern void dvi_push  (void);
extern void dvi_pop   (void);
extern void dvi_w0    (void);
extern void dvi_w     (int32_t ch);
extern void dvi_x0    (void);
extern void dvi_x     (int32_t ch);
extern void dvi_down  (int32_t y);
extern void dvi_y     (int32_t ch);
extern void dvi_y0    (void);
extern void dvi_z     (int32_t ch);
extern void dvi_z0    (void);
extern void dvi_dirchg(unsigned char dir);

extern void  dvi_do_page  (double paper_height, double x_offset, double y_offset);
extern void  dvi_scan_specials (int page_no,
				double *width, double *height,
                                double *x_offset, double *y_offset, int *landscape,
                                int *majorversion, int *minorversion,
                                int *do_enc, int *keybits, int32_t *perm,
                                char *opasswd, char *upasswd);
extern int   dvi_locate_font   (const char *name, spt_t ptsize);

/* link or nolink:
 * See dvipdfm (not x) user's manual on pdf:link and pdf:nolink.
 * This is workaround for preventing inclusion of pagenation artifact such as
 * footnote and page number in link annotation.
 */
extern void  dvi_link_annot    (int flag);
/* The followings are for calculating bounding box of text for annotation.
 * DVI uses push/pop to do line-feed-carriage-return. So line breaking is
 * handled by inspecting current depth of DVI register stack.
 */
extern void  dvi_tag_depth     (void);
extern void  dvi_untag_depth   (void);
extern void  dvi_compute_boxes (int flag);

extern void  dvi_do_special    (const void *buffer, int32_t size);

/* allow other modules (pdfdev) to ask whether we're collecting box areas */
extern int dvi_is_tracking_boxes(void);

#endif /* _DVI_H_ */
