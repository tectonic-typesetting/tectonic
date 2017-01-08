/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2016 by Jin-Hwan Cho and Shunsaku Hirata,
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
#include "dpxutil.h"

#include "pdfdev.h"
#include "pdfparse.h"
#include "pdfcolor.h"
#include "pdfdraw.h"

#include "specials.h"

#include "spc_util.h"


#ifndef ISBLANK
#define ISBLANK(c) ((c) == ' ' || (c) == '\t')
#endif
static void
skip_blank (const char **pp, const char *endptr)
{
  const char  *p = *pp;
  for ( ; p < endptr && ISBLANK(*p); p++);
  *pp = p;
}


/* From pdfcolor.c */
static int pdf_color_namedcolor (pdf_color *color, const char *colorname);

int
spc_util_read_numbers (double *values, int num_values, struct spc_arg *args)
{
  int   count;
  char *q;

  skip_blank(&args->curptr, args->endptr);
  for (count = 0;
       count < num_values &&
       args->curptr < args->endptr; ) {
    q = parse_float_decimal(&args->curptr, args->endptr);
    if (!q)
      break;
    else {
      values[count] = atof(q);
      RELEASE(q);
      skip_blank(&args->curptr, args->endptr);
      count++;
    }
  }

  return  count;
}

static void
rgb_color_from_hsv (pdf_color *color, double h, double s, double v)
{
  double  r, g, b;
  ASSERT( color );
  r = g = b = v;
  if (s != 0.0) {
    double h6, f, v1, v2, v3;
    int    i;
    h6 = h * 6; /* 360 / 60 */
    i  = (int) h6;
    f  = h6 - i;
    v1 = v * (1 - s);
    v2 = v * (1 - s * f);
    v3 = v * (1 - s * (1 - f));
    switch (i) {
    case  0: r = v ; g = v3; b = v1; break;
    case  1: r = v2; g = v ; b = v1; break;
    case  2: r = v1; g = v ; b = v3; break;
    case  3: r = v1; g = v2; b = v ; break;
    case  4: r = v3; g = v1; b = v ; break;
    case  5: r = v ; g = v1; b = v2; break;
    case  6: r = v ; g = v1; b = v2; break;
    }
  }
  pdf_color_rgbcolor(color, r, g, b);
}

static int
spc_read_color_color (struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap)
{
  char    *q;
  double   cv[4];
  int      nc;
  int      error = 0;

  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (!q) {
    spc_warn(spe, "No valid color specified?");
    return  -1;
  }
  skip_blank(&ap->curptr, ap->endptr);

  if (!strcmp(q, "rgb")) { /* Handle rgb color */
    nc = spc_util_read_numbers(cv, 3, ap);
    if (nc != 3) {
      spc_warn(spe, "Invalid value for RGB color specification.");
      error = -1;
    } else {
      pdf_color_rgbcolor(colorspec, cv[0], cv[1], cv[2]);
    }
  } else if (!strcmp(q, "cmyk")) { /* Handle cmyk color */
    nc = spc_util_read_numbers(cv, 4, ap);
    if (nc != 4) {
      spc_warn(spe, "Invalid value for CMYK color specification.");
      error = -1;
    } else {
      pdf_color_cmykcolor(colorspec, cv[0], cv[1], cv[2], cv[3]);
    }
  } else if (!strcmp(q, "gray")) { /* Handle gray */
    nc = spc_util_read_numbers(cv, 1, ap);
    if (nc != 1) {
      spc_warn(spe, "Invalid value for gray color specification.");
      error = -1;
    } else {
      pdf_color_graycolor(colorspec, cv[0]);
    }
  } else if (!strcmp(q, "spot")) { /* Handle spot colors */
    char *color_name = parse_c_ident(&ap->curptr, ap->endptr);
    if (!color_name) {
      spc_warn(spe, "No valid spot color name specified?");
      return  -1;
    }
    skip_blank(&ap->curptr, ap->endptr);
    nc = spc_util_read_numbers(cv, 1, ap);
    if (nc != 1) {
      spc_warn(spe, "Invalid value for spot color specification.");
      error = -1;
      free(color_name);
    } else {
      pdf_color_spotcolor(colorspec, color_name, cv[0]);
    }
  } else if (!strcmp(q, "hsb")) {
    nc = spc_util_read_numbers(cv, 3, ap);
    if (nc != 3) {
      spc_warn(spe, "Invalid value for HSB color specification.");
      error = -1;
    } else {
      rgb_color_from_hsv(colorspec, cv[0], cv[1], cv[2]);
      spc_warn(spe, "HSB color converted to RGB: hsb: <%g, %g, %g> ==> rgb: <%g, %g, %g>",
               cv[0], cv[1], cv[2],
               colorspec->values[0], colorspec->values[1], colorspec->values[2]);
    }
  } else { /* Must be a "named" color */
    error = pdf_color_namedcolor(colorspec, q);
    if (error)
      spc_warn(spe, "Unrecognized color name: %s", q);
  }
  RELEASE(q);

  return  error;
}

