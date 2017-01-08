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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include "system.h"
#include "mem.h"
#include "error.h"

#include "numbers.h"
#include "dpxutil.h"

#include "pdfdoc.h"

#include "pdfdraw.h"
#include "pdfdev.h"

#include "specials.h"
#include "spc_tpic.h"

#define  DEBUG 1
#define  ENABLE_SPC_NAMESPACE 1

/*
 * Following "constant" converts milli-inches to
 * device (in this case PDF stream) coordinates.
 */

#define MI2DEV (0.072/pdf_dev_scale())

/*
 * Value for 'sh' command 'g' is interpreted as
 * 
 *   gray color value 1-g for "solid"
 *   opacity value g for "opacity"
 *   shape value g for "shape"
 */
#define TPIC_MODE__FILL_SOLID   0
#define TPIC_MODE__FILL_OPACITY 1
#define TPIC_MODE__FILL_SHAPE   2

#ifndef ISBLANK
#  define ISBLANK(c) ( (c) == ' ' || (c) == '\t' )
#endif

static void
skip_blank (const char **pp, const char *endptr)
{
  const char  *p = *pp;
  for ( ; p < endptr && ISBLANK(*p); p++);
  *pp = p;
}

struct spc_tpic_
{
  struct {
    int   fill;
  } mode;

  /* state */
  double     pen_size;
  int        fill_shape; /* boolean */
  double     fill_color;

  pdf_coord *points;
  int        num_points;
  int        max_points;
};

#if  1
static struct spc_tpic_ _tpic_state;
#endif

/* We use pdf_doc_add_page_content() here
 * since we always draw isolated graphics.
 */
static void
tpic__clear (struct spc_tpic_ *tp) 
{
  if (tp->points) {
    RELEASE(tp->points);
    tp->points = NULL;
  }
  tp->num_points = 0;
  tp->max_points = 0;
  tp->fill_shape = 0;
  tp->fill_color = 0.0;
}


static pdf_obj *
create_xgstate (double a /* alpha */, int f_ais /* alpha is shape */)
{
  pdf_obj  *dict;

  dict = pdf_new_dict();
  pdf_add_dict(dict,
               pdf_new_name("Type"),
               pdf_new_name("ExtGState"));
  if (f_ais) {
    pdf_add_dict(dict,
                 pdf_new_name("AIS"),
                 pdf_new_boolean(1));
  }
  pdf_add_dict(dict,
               pdf_new_name("ca"),
               pdf_new_number(a));

  return  dict;
}

static int
check_resourcestatus (const char *category, const char *resname)
{
  pdf_obj  *dict1, *dict2;

  dict1 = pdf_doc_current_page_resources();
  if (!dict1)
    return  0;

  dict2 = pdf_lookup_dict(dict1, category);
  if (dict2 &&
      pdf_obj_typeof(dict2) == PDF_DICT) {
    if (pdf_lookup_dict(dict2, resname))
      return  1;
  }
  return  0;
}

static int
set_linestyle (double pn, double da)
{
  double  dp[2]; /* dash pattern */

  pdf_dev_setlinejoin(1);
  pdf_dev_setmiterlimit(1.4);
  pdf_dev_setlinewidth(pn);
  if (da > 0.0) {
    dp[0] =  da * 72.0;
    pdf_dev_setdash(1, dp, 0);
    pdf_dev_setlinecap(0);
  } else if (da < 0.0) {
    dp[0] =  pn;
    dp[1] = -da * 72.0;
    pdf_dev_setdash(2, dp, 0);
    pdf_dev_setlinecap(1);
  } else {
    pdf_dev_setlinecap(0);
  }

  return  0;
}

