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

#ifndef _DVI_H_
#define _DVI_H_

#include "tectonic_bridge_core.h"

#include <stdbool.h>
#include <sys/types.h>

#include "dpx-error.h"
#include "dpx-numbers.h"
/* spt_t */
#include "dpx-pdfdev.h"

/* instantiated in dvipdfmx.c */
extern double paper_width, paper_height;
extern int    landscape_mode;

double get_origin (int x);

void  dvi_reset_global_state (void);

/* returns scale (dvi2pts) */
double dvi_init  (const char *dvi_filename, double mag); /* may append .dvi or .xdv to filename */
void   dvi_close (void);  /* Closes data structures created by dvi_open */

double       dvi_tell_mag  (void);
double       dvi_unit_size (void);
double       dvi_dev_xpos  (void);
double       dvi_dev_ypos  (void);
unsigned int dvi_npages    (void);
const char  *dvi_comment   (void);

void dvi_vf_init   (int dev_font_id);
void dvi_vf_finish (void);

void dvi_set_font (int font_id);
void dvi_set      (int32_t ch);
void dvi_rule     (int32_t width, int32_t height);

void dvi_right (int32_t x);
void dvi_put   (int32_t ch);
void dvi_push  (void);
void dpx_dvi_pop(void); /* Renamed to avoid clash with XeTeX */
void dvi_w0    (void);
void dvi_w     (int32_t ch);
void dvi_x0    (void);
void dvi_x     (int32_t ch);
void dvi_down  (int32_t y);
void dvi_y     (int32_t ch);
void dvi_y0    (void);
void dvi_z     (int32_t ch);
void dvi_z0    (void);
void dvi_dirchg(unsigned char dir);

void  dvi_do_page  (double paper_height, double x_offset, double y_offset);
void  dvi_scan_specials (int page_no,
                                double *width, double *height,
                                double *x_offset, double *y_offset, int *landscape,
                                int *majorversion, int *minorversion,
                                int *do_enc, int *keybits, int32_t *perm,
                                char *opasswd, char *upasswd, int *has_id, unsigned char *id1, unsigned char *id2);
unsigned int dvi_locate_font (const char *name, spt_t ptsize);

/* link or nolink:
 * See dvipdfm (not x) user's manual on pdf:link and pdf:nolink.
 * This is workaround for preventing inclusion of pagenation artifact such as
 * footnote and page number in link annotation.
 */
void  dvi_link_annot    (int flag);
/* The followings are for calculating bounding box of text for annotation.
 * DVI uses push/pop to do line-feed-carriage-return. So line breaking is
 * handled by inspecting current depth of DVI register stack.
 */
void  dvi_tag_depth     (void);
void  dvi_untag_depth   (void);
void  dvi_compute_boxes (int flag);

void  dvi_do_special    (const void *buffer, int32_t size);

/* allow other modules (pdfdev) to ask whether we're collecting box areas */
bool dvi_is_tracking_boxes(void);

#endif /* _DVI_H_ */