/* Argument for this is PDF_Number or PDF_Array.
 * But we ignore that since we don't want to add
 * dependency to pdfxxx and @foo can not be
 * allowed for color specification. "pdf" here
 * means pdf: special syntax.
 */
static int
spc_read_color_pdf (struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap)
{
  double  cv[4]; /* at most four */
  int     nc, isarry = 0;
  int     error = 0;
  char   *q;

  skip_blank(&ap->curptr, ap->endptr);

  if (ap->curptr[0] == '[') {
    ap->curptr++; skip_blank(&ap->curptr, ap->endptr);
    isarry = 1;
  }

  nc = spc_util_read_numbers(cv, 4, ap);
  switch (nc) {
  case  1:
    pdf_color_graycolor(colorspec, cv[0]);
    break;
  case  3:
    pdf_color_rgbcolor (colorspec, cv[0], cv[1], cv[2]);
    break;
  case  4:
    pdf_color_cmykcolor(colorspec, cv[0], cv[1], cv[2], cv[3]);
    break;
  default:
    /* Try to read the color names defined in dvipsname.def */
    q = parse_c_ident(&ap->curptr, ap->endptr);
    if (!q) {
      spc_warn(spe, "No valid color specified?");
      return  -1;
    }
    error = pdf_color_namedcolor(colorspec, q);
    if (error)
      spc_warn(spe, "Unrecognized color name: %s, keep the current color", q);
    RELEASE(q);
    break;
  }

  if (isarry) {
    skip_blank(&ap->curptr, ap->endptr);
    if (ap->curptr >= ap->endptr || ap->curptr[0] != ']') {
      spc_warn(spe, "Unbalanced '[' and ']' in color specification.");
      error = -1;
    } else {
      ap->curptr++;
    }
  }

  return  error;
}

/* This is for reading *single* color specification. */
int
spc_util_read_colorspec (struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, int syntax)
{
  ASSERT(colorspec && spe && ap);

  skip_blank(&ap->curptr, ap->endptr);
  if (ap->curptr >= ap->endptr) {
    return -1;
  }

  if (syntax)
    return spc_read_color_color(spe, colorspec, ap);
  else
    return spc_read_color_pdf(spe, colorspec, ap);
}

int
spc_util_read_pdfcolor (struct spc_env *spe, pdf_color *colorspec, struct spc_arg *ap, pdf_color *defaultcolor)
{
  int error = 0;

  ASSERT(colorspec && spe && ap);

  skip_blank(&ap->curptr, ap->endptr);
  if (ap->curptr >= ap->endptr) {
    return -1;
  }
  error = spc_read_color_pdf(spe, colorspec, ap);
  if (error < 0 && defaultcolor) {
    pdf_color_copycolor(colorspec, defaultcolor);
    error = 0;
  }
  return error;
}

/* This need to allow 'true' prefix for unit and
 * length value must be divided by current magnification.
 */