static int
set_fillstyle (double g, double a, int f_ais)
{
  pdf_obj *dict;
  char     resname[32];
  char     buf[32];
  int      alp, len = 0;

  if (a > 0.0) {
    alp = round(100.0 * a);
    sprintf(resname, "_Tps_a%03d_", alp);
    if (!check_resourcestatus("ExtGState", resname)) {
      dict = create_xgstate(ROUND(0.01 * alp, 0.01), f_ais);
      pdf_doc_add_page_resource("ExtGState",
                                resname, pdf_ref_obj(dict));
      pdf_release_obj(dict);
    }
    len += sprintf(buf + len, " /%s gs", resname);

    pdf_doc_add_page_content(buf, len);  /* op: gs */
  }

  {
    pdf_color *sc, *fc, new_fc;

    pdf_color_get_current (&sc, &fc); /* get stroking and fill colors */
    pdf_color_brighten_color(&new_fc, fc, g);
    pdf_dev_set_nonstrokingcolor(&new_fc);
  }

  return  0;
}

static void
set_styles (struct spc_tpic_ *tp,
	    const pdf_coord  *c,
	    int               f_fs,
	    int               f_vp,
	    double            pn,
	    double            da) {
  pdf_tmatrix M;

  pdf_setmatrix (&M, 1.0, 0.0, 0.0, -1.0, c->x, c->y);
  pdf_dev_concat(&M);

  if (f_vp)
    set_linestyle(pn, da);

  if (f_fs) {
    double g, a;
    int f_ais;

    if (tp->mode.fill == TPIC_MODE__FILL_SOLID || !tp->fill_color) {
      g = 1.0 - tp->fill_color;
      a = 0.0;
    } else {
      g = 0.0;
      a = tp->fill_color;
    }

    f_ais = (tp->mode.fill == TPIC_MODE__FILL_SHAPE) ? 1 : 0;

    set_fillstyle(g, a, f_ais);
  }
}

static void
showpath (int f_vp, int f_fs) /* visible_path, fill_shape */
{
  if (f_vp) {
    if (f_fs)
      pdf_dev_flushpath('b', PDF_FILL_RULE_NONZERO);
    else {
      pdf_dev_flushpath('S', PDF_FILL_RULE_NONZERO);
    }
  } else {
    /*
     * Acrobat claims 'Q' as illegal operation when there are unfinished
     * path (a path without path-painting operator applied)?
     */
    if (f_fs)
      pdf_dev_flushpath('f', PDF_FILL_RULE_NONZERO);
    else {
      pdf_dev_newpath();
    }
  }
}

#define CLOSED_PATH(s) (\
  (s)->points[0].x == (s)->points[(s)->num_points-1].x && \
  (s)->points[0].y == (s)->points[(s)->num_points-1].y \
)

static int
tpic__polyline (struct spc_tpic_ *tp,
                const pdf_coord  *c,
                int               f_vp,
                double            da)
{
  double       pn    = tp->pen_size;
  int          f_fs  = tp->fill_shape;
  int          i, error = 0;

  /* Shading is applied only to closed path. */
  f_fs  = CLOSED_PATH(tp) ? f_fs : 0;
  f_vp  = (pn > 0.0) ? f_vp : 0;

  if (f_vp || f_fs) {
    pdf_dev_gsave();

    set_styles(tp, c, f_fs, f_vp, pn, da);

    pdf_dev_moveto(tp->points[0].x, tp->points[0].y);
    for (i = 0; i < tp->num_points; i++)
      pdf_dev_lineto(tp->points[i].x, tp->points[i].y);

    showpath(f_vp, f_fs);

    pdf_dev_grestore();
  }

  tpic__clear(tp);

  return  error;
}

/*
 * Accroding to
 * "Tpic: Pic for TEX", Tim Morgan, Original by Brian Kernighan, p.20:
 * 
 *  A spline is a smooth curve guided by a set of straight lines just
 *  like the line above. It begins at the same place, ends at the same
 *  place, and in between is tangent to the mid-point of each guiding
 *  line. The syntax for a spline is identical to a (path) line except
 *  for using spline instead of line.
 *
 * Spline is not a curve drawn by spline-fitting points p0, p1, ..., pn,
 * defined by tpic special "pa" command. Instead, a path defined by set
 * of points p0, p1, ... is guiding line mentioned above.
 *
 * Dvipsk draws them as a straight line from p0 to q1 = (p0 + p1)/2,
 * followed by a quadratic B-spline curve with starting point q1, (off-
 * curve) control point p1, end point q2 = (p1 + p2)/2, ..., and a
 * straight line from qn to pn.
 */

