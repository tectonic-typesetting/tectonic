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

#ifndef _PDF_DRAW_H_
#define _PDF_DRAW_H_

#include "tectonic_bridge_core.h"

#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"

#define  PDF_DASH_SIZE_MAX  16
#define  PDF_GSAVE_MAX      256

void  pdf_dev_init_gstates  (void);
void  pdf_dev_clear_gstates (void);

#define pdf_copymatrix(m,n) do {\
  (m)->a = (n)->a; (m)->b = (n)->b;\
  (m)->c = (n)->c; (m)->d = (n)->d;\
  (m)->e = (n)->e; (m)->f = (n)->f;\
} while (0)

#define pdf_setmatrix(m,p,q,r,s,t,u) do {\
  (m)->a = (p); (m)->b = (q);\
  (m)->c = (r); (m)->d = (s);\
  (m)->e = (t); (m)->f = (u);\
} while (0)

/* m -> n x m */
#define pdf_concatmatrix(m,n) do {\
  double _tmp_a, _tmp_b, _tmp_c, _tmp_d; \
  _tmp_a = (m)->a; _tmp_b = (m)->b; \
  _tmp_c = (m)->c; _tmp_d = (m)->d; \
  (m)->a  = ((n)->a) * _tmp_a + ((n)->b) * _tmp_c; \
  (m)->b  = ((n)->a) * _tmp_b + ((n)->b) * _tmp_d; \
  (m)->c  = ((n)->c) * _tmp_a + ((n)->d) * _tmp_c; \
  (m)->d  = ((n)->c) * _tmp_b + ((n)->d) * _tmp_d; \
  (m)->e += ((n)->e) * _tmp_a + ((n)->f) * _tmp_c; \
  (m)->f += ((n)->e) * _tmp_b + ((n)->f) * _tmp_d; \
} while (0)

typedef struct pdf_path_ pdf_path;

int    pdf_dev_currentmatrix (pdf_tmatrix *M);
int    pdf_dev_currentpoint  (pdf_coord *cp);

int    pdf_dev_setlinewidth  (double  width);
int    pdf_dev_setmiterlimit (double  mlimit);
int    pdf_dev_setlinecap    (int     style);
int    pdf_dev_setlinejoin   (int     style);
int    pdf_dev_setdash       (int     count,
                                     double *pattern,
                                     double  offset);

/* Path Construction */
int    pdf_dev_moveto        (double x , double y);
int    pdf_dev_rmoveto       (double x , double y);
int    pdf_dev_closepath     (void);

int    pdf_dev_lineto        (double x0 , double y0);
int    pdf_dev_rlineto       (double x0 , double y0);
int    pdf_dev_curveto       (double x0 , double y0,
                                     double x1 , double y1,
                                     double x2 , double y2);
int    pdf_dev_vcurveto      (double x0 , double y0,
                                     double x1 , double y1);
int    pdf_dev_ycurveto      (double x0 , double y0,
                                     double x1 , double y1);
int    pdf_dev_rcurveto      (double x0 , double y0,
                                     double x1 , double y1,
                                     double x2 , double y2);
int    pdf_dev_arc           (double c_x, double c_y, double r,
                                     double a_0, double a_1);
int    pdf_dev_arcn          (double c_x, double c_y, double r,
                                     double a_0, double a_1);

#define PDF_FILL_RULE_NONZERO 0
#define PDF_FILL_RULE_EVENODD 1

int    pdf_dev_newpath       (void);

/* Path Painting */
int    pdf_dev_clip          (void);
int    pdf_dev_eoclip        (void);

int    pdf_dev_rectfill      (double x, double y, double w, double h);
int    pdf_dev_rectclip      (double x, double y, double w, double h);
int    pdf_dev_rectadd       (double x, double y, double w, double h);

int    pdf_dev_flushpath     (char p_op, int fill_rule);

int    pdf_dev_concat        (const pdf_tmatrix *M);
/* NULL pointer of M mean apply current transformation */
void   pdf_dev_dtransform    (pdf_coord *p, const pdf_tmatrix *M);
void   pdf_dev_idtransform   (pdf_coord *p, const pdf_tmatrix *M);
void   pdf_dev_transform     (pdf_coord *p, const pdf_tmatrix *M);

int    pdf_dev_gsave         (void);
int    pdf_dev_grestore      (void);

/* Requires from mpost.c because new MetaPost graphics must initialize
 * the current gstate. */
int    pdf_dev_push_gstate (void);
int    pdf_dev_pop_gstate (void);


/* extension */
int    pdf_dev_arcx          (double c_x, double c_y,
                                     double r_x, double r_y,
                                     double a_0, double a_1,
                                     int    a_d, /* arc direction   */
                                     double xar  /* x-axis-rotation */
                                    );
int    pdf_dev_bspline       (double x0, double y0,
                                     double x1, double y1,
                                     double x2, double y2);


void   pdf_invertmatrix      (pdf_tmatrix *M);

/* The depth here is the depth of q/Q nesting.
 * We must remember current depth of nesting when starting a page or xform,
 * and must recover until that depth at the end of page/xform.
 */
int    pdf_dev_current_depth (void);
void   pdf_dev_grestore_to   (int depth);

void pdf_dev_set_fixed_point (double x, double y);
void pdf_dev_get_fixed_point (pdf_coord *p);

void   pdf_dev_set_color     (const pdf_color *color, char mask, int force);
#define pdf_dev_set_strokingcolor(c)     pdf_dev_set_color(c,    0, 0);
#define pdf_dev_set_nonstrokingcolor(c)  pdf_dev_set_color(c, 0x20, 0);

#endif /* _PDF_DRAW_H_ */