static int
spc_util_read_length (struct spc_env *spe, double *vp /* ret. */, struct spc_arg *ap)
{
  char   *q;
  double  v, u = 1.0;
  const char *ukeys[] = {
#define K_UNIT__PT  0
#define K_UNIT__IN  1
#define K_UNIT__CM  2
#define K_UNIT__MM  3
#define K_UNIT__BP  4
    "pt", "in", "cm", "mm", "bp", NULL
  };
  int     k, error = 0;

  q = parse_float_decimal(&ap->curptr, ap->endptr);
  if (!q)
    return  -1;

  v = atof(q);
  RELEASE(q);

  skip_white(&ap->curptr, ap->endptr);
  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (q) {
    char *qq = q;
    if (strlen(q) >= strlen("true") &&
        !memcmp(q, "true", strlen("true"))) {
      u /= spe->mag != 0.0 ? spe->mag : 1.0; /* inverse magnify */
      q += strlen("true");

      if (!*q) {
        RELEASE(qq);
        skip_white(&ap->curptr, ap->endptr);
        qq = q = parse_c_ident(&ap->curptr, ap->endptr);
      }
    }

    if (q) {
      for (k = 0; ukeys[k] && strcmp(ukeys[k], q); k++);
      switch (k) {
      case K_UNIT__PT: u *= 72.0 / 72.27; break;
      case K_UNIT__IN: u *= 72.0; break;
      case K_UNIT__CM: u *= 72.0 / 2.54 ; break;
      case K_UNIT__MM: u *= 72.0 / 25.4 ; break;
      case K_UNIT__BP: u *= 1.0 ; break;
      default:
        spc_warn(spe, "Unknown unit of measure: %s", q);
        error = -1;
        break;
      }
      RELEASE(qq);
    }
    else {
      spc_warn(spe, "Missing unit of measure after \"true\"");
      error = -1;
    }
  }

  *vp = v * u;
  return  error;
}


/*
 * Compute a transformation matrix
 * transformations are applied in the following
 * order: scaling, rotate, displacement.
 */
static void
make_transmatrix (pdf_tmatrix *M,
                  double xoffset, double yoffset,
                  double xscale,  double yscale,
                  double rotate)
{
  double c, s;

  c = cos(rotate);
  s = sin(rotate);

  M->a =  xscale * c; M->b = xscale * s;
  M->c = -yscale * s; M->d = yscale * c;
  M->e = xoffset;     M->f = yoffset;
}