static int
tpic__spline (struct spc_tpic_ *tp,
              const pdf_coord  *c,
              int               f_vp,
              double            da)
{
  double       v[6];
  double       pn    = tp->pen_size;
  int          f_fs  = tp->fill_shape;
  int          i, error = 0;

  f_fs  = CLOSED_PATH(tp) ? f_fs : 0;
  f_vp  = (pn > 0.0) ? f_vp : 0;

  if (f_vp || f_fs) {
    pdf_dev_gsave();

    set_styles(tp, c, f_fs, f_vp, pn, da);

    pdf_dev_moveto(tp->points[0].x, tp->points[0].y);

    v[0] = 0.5 * (tp->points[0].x + tp->points[1].x);
    v[1] = 0.5 * (tp->points[0].y + tp->points[1].y);
    pdf_dev_lineto(v[0], v[1]);
    for (i = 1; i < tp->num_points - 1; i++) {
      /* B-spline control points */
      v[0] = 0.5 * (tp->points[i-1].x + tp->points[i].x);
      v[1] = 0.5 * (tp->points[i-1].y + tp->points[i].y);
      v[2] = tp->points[i].x;
      v[3] = tp->points[i].y;
      v[4] = 0.5 * (tp->points[i].x + tp->points[i+1].x);
      v[5] = 0.5 * (tp->points[i].y + tp->points[i+1].y);
      pdf_dev_bspline(v[0], v[1], v[2], v[3], v[4], v[5]);
    }
    pdf_dev_lineto(tp->points[i].x, tp->points[i].y);

    showpath(f_vp, f_fs);

    pdf_dev_grestore();
  }
  tpic__clear(tp);

  return  error;
}

static int
tpic__arc (struct spc_tpic_ *tp,
           const pdf_coord  *c,
           int               f_vp,
           double            da,
           double           *v /* 6 numbers */ )
{
  double       pn    = tp->pen_size;
  int          f_fs  = tp->fill_shape;

  f_fs  = (round(fabs(v[4] - v[5]) + 0.5) >= 360) ? f_fs : 0;
  f_vp  = (pn > 0.0) ? f_vp : 0;

  if (f_vp || f_fs) {
    pdf_dev_gsave();

    set_styles(tp, c, f_fs, f_vp, pn, da);

    /* The arcx operator here draws an excess straight line from current
     * point to the starting point of the arc if they are different, as in
     * PostScript language. It may cuase an unexpected behavior when DVIPS
     * transformation command is inserted before TPIC ar command: it invokes
     * moveto and sets currentpoint which may be different from the starting
     * point of arc to be drawn. We use newpath here to avoid drawing an
     * excess line. I'm not sure if it is proper TPIC implementation but this
     * seems to be DVIPS compatible behavior.
     */
    pdf_dev_newpath();
    pdf_dev_arcx(v[0], v[1], v[2], v[3], v[4], v[5], +1, 0.0);

    showpath(f_vp, f_fs);

    pdf_dev_grestore();
  }
  tpic__clear(tp);

  return  0;
}

#if  1
static int
spc_currentpoint (struct spc_env *spe, int *pg, pdf_coord *cp)
{
  *pg = 0;
  cp->x = spe->x_user;
  cp->y = spe->y_user;
  return  0;
}
#endif

static int
spc_handler_tpic_pn (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct spc_tpic_ *tp = &_tpic_state;
  char  *q;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (!q) {
    spc_warn(spe, "Invalid pen size specified?");
    return -1;
  }
  tp->pen_size = atof(q) * MI2DEV;
  RELEASE(q);

  return  0;
}

static int
spc_handler_tpic_pa (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct spc_tpic_ *tp = &_tpic_state;
  char   *q;
  int     i;
  double  v[2];

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  for (i = 0;
       i < 2 && ap->curptr < ap->endptr; i++) {
    q = parse_float_decimal(&ap->curptr, ap->endptr);
    if (!q) {
      spc_warn(spe, "Missing numbers for TPIC \"pa\" command.");
      return  -1;
    }
    v[i] = atof(q);
    RELEASE(q);
    skip_blank(&ap->curptr, ap->endptr);
  }
  if (i != 2) {
    spc_warn(spe, "Invalid arg for TPIC \"pa\" command.");
    return  -1;
  }

  if (tp->num_points >= tp->max_points) {
    tp->max_points += 256;
    tp->points = RENEW(tp->points, tp->max_points, pdf_coord);
  }
  tp->points[tp->num_points].x = v[0] * MI2DEV;
  tp->points[tp->num_points].y = v[1] * MI2DEV;
  tp->num_points += 1;

  return  0;
}