static int
spc_read_dimtrns_dvips (struct spc_env *spe,
                        transform_info *t, struct spc_arg *ap)
{
  static const char *_dtkeys[] = {
#define  K_TRN__HOFFSET  0
#define  K_TRN__VOFFSET  1
    "hoffset", "voffset",
#define  K_DIM__HSIZE    2
#define  K_DIM__VSIZE    3
    "hsize", "vsize",
#define  K_TRN__HSCALE   4
#define  K_TRN__VSCALE   5
    "hscale", "vscale",
#define  K_TRN__ANGLE    6
    "angle",
#define  K__CLIP         7
    "clip",
#define  K_DIM__LLX      8
#define  K_DIM__LLY      9
#define  K_DIM__URX     10
#define  K_DIM__URY     11
    "llx", "lly", "urx", "ury",
#define  K_DIM__RWI     12
#define  K_DIM__RHI     13
    "rwi", "rhi",
    NULL
  };
  double xoffset, yoffset, xscale, yscale, rotate;
  int    error  = 0;

  xoffset = yoffset = rotate = 0.0; xscale = yscale = 1.0;

  skip_blank(&ap->curptr, ap->endptr);
  while (!error && ap->curptr < ap->endptr) {
    char  *kp, *vp;
    int    k;

    kp = parse_c_ident(&ap->curptr, ap->endptr);
    if (!kp)
      break;

    for (k = 0; _dtkeys[k] && strcmp(kp, _dtkeys[k]); k++);
    if (!_dtkeys[k]) {
      spc_warn(spe, "Unrecognized dimension/transformation key: %s", kp);
      error = -1;
      RELEASE(kp);
      break;
    }

    skip_blank(&ap->curptr, ap->endptr);
    if (k == K__CLIP) {
      t->flags |= INFO_DO_CLIP;
      RELEASE(kp);
      continue; /* not key-value */
    }

    if (ap->curptr < ap->endptr && ap->curptr[0] == '=') {
      ap->curptr++;
      skip_blank(&ap->curptr, ap->endptr);
    }

    vp = NULL;
    if (ap->curptr[0] == '\'' || ap->curptr[0] == '\"') {
      char  qchr = ap->curptr[0];
      ap->curptr++;
      skip_blank(&ap->curptr, ap->endptr);
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      skip_blank(&ap->curptr, ap->endptr);
      if (vp && qchr != ap->curptr[0]) {
        spc_warn(spe, "Syntax error in dimension/transformation specification.");
        error = -1;
        RELEASE(vp); vp = NULL;
      }
      ap->curptr++;
    } else {
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
    }
    if (!error && !vp) {
      spc_warn(spe, "Missing value for dimension/transformation: %s", kp);
      error = -1;
    }
    RELEASE(kp);
    if (!vp || error) {
      break;
    }

    switch (k) {
    case  K_TRN__HOFFSET:
      xoffset = atof(vp);
      break;
    case  K_TRN__VOFFSET:
      yoffset = atof(vp);
      break;
    case  K_DIM__HSIZE:
      t->width   = atof(vp);
      t->flags  |= INFO_HAS_WIDTH;
      break;
    case  K_DIM__VSIZE:
      t->height  = atof(vp);
      t->flags  |= INFO_HAS_HEIGHT;
      break;
    case  K_TRN__HSCALE:
      xscale  = atof(vp) / 100.0;
      break;
    case  K_TRN__VSCALE:
      yscale  = atof(vp) / 100.0;
      break;
    case  K_TRN__ANGLE:
      rotate  = M_PI * atof(vp) / 180.0;
      break;
    case  K_DIM__LLX:
      t->bbox.llx = atof(vp);
      t->flags   |= INFO_HAS_USER_BBOX;
      break;
    case  K_DIM__LLY:
      t->bbox.lly = atof(vp);
      t->flags   |= INFO_HAS_USER_BBOX;
      break;
    case  K_DIM__URX:
      t->bbox.urx = atof(vp);
      t->flags   |= INFO_HAS_USER_BBOX;
      break;
    case  K_DIM__URY:
      t->bbox.ury = atof(vp);
      t->flags   |= INFO_HAS_USER_BBOX;
      break;
    case  K_DIM__RWI:
      t->width  = atof(vp) / 10.0;
      t->flags |= INFO_HAS_WIDTH;
      break;
    case  K_DIM__RHI:
      t->height = atof(vp) / 10.0;
      t->flags |= INFO_HAS_HEIGHT;
      break;
    }
    skip_blank(&ap->curptr, ap->endptr);
    RELEASE(vp);
  }
  make_transmatrix(&(t->matrix), xoffset, yoffset, xscale, yscale, rotate);

  return  error;
}

/* "page" and "pagebox" are not dimension nor transformation nor
 * something acceptable to put into here.
 * PLEASE DONT ADD HERE!
 */
static int
spc_read_dimtrns_pdfm (struct spc_env *spe,
                       transform_info *p, struct spc_arg *ap)
{
  int     has_scale, has_xscale, has_yscale, has_rotate, has_matrix;
  const char *_dtkeys[] = {
#define  K_DIM__WIDTH  0
#define  K_DIM__HEIGHT 1
#define  K_DIM__DEPTH  2
    "width", "height", "depth",
#define  K_TRN__SCALE  3
#define  K_TRN__XSCALE 4
#define  K_TRN__YSCALE 5
#define  K_TRN__ROTATE 6
    "scale", "xscale", "yscale", "rotate", /* See "Dvipdfmx User's Manual", p.5 */
#define  K_TRN__BBOX   7
    "bbox", /* See "Dvipdfmx User's Manual", p.5 */
#define  K_TRN__MATRIX 8
    "matrix",
#undef  K__CLIP
#define  K__CLIP       9
    "clip",
#define  K__HIDE       10
    "hide",
     NULL
  };
  double xscale, yscale, rotate;
  int    error = 0;

  has_xscale = has_yscale = has_scale = has_rotate = has_matrix = 0;
  xscale = yscale = 1.0; rotate = 0.0;
  p->flags |= INFO_DO_CLIP;   /* default: do clipping */
  p->flags &= ~INFO_DO_HIDE;   /* default: do clipping */

  skip_blank(&ap->curptr, ap->endptr);

  while (!error && ap->curptr < ap->endptr) {
    char  *kp, *vp;
    int    k;

    kp = parse_c_ident(&ap->curptr, ap->endptr);
    if (!kp)
      break;

    skip_blank(&ap->curptr, ap->endptr);
    for (k = 0; _dtkeys[k] && strcmp(_dtkeys[k], kp); k++);
    switch (k) {
    case  K_DIM__WIDTH:
      error = spc_util_read_length(spe, &p->width , ap);
      p->flags |= INFO_HAS_WIDTH;
      break;
    case  K_DIM__HEIGHT:
      error = spc_util_read_length(spe, &p->height, ap);
      p->flags |= INFO_HAS_HEIGHT;
      break;
    case  K_DIM__DEPTH:
      error = spc_util_read_length(spe, &p->depth , ap);
      p->flags |= INFO_HAS_HEIGHT;
      break;
    case  K_TRN__SCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        xscale = yscale = atof(vp);
        has_scale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__XSCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        xscale  = atof(vp);
        has_xscale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__YSCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        yscale  = atof(vp);
        has_yscale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__ROTATE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        rotate = M_PI * atof(vp) / 180.0;
        has_rotate = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__BBOX:
      {
        double  v[4];
        if (spc_util_read_numbers(v, 4, ap) != 4)
          error = -1;
        else {
          p->bbox.llx = v[0];
          p->bbox.lly = v[1];
          p->bbox.urx = v[2];
          p->bbox.ury = v[3];
          p->flags   |= INFO_HAS_USER_BBOX;
        }
      }
      break;
    case  K_TRN__MATRIX:
      {
        double  v[6];
        if (spc_util_read_numbers(v, 6, ap) != 6)
          error = -1;
        else {
          pdf_setmatrix(&(p->matrix), v[0], v[1], v[2], v[3], v[4], v[5]);
          has_matrix = 1;
        }
      }
      break;
    case  K__CLIP:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        if (atof(vp))
          p->flags |= INFO_DO_CLIP;
        else
          p->flags &= ~INFO_DO_CLIP;
        RELEASE(vp);
      }
      break;
    case  K__HIDE:
      p->flags |= INFO_DO_HIDE;
      break;

    default:
      error = -1;
      break;
    }
    if (error)
      spc_warn(spe, "Unrecognized key or invalid value for " \
                     "dimension/transformation: %s", kp);
    else
      skip_blank(&ap->curptr, ap->endptr);
    RELEASE(kp);
  }

  if (!error) {
    /* Check consistency */
    if (has_xscale && (p->flags & INFO_HAS_WIDTH)) {
      spc_warn(spe, "Can't supply both width and xscale. Ignore xscale.");
      xscale = 1.0;
    } else if (has_yscale &&
               (p->flags & INFO_HAS_HEIGHT)) {
      spc_warn(spe, "Can't supply both height/depth and yscale. Ignore yscale.");
      yscale = 1.0;
    } else if (has_scale &&
               (has_xscale || has_yscale)) {
      spc_warn(spe, "Can't supply overall scale along with axis scales.");
      error = -1;
    } else if (has_matrix &&
               (has_scale || has_xscale || has_yscale || has_rotate)) {
      spc_warn(spe, "Can't supply transform matrix along with scales or rotate. Ignore scales and rotate.");
    }
  }

  if (!has_matrix) {
    make_transmatrix(&(p->matrix), 0.0, 0.0, xscale, yscale, rotate);
  }

  if (!(p->flags & INFO_HAS_USER_BBOX)) {
    p->flags &= ~INFO_DO_CLIP;    /* no clipping needed */
  }

  return  error;
}

int
spc_util_read_dimtrns (struct spc_env *spe,
                       transform_info *ti, struct spc_arg *args, int syntax)
{
  if (!ti || !spe || !args)
    return -1;

  if (syntax) {
    return  spc_read_dimtrns_dvips(spe, ti, args);
  } else {
    return  spc_read_dimtrns_pdfm (spe, ti, args);
  }
}