static int
spc_handler_tpic_fp (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct spc_tpic_ *tp = &_tpic_state;
  pdf_coord  cp;
  int        pg;

  ASSERT(spe && ap && tp);

  if (tp->num_points <= 1) {
    spc_warn(spe, "Too few points (< 2) for polyline path.");
    return  -1;
  }

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__polyline(tp, &cp, 1, 0.0);
}

static int
spc_handler_tpic_ip (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct spc_tpic_ *tp = &_tpic_state;
  pdf_coord  cp;
  int        pg;

  ASSERT(spe && ap && tp);

  if (tp->num_points <= 1) {
    spc_warn(spe, "Too few points (< 2) for polyline path.");
    return  -1;
  }

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__polyline(tp, &cp, 0, 0.0);
}

static int
spc_handler_tpic_da (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  char      *q;
  double     da = 0.0;
  pdf_coord  cp;
  int        pg;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (q) {
    da = atof(q);
    RELEASE(q);
  }
  if (tp->num_points <= 1) {
    spc_warn(spe, "Too few points (< 2) for polyline path.");
    return  -1;
  }

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__polyline(tp, &cp, 1, da);
}

static int
spc_handler_tpic_dt (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  char      *q;
  double     da = 0.0;
  pdf_coord  cp;
  int        pg;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (q) {
    da = -atof(q);
    RELEASE(q);
  }
  if (tp->num_points <= 1) {
    spc_warn(spe, "Too few points (< 2) for polyline path.");
    return  -1;
  }

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__polyline(tp, &cp, 1, da);
}

static int
spc_handler_tpic_sp (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  char      *q;
  double     da = 0.0;
  pdf_coord  cp;
  int        pg;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (q) {
    da = atof(q);
    RELEASE(q);
  }
  if (tp->num_points <= 2) {
    spc_warn(spe, "Too few points (< 3) for spline path.");
    return  -1;
  }

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__spline(tp, &cp, 1, da);
}

static int
spc_handler_tpic_ar (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  double     v[6];
  pdf_coord  cp;
  int        pg;
  char      *q;
  int        i;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  for (i = 0;
       i < 6 && ap->curptr < ap->endptr; i++) {
    q = parse_float_decimal(&ap->curptr, ap->endptr);
    if (!q) {
      spc_warn(spe, "Invalid args. in TPIC \"ar\" command.");
      return  -1;
    }
    v[i] = atof(q);
    RELEASE(q);
    skip_blank(&ap->curptr, ap->endptr);
  }
  if (i != 6) {
    spc_warn(spe, "Invalid arg for TPIC \"ar\" command.");
    return  -1;
  }

  v[0] *= MI2DEV; v[1] *= MI2DEV;
  v[2] *= MI2DEV; v[3] *= MI2DEV;
  v[4] *= 180.0 / M_PI;
  v[5] *= 180.0 / M_PI;

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__arc(tp, &cp, 1, 0.0, v);
}

static int
spc_handler_tpic_ia (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  double     v[6];
  pdf_coord  cp;
  int        pg;
  char      *q;
  int        i;

  ASSERT(spe && ap && tp);

  skip_blank(&ap->curptr, ap->endptr);
  for (i = 0;
       i < 6 && ap->curptr < ap->endptr; i++) {
    q = parse_float_decimal(&ap->curptr, ap->endptr);
    if (!q) {
      spc_warn(spe, "Invalid args. in TPIC \"ia\" command.");
      return  -1;
    }
    v[i] = atof(q);
    RELEASE(q);
    skip_blank(&ap->curptr, ap->endptr);
  }
  if (i != 6) {
    spc_warn(spe, "Invalid arg for TPIC \"ia\" command.");
    return  -1;
  }

  v[0] *= MI2DEV; v[1] *= MI2DEV;
  v[2] *= MI2DEV; v[3] *= MI2DEV;
  v[4] *= 180.0 / M_PI;
  v[5] *= 180.0 / M_PI;

  spc_currentpoint(spe, &pg, &cp);

  return  tpic__arc(tp, &cp, 0, 0.0, v);
}