int
spc_util_read_blahblah (struct spc_env *spe,
                        transform_info *p, int *page_no, int *bbox_type,
                        struct spc_arg *ap)
{
  int     has_scale, has_xscale, has_yscale, has_rotate, has_matrix;
  const char *_dtkeys[] = {
    "width", "height", "depth",
    "scale", "xscale", "yscale", "rotate",
    "bbox",
    "matrix",
    "clip",
    "hide",
#define  K__PAGE       11
    "page",
#define  K__PAGEBOX    12
    "pagebox",
     NULL
  };
  double xscale, yscale, rotate;
  int    error = 0;

  has_xscale = has_yscale = has_scale = has_rotate = has_matrix = 0;
  xscale = yscale = 1.0; rotate = 0.0;
  p->flags |= INFO_DO_CLIP;   /* default: do clipping */
  p->flags &= ~INFO_DO_HIDE;   /* default: do clipping */

  skip_blank(&ap->curptr, ap->endptr);

  while (!error && ap->curptr < ap->endptr) {
    char  *kp, *vp;
    int    k;

    kp = parse_c_ident(&ap->curptr, ap->endptr);
    if (!kp)
      break;

    skip_blank(&ap->curptr, ap->endptr);
    for (k = 0; _dtkeys[k] && strcmp(_dtkeys[k], kp); k++);
    switch (k) {
    case  K_DIM__WIDTH:
      error = spc_util_read_length(spe, &p->width , ap);
      p->flags |= INFO_HAS_WIDTH;
      break;
    case  K_DIM__HEIGHT:
      error = spc_util_read_length(spe, &p->height, ap);
      p->flags |= INFO_HAS_HEIGHT;
      break;
    case  K_DIM__DEPTH:
      error = spc_util_read_length(spe, &p->depth , ap);
      p->flags |= INFO_HAS_HEIGHT;
      break;
    case  K_TRN__SCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        xscale = yscale = atof(vp);
        has_scale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__XSCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        xscale  = atof(vp);
        has_xscale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__YSCALE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        yscale  = atof(vp);
        has_yscale = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__ROTATE:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        rotate = M_PI * atof(vp) / 180.0;
        has_rotate = 1;
        RELEASE(vp);
      }
      break;
    case  K_TRN__BBOX:
      {
        double  v[4];
        if (spc_util_read_numbers(v, 4, ap) != 4)
          error = -1;
        else {
          p->bbox.llx = v[0];
          p->bbox.lly = v[1];
          p->bbox.urx = v[2];
          p->bbox.ury = v[3];
          p->flags   |= INFO_HAS_USER_BBOX;
        }
      }
      break;
    case  K_TRN__MATRIX:
      {
        double  v[6];
        if (spc_util_read_numbers(v, 6, ap) != 6)
          error = -1;
        else {
          pdf_setmatrix(&(p->matrix), v[0], v[1], v[2], v[3], v[4], v[5]);
          has_matrix = 1;
        }
      }
      break;
    case  K__CLIP:
      vp = parse_float_decimal(&ap->curptr, ap->endptr);
      if (!vp)
        error = -1;
      else {
        if (atof(vp))
          p->flags |= INFO_DO_CLIP;
        else
          p->flags &= ~INFO_DO_CLIP;
        RELEASE(vp);
      }
      break;

    case  K__PAGE:
      {
        double page;
        if (page_no && spc_util_read_numbers(&page, 1, ap) == 1)
          *page_no = (int) page;
        else
          error = -1;
      }
      break;
    case  K__HIDE:
      p->flags |= INFO_DO_HIDE;
      break;
    case  K__PAGEBOX:
      {
        char *q;
        q = parse_c_ident (&ap->curptr, ap->endptr);
        if (q) {
          if (bbox_type) {
            if (strcasecmp(q, "cropbox") == 0)       *bbox_type = 1;
            else if (strcasecmp(q, "mediabox") == 0) *bbox_type = 2;
            else if (strcasecmp(q, "artbox") == 0)   *bbox_type = 3;
            else if (strcasecmp(q, "trimbox") == 0)  *bbox_type = 4;
            else if (strcasecmp(q, "bleedbox") == 0) *bbox_type = 5;
          }
          RELEASE(q);
        } else if (bbox_type) {
          *bbox_type = 0;
        }
      }
      break;

    default:
      error = -1;
      break;
    }
    if (error)
      spc_warn(spe, "Unrecognized key or invalid value for " \
                     "dimension/transformation: %s", kp);
    else
      skip_blank(&ap->curptr, ap->endptr);
    RELEASE(kp);
  }

  if (!error) {
    /* Check consistency */
    if (has_xscale && (p->flags & INFO_HAS_WIDTH)) {
      spc_warn(spe, "Can't supply both width and xscale. Ignore xscale.");
      xscale = 1.0;
    } else if (has_yscale &&
               (p->flags & INFO_HAS_HEIGHT)) {
      spc_warn(spe, "Can't supply both height/depth and yscale. Ignore yscale.");
      yscale = 1.0;
    } else if (has_scale &&
               (has_xscale || has_yscale)) {
      spc_warn(spe, "Can't supply overall scale along with axis scales.");
      error = -1;
    } else if (has_matrix &&
               (has_scale || has_xscale || has_yscale || has_rotate)) {
      spc_warn(spe, "Can't supply transform matrix along with scales or rotate. Ignore scales and rotate.");
    }
  }

  if (!has_matrix) {
    make_transmatrix(&(p->matrix), 0.0, 0.0, xscale, yscale, rotate);
  }

  if (!(p->flags & INFO_HAS_USER_BBOX)) {
    p->flags &= ~INFO_DO_CLIP;    /* no clipping needed */
  }

  return  error;
}