static int
spc_handler_tpic_sh (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;
  char   *q;

  ASSERT(spe && ap && tp);

  tp->fill_shape = 1;
  tp->fill_color = 0.5;

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (q) {
    double g = atof(q);
    RELEASE(q);
    if (g >= 0.0 && g <= 1.0)
      tp->fill_color = g;
    else {
      WARN("Invalid fill color specified: %g\n", g);
      return -1;
    }      
  }

  return  0;
}

static int
spc_handler_tpic_wh (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;

  ASSERT(spe && ap && tp);

  tp->fill_shape = 1;
  tp->fill_color = 0.0;

  return  0;
}

static int
spc_handler_tpic_bk (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;

  ASSERT(spe && ap && tp);

  tp->fill_shape = 1;
  tp->fill_color = 1.0;

  return  0;
}

static int
spc_handler_tpic_tx (struct spc_env *spe,
                     struct spc_arg *ap ) /* , void *dp) */
{
  struct  spc_tpic_ *tp = &_tpic_state;

  ASSERT(spe && ap && tp);

  spc_warn(spe, "TPIC command \"tx\" not supported.");

  return  -1;
}


static int
spc_handler_tpic__init (struct spc_env *spe, void *dp)
{
  struct spc_tpic_ *tp = dp;

#if  0
  tp->mode.fill  = TPIC_MODE__FILL_SOLID;
#endif 
  tp->pen_size   = 1.0;
  tp->fill_shape = 0;
  tp->fill_color = 0.0;

  tp->points     = NULL;
  tp->num_points = 0;
  tp->max_points = 0;

  if (tp->mode.fill != TPIC_MODE__FILL_SOLID && pdf_get_version() < 4) {
      spc_warn(spe, "Tpic shading support requires PDF version 1.4.");
    tp->mode.fill = TPIC_MODE__FILL_SOLID;
  }

  return  0;
}

static int
spc_handler_tpic__bophook (void *dp)
{
  struct spc_tpic_ *tp = dp;

  ASSERT(tp);

  tpic__clear(tp);

  return  0;
}

static int
spc_handler_tpic__eophook (struct spc_env *spe, void *dp)
{
  struct spc_tpic_ *tp = dp;

  ASSERT(tp);

  if (tp->num_points > 0)
    spc_warn(spe, "Unflushed tpic path at end of the page.");
  tpic__clear(tp);

  return  0;
}

static int
spc_handler_tpic__clean (struct spc_env *spe, void *dp)
{
  struct spc_tpic_ *tp = dp;

  ASSERT(tp);

  if (tp->num_points > 0)
    spc_warn(spe, "Unflushed tpic path at end of the document.");

  tpic__clear(tp);
#if  0
  RELEASE(tp);
#endif

  return  0;
}

void
tpic_set_fill_mode (int mode)
{
  struct spc_tpic_ *tp = &_tpic_state;
  tp->mode.fill = mode;
}


int
spc_tpic_at_begin_page (void)
{
  struct spc_tpic_ *tp = &_tpic_state;
  return  spc_handler_tpic__bophook(tp);
}

int
spc_tpic_at_end_page (void)
{
  struct spc_tpic_ *tp = &_tpic_state;
  return  spc_handler_tpic__eophook(NULL, tp);
}


int
spc_tpic_at_begin_document (void)
{
  struct spc_tpic_ *tp = &_tpic_state;
  return  spc_handler_tpic__init(NULL, tp);
}

int
spc_tpic_at_end_document (void)
{
  struct spc_tpic_ *tp = &_tpic_state;
  return  spc_handler_tpic__clean(NULL, tp);
}


#if  DEBUG
#include "pdfparse.h" /* parse_val_ident :( */