/* Color names */
#ifdef  rgb
#undef  rgb
#endif
#ifdef  cmyk
#undef  cmyk
#endif
#define gray(g)       {1, NULL, {g}}
#define rgb8(r,g,b)   {3, NULL, {((r)/255.0), ((g)/255.0), ((b)/255.0), 0.0}}
#define cmyk(c,m,y,k) {4, NULL, {(c), (m), (y), (k)}}

static struct colordef_
{
  const char  *key;
  pdf_color    color;
} colordefs[] = {
  {"GreenYellow",    cmyk(0.15, 0.00, 0.69, 0.00)},
  {"Yellow",         cmyk(0.00, 0.00, 1.00, 0.00)},
  {"Goldenrod",      cmyk(0.00, 0.10, 0.84, 0.00)},
  {"Dandelion",      cmyk(0.00, 0.29, 0.84, 0.00)},
  {"Apricot",        cmyk(0.00, 0.32, 0.52, 0.00)},
  {"Peach",          cmyk(0.00, 0.50, 0.70, 0.00)},
  {"Melon",          cmyk(0.00, 0.46, 0.50, 0.00)},
  {"YellowOrange",   cmyk(0.00, 0.42, 1.00, 0.00)},
  {"Orange",         cmyk(0.00, 0.61, 0.87, 0.00)},
  {"BurntOrange",    cmyk(0.00, 0.51, 1.00, 0.00)},
  {"Bittersweet",    cmyk(0.00, 0.75, 1.00, 0.24)},
  {"RedOrange",      cmyk(0.00, 0.77, 0.87, 0.00)},
  {"Mahogany",       cmyk(0.00, 0.85, 0.87, 0.35)},
  {"Maroon",         cmyk(0.00, 0.87, 0.68, 0.32)},
  {"BrickRed",       cmyk(0.00, 0.89, 0.94, 0.28)},
  {"Red",            cmyk(0.00, 1.00, 1.00, 0.00)},
  {"OrangeRed",      cmyk(0.00, 1.00, 0.50, 0.00)},
  {"RubineRed",      cmyk(0.00, 1.00, 0.13, 0.00)},
  {"WildStrawberry", cmyk(0.00, 0.96, 0.39, 0.00)},
  {"Salmon",         cmyk(0.00, 0.53, 0.38, 0.00)},
  {"CarnationPink",  cmyk(0.00, 0.63, 0.00, 0.00)},
  {"Magenta",        cmyk(0.00, 1.00, 0.00, 0.00)},
  {"VioletRed",      cmyk(0.00, 0.81, 0.00, 0.00)},
  {"Rhodamine",      cmyk(0.00, 0.82, 0.00, 0.00)},
  {"Mulberry",       cmyk(0.34, 0.90, 0.00, 0.02)},
  {"RedViolet",      cmyk(0.07, 0.90, 0.00, 0.34)},
  {"Fuchsia",        cmyk(0.47, 0.91, 0.00, 0.08)},
  {"Lavender",       cmyk(0.00, 0.48, 0.00, 0.00)},
  {"Thistle",        cmyk(0.12, 0.59, 0.00, 0.00)},
  {"Orchid",         cmyk(0.32, 0.64, 0.00, 0.00)},
  {"DarkOrchid",     cmyk(0.40, 0.80, 0.20, 0.00)},
  {"Purple",         cmyk(0.45, 0.86, 0.00, 0.00)},
  {"Plum",           cmyk(0.50, 1.00, 0.00, 0.00)},
  {"Violet",         cmyk(0.79, 0.88, 0.00, 0.00)},
  {"RoyalPurple",    cmyk(0.75, 0.90, 0.00, 0.00)},
  {"BlueViolet",     cmyk(0.86, 0.91, 0.00, 0.04)},
  {"Periwinkle",     cmyk(0.57, 0.55, 0.00, 0.00)},
  {"CadetBlue",      cmyk(0.62, 0.57, 0.23, 0.00)},
  {"CornflowerBlue", cmyk(0.65, 0.13, 0.00, 0.00)},
  {"MidnightBlue",   cmyk(0.98, 0.13, 0.00, 0.43)},
  {"NavyBlue",       cmyk(0.94, 0.54, 0.00, 0.00)},
  {"RoyalBlue",      cmyk(1.00, 0.50, 0.00, 0.00)},
  {"Blue",           cmyk(1.00, 1.00, 0.00, 0.00)},
  {"Cerulean",       cmyk(0.94, 0.11, 0.00, 0.00)},
  {"Cyan",           cmyk(1.00, 0.00, 0.00, 0.00)},
  {"ProcessBlue",    cmyk(0.96, 0.00, 0.00, 0.00)},
  {"SkyBlue",        cmyk(0.62, 0.00, 0.12, 0.00)},
  {"Turquoise",      cmyk(0.85, 0.00, 0.20, 0.00)},
  {"TealBlue",       cmyk(0.86, 0.00, 0.34, 0.02)},
  {"Aquamarine",     cmyk(0.82, 0.00, 0.30, 0.00)},
  {"BlueGreen",      cmyk(0.85, 0.00, 0.33, 0.00)},
  {"Emerald",        cmyk(1.00, 0.00, 0.50, 0.00)},
  {"JungleGreen",    cmyk(0.99, 0.00, 0.52, 0.00)},
  {"SeaGreen",       cmyk(0.69, 0.00, 0.50, 0.00)},
  {"Green",          cmyk(1.00, 0.00, 1.00, 0.00)},
  {"ForestGreen",    cmyk(0.91, 0.00, 0.88, 0.12)},
  {"PineGreen",      cmyk(0.92, 0.00, 0.59, 0.25)},
  {"LimeGreen",      cmyk(0.50, 0.00, 1.00, 0.00)},
  {"YellowGreen",    cmyk(0.44, 0.00, 0.74, 0.00)},
  {"SpringGreen",    cmyk(0.26, 0.00, 0.76, 0.00)},
  {"OliveGreen",     cmyk(0.64, 0.00, 0.95, 0.40)},
  {"RawSienna",      cmyk(0.00, 0.72, 1.00, 0.45)},
  {"Sepia",          cmyk(0.00, 0.83, 1.00, 0.70)},
  {"Brown",          cmyk(0.00, 0.81, 1.00, 0.60)},
  {"Tan",            cmyk(0.14, 0.42, 0.56, 0.00)},
  /* Adobe Reader 7 and 8 had problem when gray and cmyk black colors
   * are mixed. No problem with Previewer.app.
   * It happens when \usepackage[dvipdfm]{graphicx} and then called
   * \usepackage{color} without dvipdfm option. */
  {"Gray",           gray(0.5)},
  {"Black",          gray(0.0)},
  {"White",          gray(1.0)},
  {NULL}
};


static int
pdf_color_namedcolor (pdf_color *color, const char *name)
{
  int   i;
  for (i = 0; colordefs[i].key; i++) {
    if (!strcmp(colordefs[i].key, name)) {
      pdf_color_copycolor(color, &colordefs[i].color);
      return  0;
    }
  }
  return  -1;
}