static pdf_obj *
spc_parse_kvpairs (struct spc_arg *ap)
{
  pdf_obj *dict;
  char    *kp, *vp;
  int      error = 0;

  dict = pdf_new_dict();

  skip_blank(&ap->curptr, ap->endptr);
  while (!error && ap->curptr < ap->endptr) {
    kp = parse_val_ident(&ap->curptr, ap->endptr);
    if (!kp)
      break;
    skip_blank(&ap->curptr, ap->endptr);
    if (ap->curptr < ap->endptr &&
        ap->curptr[0] == '=') {
      ap->curptr++;
      skip_blank(&ap->curptr, ap->endptr);
      if (ap->curptr == ap->endptr) {
        RELEASE(kp);
        error = -1;
        break;
      }
      vp = parse_c_string(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        pdf_add_dict(dict,
                     pdf_new_name(kp),
                     pdf_new_string(vp, strlen(vp) + 1)); /* NULL terminate */
        RELEASE(vp);
      }
    } else {
      /* Treated as 'flag' */
      pdf_add_dict(dict,
                   pdf_new_name(kp),
                   pdf_new_boolean(1));
    }
    RELEASE(kp);
    if (!error)
      skip_blank(&ap->curptr, ap->endptr);
  }

  if (error) {
    pdf_release_obj(dict);
    dict = NULL;
  }

  return  dict;
}

static int
tpic_filter_getopts (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  struct spc_tpic_ *tp = dp;
  char  *k, *v;
  int    error = 0;

  ASSERT( kp && vp && tp );

  k = pdf_name_value(kp);
  if (!strcmp(k, "fill-mode")) {
    if (pdf_obj_typeof(vp) != PDF_STRING) {
      WARN("Invalid value for TPIC option fill-mode...");
      error = -1;
    } else {
      v = pdf_string_value(vp);
      if (!strcmp(v, "shape"))
        tp->mode.fill = TPIC_MODE__FILL_SHAPE;
      else if (!strcmp(v, "opacity"))
        tp->mode.fill = TPIC_MODE__FILL_OPACITY;
      else if (!strcmp(v, "solid"))
        tp->mode.fill = TPIC_MODE__FILL_SOLID;
      else {
        WARN("Invalid value for TPIC option fill-mode: %s", v);
        error = -1;
      }
    }
  } else {
    WARN("Unrecognized option for TPIC special handler: %s", k);
    error = -1;
  }

  return  error;
}

static int
spc_handler_tpic__setopts (struct spc_env *spe,
                           struct spc_arg *ap)
{
  struct spc_tpic_ *tp = &_tpic_state;
  pdf_obj  *dict;
  int       error = 0;

  dict  = spc_parse_kvpairs(ap);
  if (!dict)
    return  -1;
  error = pdf_foreach_dict(dict, tpic_filter_getopts, tp);
  if (!error) {
    if (tp->mode.fill != TPIC_MODE__FILL_SOLID &&
        pdf_get_version() < 4) {
      spc_warn(spe, "Transparent fill mode requires PDF version 1.4.");
      tp->mode.fill = TPIC_MODE__FILL_SOLID;
    }
  }

  return  error;
}
#endif  /* DEBUG */


static struct spc_handler tpic_handlers[] = {
  {"pn", spc_handler_tpic_pn},
  {"pa", spc_handler_tpic_pa},
  {"fp", spc_handler_tpic_fp},
  {"ip", spc_handler_tpic_ip},
  {"da", spc_handler_tpic_da},
  {"dt", spc_handler_tpic_dt},
  {"sp", spc_handler_tpic_sp},
  {"ar", spc_handler_tpic_ar},
  {"ia", spc_handler_tpic_ia},
  {"sh", spc_handler_tpic_sh},
  {"wh", spc_handler_tpic_wh},
  {"bk", spc_handler_tpic_bk},
  {"tx", spc_handler_tpic_tx}
};

int
spc_tpic_check_special (const char *buf, int len)
{
  int    istpic = 0;
  char  *q;
  const char *p, *endptr;
  int    i, hasnsp = 0;

  p      = buf;
  endptr = p + len;

  skip_blank(&p, endptr);
#if  ENABLE_SPC_NAMESPACE
  if (p + strlen("tpic:") < endptr &&
      !memcmp(p, "tpic:", strlen("tpic:")))
  {
    p += strlen("tpic:");
    hasnsp = 1;
  }
#endif
  q = parse_c_ident(&p, endptr);

  if (!q)
    istpic = 0;
  else if (q && hasnsp && !strcmp(q, "__setopt__")) {
#if  DEBUG
    istpic = 1;
#endif
    RELEASE(q);
  } else {
    for (i = 0;
         i < sizeof(tpic_handlers)/sizeof(struct spc_handler); i++) {
      if (!strcmp(q, tpic_handlers[i].key)) {
        istpic = 1;
        break;
      }
    }
    RELEASE(q);
  }

  return  istpic;
}


int
spc_tpic_setup_handler (struct spc_handler *sph,
                        struct spc_env *spe, struct spc_arg *ap)
{
  char  *q;
  int    i, hasnsp = 0, error = -1;

  ASSERT(sph && spe && ap);

  skip_blank(&ap->curptr, ap->endptr);
#if  ENABLE_SPC_NAMESPACE
  if (ap->curptr + strlen("tpic:") < ap->endptr &&
      !memcmp(ap->curptr, "tpic:", strlen("tpic:")))
  {
    ap->curptr += strlen("tpic:");
    hasnsp = 1;
  }
#endif
  q = parse_c_ident(&ap->curptr, ap->endptr);

  if (!q)
    error = -1;
  else if (q && hasnsp && !strcmp(q, "__setopt__")) {
#if  DEBUG
    ap->command = "__setopt__";
    sph->key    = "tpic:";
    sph->exec   = spc_handler_tpic__setopts;
    skip_blank(&ap->curptr, ap->endptr);
    error = 0;
#endif
    RELEASE(q);
  } else {
    for (i = 0;
         i < sizeof(tpic_handlers)/sizeof(struct spc_handler); i++) {
      if (!strcmp(q, tpic_handlers[i].key)) {
        ap->command = tpic_handlers[i].key;
        sph->key    = "tpic:";
        sph->exec   = tpic_handlers[i].exec;
        skip_blank(&ap->curptr, ap->endptr);
        error = 0;
        break;
      }
    }
    RELEASE(q);
  }

  return  error;
}


#if  0
int
spc_load_tpic_special  (struct spc_env *spe, pdf_obj *lopts)
{
  struct spc_def   *spd;
  struct spc_tpic_ *sd;

  sd  = NEW(1, struct spc_tpic_);

  spd = NEW(1, struct spc_def);
  spc_init_def(spd);

  spc_def_init   (spd, &spc_handler_tpic__init);
  spc_def_setopts(spd, &spc_handler_tpic__setopts);
  spc_def_bophook(spd, &spc_handler_tpic__bophook);
  spc_def_eophook(spd, &spc_handler_tpic__eophook);
  spc_def_clean  (spd, &spc_handler_tpic__clean);

  spc_def_func(spd, "pn", &spc_handler_tpic_pn);
  spc_def_func(spd, "pa", &spc_handler_tpic_pa);
  spc_def_func(spd, "fp", &spc_handler_tpic_fp);
  spc_def_func(spd, "ip", &spc_handler_tpic_ip);
  spc_def_func(spd, "da", &spc_handler_tpic_da);
  spc_def_func(spd, "dt", &spc_handler_tpic_dt);
  spc_def_func(spd, "sp", &spc_handler_tpic_sp);
  spc_def_func(spd, "ar", &spc_handler_tpic_ar);
  spc_def_func(spd, "ia", &spc_handler_tpic_ia);
  spc_def_func(spd, "sh", &spc_handler_tpic_sh);
  spc_def_func(spd, "wh", &spc_handler_tpic_wh);
  spc_def_func(spd, "bk", &spc_handler_tpic_bk);
  spc_def_func(spd, "tx", &spc_handler_tpic_tx);

  spc_add_special(spe, "tpic", spd, sd);

  return  0;
}
#endif /* 0 */

