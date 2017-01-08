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

#include <math.h>

#include "system.h"
#include "error.h"
#include "mem.h"
#include "mfileio.h"
#include "dpxutil.h"
#include "numbers.h"

#include "pdfdoc.h"
#include "pdfdev.h"
#include "pdfcolor.h"

#include "pdfdraw.h"


/*
 * Numbers are rounding to 0-5 fractional digits
 * in output routine. 
 */
#define detM(M) ((M).a * (M).d - (M).b * (M).c)
#define detP(M) ((M)->a * (M)->d - (M)->b * (M)->c)


static /* __inline__ */ int
inversematrix (pdf_tmatrix *W, const pdf_tmatrix *M)
{
  double  det;

  det = detP(M);
  if (fabs(det) < 1.e-8) {
    WARN("Inverting matrix with zero determinant...");
    return -1; /* result is undefined. */
  }

  W->a =  (M->d) / det;  W->b = -(M->b) / det;
  W->c = -(M->c) / det;  W->d =  (M->a) / det;
  W->e =  (M->c) * (M->f) - (M->d) * (M->e);
  W->f =  (M->b) * (M->e) - (M->a) * (M->f);

  return 0;
}

/* pdf_coord as vector */
#define vecprd(v,w) ((v).x * (w).x + (v).y * (w).y)
#define vecrot(v,w) ((v).x * (w).y - (v).y * (w).x)
#define dsign(v)    (((v) >= 0.0) ? 1.0 : -1.0)
/* acos => [0, pi] */
#define vecang(v,w) ( \
  dsign(vecrot((v),(w))) * \
    acos(vecprd((v),(w)) / sqrt(vecprd((v),(v)) * vecprd((w),(w)))) \
)

static /* __inline__ */ int
pdf_coord__equal (const pdf_coord *p1, const pdf_coord *p2)
{
  if (fabs(p1->x - p2->x) < 1.e-7 &&
      fabs(p1->y - p2->y) < 1.e-7)
    return 1;
  return 0;
}
#define COORD_EQUAL(p,q) pdf_coord__equal((p),(q))

#if 0
static int
pdf_coord__sort_compar_X (const void *pp1, const void *pp2)
{
  pdf_coord *p1 = (pdf_coord *)pp1;
  pdf_coord *p2 = (pdf_coord *)pp2;

  if (pdf_coord__equal(p1, p2))
    return 0;
  else
    return (int) dsign(p1->x - p2->x);
 
  return 1;
}

static int
pdf_coord__sort_compar_Y (const void *pp1, const void *pp2)
{
  pdf_coord *p1 = (pdf_coord *)pp1;
  pdf_coord *p2 = (pdf_coord *)pp2;

  if (pdf_coord__equal(p1, p2))
    return 0;
  else
    return (int) dsign(p1->y - p2->y);
 
  return 1;
}
#endif


static /* __inline__ */ int
pdf_coord__transform (pdf_coord *p, const pdf_tmatrix *M)
{
  double x, y;

  x = p->x; y = p->y;
  p->x = x * M->a + y * M->c + M->e;
  p->y = x * M->b + y * M->d + M->f;

  return 0;
}

#if 0
static /* __inline__ */ int
pdf_coord__itransform (pdf_coord *p, const pdf_tmatrix *M)
{
  pdf_tmatrix W;  
  double      x, y;
  int         error;

  error = inversematrix(&W, M);
  if (error) 
    return error;

  x = p->x;  y = p->y;
  p->x = x * W.a + y * W.c + W.e;
  p->y = x * W.b + y * W.d + W.f;  

  return 0;
}
#endif

static /* __inline__ */ int
pdf_coord__dtransform (pdf_coord *p, const pdf_tmatrix *M)
{
  double x, y;

  x = p->x; y = p->y;
  p->x = x * M->a + y * M->c;
  p->y = x * M->b + y * M->d;

  return 0;
}

static /* __inline__ */ int
pdf_coord__idtransform (pdf_coord *p, const pdf_tmatrix *M)
{
  pdf_tmatrix W;  
  double      x, y;
  int         error;

  error = inversematrix(&W, M);
  if (error) 
    return error;

  x = p->x;  y = p->y;
  p->x = x * W.a + y * W.c;
  p->y = x * W.b + y * W.d;

  return 0;
}


/* Modify M itself */
void
pdf_invertmatrix (pdf_tmatrix *M)
{
  pdf_tmatrix W;  
  double      det;

  ASSERT(M);

  det = detP(M);
  if (fabs(det) < 1.e-8) {
    WARN("Inverting matrix with zero determinant...");
    W.a = 1.0; W.c = 0.0;
    W.b = 0.0; W.d = 1.0;
    W.e = 0.0; W.f = 0.0;
  } else {
    W.a =  (M->d) / det;  W.b = -(M->b) / det;
    W.c = -(M->c) / det;  W.d =  (M->a) / det;
    W.e =  (M->c) * (M->f) - (M->d) * (M->e);
    W.f =  (M->b) * (M->e) - (M->a) * (M->f);
    W.e /= det; W.f /= det;
  }

  pdf_copymatrix(M, &W);

  return;
}


typedef struct pa_elem_
{
  int       type;
  pdf_coord p[3];
} pa_elem;

/* each subpath delimitted by moveto */
struct pdf_path_
{
  int       num_paths;
  int       max_paths;
  pa_elem  *path;
};

static const struct {
  char        opchr;  /* PDF operator char  */
  int         n_pts;  /* number of *points* */
  const char *strkey;
} petypes[] = {
#define PE_TYPE__INVALID  -1
#define PE_TYPE__MOVETO    0
  {'m', 1, "moveto"  },
#define PE_TYPE__LINETO    1 
  {'l', 1, "lineto"  },
#define PE_TYPE__CURVETO   2 
  {'c', 3, "curveto" },
  /* no PS correspondence for v and y */
#define PE_TYPE__CURVETO_V 3 
  {'v', 2, "vcurveto"}, /* current point replicated */
#define PE_TYPE__CURVETO_Y 4 
  {'y', 2, "ycurveto"}, /* last point replicated */
#define PE_TYPE__CLOSEPATH 5
  {'h', 0, "closepath"}, 
#define PE_TYPE__TERMINATE 6
  {' ', 0,  NULL}
};

#define PE_VALID(p) ((p) && \
  (p)->type > PE_TYPE__INVALID && (p)->type < PE_TYPE__TERMINATE)
#define PE_N_PTS(p)  (PE_VALID((p)) ? petypes[(p)->type].n_pts : 0)
#define PE_OPCHR(p)  (PE_VALID((p)) ? petypes[(p)->type].opchr : ' ')

#define PA_LENGTH(pa) ((pa)->num_paths)

#define GS_FLAG_CURRENTPOINT_SET (1 << 0)


#define FORMAT_BUFF_LEN 1024
static char    fmt_buf[FORMAT_BUFF_LEN];

static void
init_a_path (pdf_path *p)
{
  ASSERT(p);

  p->num_paths = 0;
  p->max_paths = 0;
  p->path      = NULL;

  return;
}

static void
pdf_path__clearpath (pdf_path *p)
{
  ASSERT(p);

  p->num_paths = 0;
  
  return;
}

static int
pdf_path__growpath  (pdf_path *p, int max_pe)
{
  if (max_pe < p->max_paths)
    return 0;

  p->max_paths = MAX(p->max_paths + 8, max_pe);
  p->path = RENEW(p->path, p->max_paths, pa_elem);

  return 0;
}
    
static void
clear_a_path (pdf_path *p)
{
  ASSERT(p);

  if (p->path)
    RELEASE(p->path);
  p->path = NULL;
  p->num_paths = 0;
  p->max_paths = 0;

  return;
}

static int
pdf_path__copypath (pdf_path *p1, const pdf_path *p0)
{
  pa_elem  *pe0, *pe1;
  int       i;

  pdf_path__growpath(p1, PA_LENGTH(p0));
  for (i = 0; i < PA_LENGTH(p0); i++) {
    pe1 = &(p1->path[i]);
    pe0 = &(p0->path[i]);
    /* FIXME */
    pe1->type   = pe0->type;
    pe1->p[0].x = pe0->p[0].x;
    pe1->p[0].y = pe0->p[0].y;
    pe1->p[1].x = pe0->p[1].x;
    pe1->p[1].y = pe0->p[1].y;
    pe1->p[2].x = pe0->p[2].x;
    pe1->p[2].y = pe0->p[2].y;
  }
  p1->num_paths = PA_LENGTH(p0);

  return 0;
}


/* start new subpath */
static int
pdf_path__moveto  (pdf_path        *pa,
                   pdf_coord       *cp,
                   const pdf_coord *p0)
{
  pa_elem  *pe;
  
  pdf_path__growpath(pa, PA_LENGTH(pa) + 1);
  if (PA_LENGTH(pa) > 0) {
    pe = &pa->path[pa->num_paths-1];
    if (pe->type == PE_TYPE__MOVETO) {
      pe->p[0].x = cp->x = p0->x;
      pe->p[0].y = cp->y = p0->y;
      return 0;
    }
  }
  pe = &pa->path[pa->num_paths++];
  pe->type   = PE_TYPE__MOVETO;
  pe->p[0].x = cp->x = p0->x;
  pe->p[0].y = cp->y = p0->y;

  return 0;  
}

/* Do 'compression' of path while adding new path elements.
 * Sequantial moveto command will be replaced with a
 * single moveto. If cp is not equal to the last point in pa,
 * then moveto is inserted (starting new subpath).
 * FIXME: 
 * 'moveto' must be used to enforce starting new path.
 * This affects how 'closepath' is treated.     
 */
static pa_elem *
pdf_path__next_pe (pdf_path *pa, const pdf_coord *cp)
{
  pa_elem  *pe;

  pdf_path__growpath(pa, PA_LENGTH(pa) + 2);
  if (PA_LENGTH(pa) == 0) {
    pe = &pa->path[pa->num_paths++];
    pe->type   = PE_TYPE__MOVETO;
    pe->p[0].x = cp->x;
    pe->p[0].y = cp->y;

    return &pa->path[pa->num_paths++];
  }
    
  pe = &pa->path[pa->num_paths-1];
  switch (pe->type) {
  case PE_TYPE__MOVETO:
    pe->p[0].x = cp->x;
    pe->p[0].y = cp->y;
    break;
  case PE_TYPE__LINETO:
    if (!COORD_EQUAL(&pe->p[0], cp)) {
      pe = &pa->path[pa->num_paths++];
      pe->type   = PE_TYPE__MOVETO;
      pe->p[0].x = cp->x;
      pe->p[0].y = cp->y;
    }
    break;
  case PE_TYPE__CURVETO:
    if (!COORD_EQUAL(&pe->p[2], cp)) {
      pe = &pa->path[pa->num_paths++];
      pe->type   = PE_TYPE__MOVETO;
      pe->p[0].x = cp->x;
      pe->p[0].y = cp->y;
    }
    break;
  case PE_TYPE__CURVETO_Y:
  case PE_TYPE__CURVETO_V:
    if (!COORD_EQUAL(&pe->p[1], cp)) {
      pe = &pa->path[pa->num_paths++];
      pe->type   = PE_TYPE__MOVETO;
      pe->p[0].x = cp->x;
      pe->p[0].y = cp->y;
    }
    break;
  case PE_TYPE__CLOSEPATH:
    pe = &pa->path[pa->num_paths++];
    pe->type   = PE_TYPE__MOVETO;
    pe->p[0].x = cp->x;
    pe->p[0].y = cp->y;
    break;
  }

  return &pa->path[pa->num_paths++];
}

static int
pdf_path__transform (pdf_path *pa, const pdf_tmatrix *M)
{
  pa_elem *pe;
  int      n = 0, i;

  ASSERT(pa && M);

  for (i = 0; i < PA_LENGTH(pa); i++) {
    pe = &(pa->path[i]);
    n  = PE_N_PTS(pe);
    while (n-- > 0)
      pdf_coord__transform(&(pe->p[n]), M);
  }

  return 0;
}


/* Path Construction */
static int
pdf_path__lineto (pdf_path        *pa,
                  pdf_coord       *cp,
                  const pdf_coord *p0)
{
  pa_elem  *pe;

  pe = pdf_path__next_pe(pa, cp);
  pe->type   = PE_TYPE__LINETO;
  pe->p[0].x = cp->x = p0->x;
  pe->p[0].y = cp->y = p0->y;

  return 0;
}

static int
pdf_path__curveto (pdf_path        *pa,
                   pdf_coord       *cp,
                   const pdf_coord *p0,
                   const pdf_coord *p1,
                   const pdf_coord *p2
                  )
{
  pa_elem *pe;

  pe = pdf_path__next_pe(pa, cp);
  if (COORD_EQUAL(cp, p0)) {
    pe->type   = PE_TYPE__CURVETO_V;
    pe->p[0].x = p1->x;
    pe->p[0].y = p1->y;
    pe->p[1].x = cp->x = p2->x;
    pe->p[1].y = cp->y = p2->y;
  } else if (COORD_EQUAL(p1, p2)) {
    pe->type   = PE_TYPE__CURVETO_Y;
    pe->p[0].x = p0->x;
    pe->p[0].y = p0->y;
    pe->p[1].x = cp->x = p1->x;
    pe->p[1].y = cp->y = p1->y;
  } else {
    pe->type   = PE_TYPE__CURVETO;
    pe->p[0].x = p0->x;
    pe->p[0].y = p0->y;
    pe->p[1].x = p1->x;
    pe->p[1].y = p1->y;
    pe->p[2].x = cp->x = p2->x;
    pe->p[2].y = cp->y = p2->y;
  }

  return 0;
}

#if 0
#define QB_TWO_THIRD (2.0/3.0)
#define QB_ONE_THIRD (1.0/3.0)

static int
pdf_path__curveto_QB (pdf_path        *pa,
                      pdf_coord       *cp,
                      const pdf_coord *p0,
                      const pdf_coord *p1
                     )
{
  pdf_coord  q0, q1;

  q0.x = cp->x + QB_TWO_THIRD * (p0->x - cp->x);
  q0.y = cp->y + QB_TWO_THIRD * (p0->y - cp->y);
  q1.x = p0->x + QB_ONE_THIRD * (p1->x - p0->x);
  q1.y = p0->y + QB_ONE_THIRD * (p1->y - p0->y);
  /* q2 == p1 */
 
  return pdf_path__curveto(pa, cp, &q0, &q1, p1);
}
#endif


/* This isn't specified as cp to somewhere. */
static int
pdf_path__elliptarc (pdf_path         *pa,
                     pdf_coord        *cp,
                     const pdf_coord  *ca,   /* ellipsis center        */
                     double            r_x,  /* x radius               */
                     double            r_y,  /* y radius               */
                     double            xar,  /* x-axis-rotation (deg!) */
                     double            a_0,  /* start angle            */
                     double            a_1,  /* stop angle             */
                     int               a_d   /* arc orientation        */
                    )
{
  double      b, b_x, b_y;
  double      d_a, q;
  pdf_coord   p0, p1, p2, p3;
  pdf_coord   e0, e1;
  pdf_tmatrix T;
  int         n_c; /* number of segments */
  int         i, error = 0;

  if (fabs(r_x) < 1.e-8 ||
      fabs(r_y) < 1.e-8)
    return -1;

  if (a_d < 0) {
    for ( ; a_1 > a_0; a_1 -= 360.0);
  } else {
    for ( ; a_1 < a_0; a_0 -= 360.0);
  }

  d_a  = a_1 - a_0;
  for (n_c = 1; fabs(d_a) > 90.0 * n_c; n_c++);
  d_a /= n_c;
  if (fabs(d_a) < 1.e-8)
    return -1;

  a_0 *= M_PI / 180.0;
  a_1 *= M_PI / 180.0;
  d_a *= M_PI / 180.0;
  xar *= M_PI / 180.0;
  T.a  = cos(xar);  T.c = -sin(xar);
  T.b  = -T.c    ;  T.d = T.a;
  T.e  = 0.0     ;  T.f = 0.0;

  /* A parameter that controls cb-curve (off-curve) points */
  b    = 4.0 * (1.0 - cos(.5 * d_a)) / (3.0 * sin(.5 * d_a));
  b_x  = r_x * b;
  b_y  = r_y * b;

  p0.x = r_x * cos(a_0);
  p0.y = r_y * sin(a_0);
  pdf_coord__transform(&p0, &T);
  p0.x += ca->x; p0.y += ca->y;
  if (PA_LENGTH(pa) == 0) {
    pdf_path__moveto(pa, cp, &p0);
  } else if (!COORD_EQUAL(cp, &p0)) {
    pdf_path__lineto(pa, cp, &p0); /* add line seg */
  } 
  for (i = 0; !error && i < n_c; i++) {
    q = a_0 + i * d_a;
    e0.x = cos(q); e0.y = sin(q);
    e1.x = cos(q + d_a); e1.y = sin(q + d_a);

   /* Condition for tangent vector requirs
    *  d1 = p1 - p0 = f ( sin a, -cos a)
    *  d2 = p2 - p3 = g ( sin b, -cos b)
    * and from symmetry
    *  g^2 = f^2
    */ 
    p0.x = r_x * e0.x; /* s.p. */
    p0.y = r_y * e0.y;
    p3.x = r_x * e1.x; /* e.p. */
    p3.y = r_y * e1.y;

    p1.x = -b_x * e0.y;
    p1.y =  b_y * e0.x;
    p2.x =  b_x * e1.y;
    p2.y = -b_y * e1.x;

    pdf_coord__transform(&p0, &T);
    pdf_coord__transform(&p1, &T);
    pdf_coord__transform(&p2, &T);
    pdf_coord__transform(&p3, &T);
    p0.x += ca->x; p0.y += ca->y;
    p3.x += ca->x; p3.y += ca->y;
    p1.x += p0.x ; p1.y += p0.y ;
    p2.x += p3.x ; p2.y += p3.y ;

    error = pdf_path__curveto(pa, &p0, &p1, &p2, &p3);
    cp->x = p3.x; cp->y = p3.y;
  }

  return error;
}

static int
pdf_path__closepath (pdf_path *pa, pdf_coord *cp /* no arg */)
{
  pa_elem  *pe = NULL;
  int       i;

  /* search for start point of the last subpath */
  for (i = PA_LENGTH(pa) - 1; i >= 0; i--) {
    pe = &pa->path[i];
    if (pe->type == PE_TYPE__MOVETO)
      break;
  }

  if (!pe || i < 0)
    return -1; /* No path or no start point(!) */

  cp->x = pe->p[0].x;
  cp->y = pe->p[0].y;

  pdf_path__growpath(pa, PA_LENGTH(pa) + 1);

  /* NOTE:
   *  Manually closed path without closepath is not
   *  affected by linejoin. A path with coincidental
   *  starting and ending point is not the same as
   *  'closed' path.
   */
  pe = &pa->path[pa->num_paths++];
  pe->type = PE_TYPE__CLOSEPATH;

  return 0;
}

/*
 *  x y width height re
 * 
 * is equivalent to
 *
 *  x y m
 *  (x + width) y l
 *  (x + width) (y + height) l
 *  x (y + height) l
 *  h
 */
/* Just for quick test */ 
static /* __inline__ */ int
pdf_path__isarect (pdf_path *pa,
                   int       f_ir /* fill-rule is ignorable */
                  )
{
  pa_elem *pe0, *pe1, *pe2, *pe3, *pe4;

  if (PA_LENGTH(pa) == 5) {
    pe0 = &(pa->path[0]);
    pe1 = &(pa->path[1]);
    pe2 = &(pa->path[2]);
    pe3 = &(pa->path[3]);
    pe4 = &(pa->path[4]);
    if (pe0->type == PE_TYPE__MOVETO &&
        pe1->type == PE_TYPE__LINETO &&
        pe2->type == PE_TYPE__LINETO &&
        pe3->type == PE_TYPE__LINETO &&
        pe4->type == PE_TYPE__CLOSEPATH) {
      if (pe1->p[0].y - pe0->p[0].y == 0 &&
          pe2->p[0].x - pe1->p[0].x == 0 &&
          pe3->p[0].y - pe2->p[0].y == 0) {
        if (pe1->p[0].x - pe0->p[0].x
            == pe2->p[0].x - pe3->p[0].x) {
          return 1;
        }
    /* Winding number is different but ignore it here. */
      } else if (f_ir &&
                 pe1->p[0].x - pe0->p[0].x == 0 &&
                 pe2->p[0].y - pe1->p[0].y == 0 &&
                 pe3->p[0].x - pe2->p[0].x == 0) {
        if (pe1->p[0].y - pe0->p[0].y
              == pe2->p[0].y - pe3->p[0].y) {
          return 1;
        }
      }
    }
  }

  return 0;
}

/* Path Painting */
/* F is obsoleted */
#define PT_OP_VALID(c) ( \
 (c) == 'f' || (c) == 'F' || \
 (c) == 's' || (c) == 'S' || \
 (c) == 'b' || (c) == 'B' || \
 (c) == 'W' || (c) == ' ' \
)

static /* __inline__ */ int
INVERTIBLE_MATRIX (const pdf_tmatrix *M)
{
  if (fabs(detP(M)) < 1.e-8) {
    WARN("Transformation matrix not invertible.");
    WARN("--- M = [%g %g %g %g %g %g]",
         M->a, M->b, M->c, M->d, M->e, M->f);
    return -1;
  }
  return 0;
}

/* rectfill, rectstroke, rectclip, recteoclip
 *
 * Draw isolated rectangle without actually doing
 * gsave/grestore operation.
 * 
 * TODO:
 *  linestyle, fill-opacity, stroke-opacity,....
 *  As this routine draw a single graphics object
 *  each time, there should be options for specifying
 *  various drawing styles, which might inherite
 *  current graphcs state parameter.
 */ 
static int
pdf_dev__rectshape (const pdf_rect    *r,
                    const pdf_tmatrix *M,
                    char               opchr
                   )
{
  char     *buf = fmt_buf;
  int       len = 0;
  int       isclip = 0;
  pdf_coord p;
  double    wd, ht;

  ASSERT(r && PT_OP_VALID(opchr));

  isclip = (opchr == 'W' || opchr == ' ') ? 1 : 0;

  /* disallow matrix for clipping.
   * q ... clip Q does nothing and
   * n M cm ... clip n alter CTM.
   */
  if (M && (isclip ||
            !INVERTIBLE_MATRIX(M)))
    return -1;

  graphics_mode();

  buf[len++] = ' ';
  if (!isclip) {
    buf[len++] = 'q';
    if (M) {
      buf[len++] = ' ';
      len += pdf_sprint_matrix(buf + len, M);
      buf[len++] = ' ';
      buf[len++] = 'c'; buf[len++] = 'm';
    }
    buf[len++] = ' ';
  }
  buf[len++] = 'n';

  p.x = r->llx; p.y = r->lly;
  wd  = r->urx - r->llx;
  ht  = r->ury - r->lly;
  buf[len++] = ' ';
  len += pdf_sprint_coord (buf + len, &p);
  buf[len++] = ' ';
  len += pdf_sprint_length(buf + len, wd);
  buf[len++] = ' ';
  len += pdf_sprint_length(buf + len, ht);
  buf[len++] = ' ';
  buf[len++] = 'r'; buf[len++] = 'e';

  if (opchr != ' ') {
    buf[len++] = ' ';
    buf[len++] = opchr;

    buf[len++] = ' ';
    buf[len++] = isclip ? 'n' : 'Q';
  }

  pdf_doc_add_page_content(buf, len);  /* op: q cm n re Q */

  return 0;
}

static int path_added = 0;

/* FIXME */
static int
pdf_dev__flushpath (pdf_path  *pa,
                    char       opchr,
                    int        rule,
                    int        ignore_rule)
{
  pa_elem   *pe, *pe1;
  char      *b      = fmt_buf;
  int        b_len  = FORMAT_BUFF_LEN;
  pdf_rect   r; /* FIXME */
  pdf_coord *pt;
  int        n_pts, n_seg;
  int        len = 0;
  int        isclip = 0;
  int        isrect, i, j;

  ASSERT(pa && PT_OP_VALID(opchr));

  isclip = (opchr == 'W') ? 1 : 0;

  if (PA_LENGTH(pa) <= 0 && path_added == 0)
    return 0;

  path_added = 0;
  graphics_mode();
  isrect = pdf_path__isarect(pa, ignore_rule); 
  if (isrect) {
    pe  = &(pa->path[0]);
    pe1 = &(pa->path[2]);

    r.llx = pe->p[0].x;
    r.lly = pe->p[0].y;
    r.urx = pe1->p[0].x - pe->p[0].x; /* width...  */
    r.ury = pe1->p[0].y - pe->p[0].y; /* height... */

    b[len++] = ' ';
    len += pdf_sprint_rect(b + len, &r);
    b[len++] = ' ';
    b[len++] = 'r';
    b[len++] = 'e';
    pdf_doc_add_page_content(b, len);  /* op: re */
    len = 0;
  } else {
    n_seg = PA_LENGTH(pa);
    for (i = 0, len = 0, pe = &pa->path[0];
         i < n_seg; pe++, i++) {
      n_pts = PE_N_PTS(pe);
      for (j = 0, pt = &pe->p[0];
           j < n_pts; j++, pt++) {
        b[len++] = ' ';
        len += pdf_sprint_coord(b + len, pt);
      }
      b[len++] = ' ';
      b[len++] = PE_OPCHR(pe);
      if (len + 128 > b_len) {
        pdf_doc_add_page_content(b, len);  /* op: m l c v y h */
	len = 0;
      }
    }
    if (len > 0) {
      pdf_doc_add_page_content(b, len);  /* op: m l c v y h */
      len = 0;
    }
  }

  b[len++] = ' ';
  b[len++] = opchr;
  if (rule == PDF_FILL_RULE_EVENODD)
    b[len++] = '*';
  if (isclip) {
    b[len++] = ' '; b[len++] = 'n';
  }

  pdf_doc_add_page_content(b, len);  /* op: f F s S b B W f* F* s* S* b* B* W* */

  return 0;
}


/* Graphics State */
typedef struct pdf_gstate_
{
  pdf_coord   cp;

  pdf_tmatrix matrix;   /* cm,  - */

  pdf_color   strokecolor;
  pdf_color   fillcolor;
  /* colorspace here */

  struct {
    int     num_dash;
    double  pattern[PDF_DASH_SIZE_MAX];
    double  offset;
  } linedash;           /* d,  D  */

  double    linewidth;  /* w,  LW */

  int       linecap;    /* J,  LC */
  int       linejoin;   /* j,  LJ */
  double    miterlimit; /* M,  ML */

  int       flatness;   /* i,  FL, 0 to 100 (0 for use device-default) */

  /* internal */
  pdf_path  path;
  int       flags;
  /* bookkeeping the origin of the last transform applied */
  pdf_coord pt_fixee;
} pdf_gstate;


typedef struct m_stack_elem
{
  void                *data;
  struct m_stack_elem *prev;
} m_stack_elem;

typedef struct m_stack
{
  int           size;
  m_stack_elem *top;
  m_stack_elem *bottom;
} m_stack;

static void
m_stack_init (m_stack *stack)
{
  ASSERT(stack);

  stack->size   = 0;
  stack->top    = NULL;
  stack->bottom = NULL;

  return;
}

static void
m_stack_push (m_stack *stack, void *data)
{
  m_stack_elem  *elem;

  ASSERT(stack);

  elem = NEW(1, m_stack_elem);
  elem->prev = stack->top;
  elem->data = data;

  stack->top = elem;
  if (stack->size == 0)
    stack->bottom = elem;

  stack->size++;

  return;
}

static void *
m_stack_pop (m_stack *stack)
{
  m_stack_elem *elem;
  void         *data;

  ASSERT(stack);

  if (stack->size == 0)
    return NULL;

  data = stack->top->data;
  elem = stack->top;
  stack->top = elem->prev;
  if (stack->size == 1)
    stack->bottom = NULL;
  RELEASE(elem);

  stack->size--;

  return data;
}

static void *
m_stack_top (m_stack *stack)
{
  void  *data;

  ASSERT(stack);

  if (stack->size == 0)
    return NULL;

  data = stack->top->data;

  return data;
}

#define m_stack_depth(s)    ((s)->size)

static m_stack gs_stack;

static void
init_a_gstate (pdf_gstate *gs)
{
  gs->cp.x = 0.0;
  gs->cp.y = 0.0;

  pdf_setmatrix(&gs->matrix, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

  pdf_color_black(&gs->strokecolor);
  pdf_color_black(&gs->fillcolor);

  gs->linedash.num_dash = 0;
  gs->linedash.offset   = 0;
  gs->linecap    = 0;
  gs->linejoin   = 0;
  gs->linewidth  = 1.0;
  gs->miterlimit = 10.0;

  gs->flatness   = 1; /* default to 1 in PDF */

  /* Internal variables */
  gs->flags = 0;
  init_a_path(&gs->path);
  gs->pt_fixee.x = 0;
  gs->pt_fixee.y = 0;

  return;
}

static void
clear_a_gstate (pdf_gstate *gs)
{
  clear_a_path(&gs->path);
  memset(gs, 0, sizeof(pdf_gstate));

  return;
}

static void
copy_a_gstate (pdf_gstate *gs1, pdf_gstate *gs2)
{
  int   i;

  ASSERT(gs1 && gs2);

  gs1->cp.x = gs2->cp.x;
  gs1->cp.y = gs2->cp.y;

  pdf_copymatrix(&gs1->matrix, &gs2->matrix);

  /* TODO:
   * Path should be linked list and gsave only
   * record starting point within path rather than
   * copying whole path.
   */
  pdf_path__copypath(&gs1->path, &gs2->path);

  gs1->linedash.num_dash = gs2->linedash.num_dash;
  for (i = 0; i < gs2->linedash.num_dash; i++) {
    gs1->linedash.pattern[i] = gs2->linedash.pattern[i];
  }
  gs1->linedash.offset = gs2->linedash.offset;

  gs1->linecap    = gs2->linecap;
  gs1->linejoin   = gs2->linejoin;
  gs1->linewidth  = gs2->linewidth;
  gs1->miterlimit = gs2->miterlimit;
  gs1->flatness   = gs2->flatness;

  pdf_color_copycolor(&gs1->fillcolor  , &gs2->fillcolor);
  pdf_color_copycolor(&gs1->strokecolor, &gs2->strokecolor);
  gs1->pt_fixee.x = gs2->pt_fixee.x;
  gs1->pt_fixee.y = gs2->pt_fixee.y;

  return;
}
    
void
pdf_dev_init_gstates (void)
{
  pdf_gstate *gs;

  m_stack_init(&gs_stack);

  gs = NEW(1, pdf_gstate);
  init_a_gstate(gs);

  m_stack_push(&gs_stack, gs); /* Initial state */

  return;
}

void
pdf_dev_clear_gstates (void)
{
  pdf_gstate *gs;

  if (m_stack_depth(&gs_stack) > 1) /* at least 1 elem. */
    WARN("GS stack depth is not zero at the end of the document.");

  while ((gs = m_stack_pop(&gs_stack)) != NULL) {
    clear_a_gstate(gs);
    RELEASE(gs);
  }
  return;
}

int
pdf_dev_gsave (void)
{
  pdf_gstate *gs0, *gs1;

  gs0 = m_stack_top(&gs_stack);
  gs1 = NEW(1, pdf_gstate);
  init_a_gstate(gs1);
  copy_a_gstate(gs1, gs0);
  m_stack_push(&gs_stack, gs1);

  pdf_doc_add_page_content(" q", 2);  /* op: q */

  return 0;
}

int
pdf_dev_grestore (void)
{
  pdf_gstate *gs;

  if (m_stack_depth(&gs_stack) <= 1) { /* Initial state at bottom */
    WARN("Too many grestores.");
    return  -1;
  }

  gs = m_stack_pop(&gs_stack);
  clear_a_gstate(gs);
  RELEASE(gs);

  pdf_doc_add_page_content(" Q", 2);  /* op: Q */

  pdf_dev_reset_fonts(0);

  return  0;
}


int
pdf_dev_push_gstate (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs0;

  gs0 = NEW(1, pdf_gstate);

  init_a_gstate(gs0);

  m_stack_push(gss, gs0);

  return 0;
}


int
pdf_dev_pop_gstate (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs;

  if (m_stack_depth(gss) <= 1) { /* Initial state at bottom */
    WARN("Too many grestores.");
    return  -1;
  }

  gs = m_stack_pop(gss);
  clear_a_gstate(gs);
  RELEASE(gs);

  return  0;
}


int
pdf_dev_current_depth (void)
{
  return (m_stack_depth(&gs_stack) - 1); /* 0 means initial state */
}

void
pdf_dev_grestore_to (int depth)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs;

  ASSERT(depth >= 0);

  if (m_stack_depth(gss) > depth + 1) {
    WARN("Closing pending transformations at end of page/XObject.");
  }

  while (m_stack_depth(gss) > depth + 1) {
    pdf_doc_add_page_content(" Q", 2);  /* op: Q */
    gs = m_stack_pop(gss);
    clear_a_gstate(gs);
    RELEASE(gs);
  }
  pdf_dev_reset_fonts(0);

  return;
}

int
pdf_dev_currentpoint (pdf_coord *p)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_coord  *cpt = &gs->cp;

  ASSERT(p);

  p->x = cpt->x; p->y = cpt->y;

  return 0;
}

int
pdf_dev_currentmatrix (pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_tmatrix *CTM = &gs->matrix;

  ASSERT(M);

  pdf_copymatrix(M, CTM);

  return 0;
}

#if  0
int
pdf_dev_currentcolor (pdf_color *color, int is_fill)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_color  *fcl = &gs->fillcolor;
  pdf_color  *scl = &gs->strokecolor;

  ASSERT(color);

  pdf_color_copycolor(color, is_fill ? fcl : scl);

  return 0;
}
#endif /* 0 */

/*
 * mask == 0 means stroking color, mask == 0x20 nonstroking color
 *
 * force == 1 means that operators will be generated even if
 *   the color is the same as the current graphics state color
 */
void
pdf_dev_set_color (const pdf_color *color, char mask, int force)
{
  int len;

  pdf_gstate *gs  = m_stack_top(&gs_stack);
  pdf_color *current = mask ? &gs->fillcolor : &gs->strokecolor;

  ASSERT(pdf_color_is_valid(color));

  if (!(pdf_dev_get_param(PDF_DEV_PARAM_COLORMODE) &&
	(force || pdf_color_compare(color, current))))
    /* If "color" is already the current color, then do nothing
     * unless a color operator is forced
     */
    return;

  graphics_mode();
  len = pdf_color_to_string(color, fmt_buf, mask);
  fmt_buf[len++] = ' ';
  switch (pdf_color_type(color)) {
  case  PDF_COLORSPACE_TYPE_RGB:
    fmt_buf[len++] = 'R' | mask;
    fmt_buf[len++] = 'G' | mask;
    break;
  case  PDF_COLORSPACE_TYPE_CMYK:
    fmt_buf[len++] = 'K' | mask;
    break;
  case  PDF_COLORSPACE_TYPE_GRAY:
    fmt_buf[len++] = 'G' | mask;
    break;
  default: /* already verified the given color */
    break;
  }
  pdf_doc_add_page_content(fmt_buf, len);  /* op: RG K G rg k g etc. */

  pdf_color_copycolor(current, color);
}

int
pdf_dev_concat (const pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_path    *cpa = &gs->path;
  pdf_coord   *cpt = &gs->cp;
  pdf_tmatrix *CTM = &gs->matrix;
  pdf_tmatrix  W   = {0, 0, 0, 0, 0, 0};  /* Init to avoid compiler warning */
  char        *buf = fmt_buf;
  int          len = 0;

  ASSERT(M);

  /* Adobe Reader erases page content if there are
   * non invertible transformation.
   */
  if (fabs(detP(M)) < 1.0e-8) {
    WARN("Transformation matrix not invertible.");
    WARN("--- M = [%g %g %g %g %g %g]",
         M->a, M->b, M->c, M->d, M->e, M->f);
    return -1;
  }

  if (fabs(M->a - 1.0) > 1.e-8 || fabs(M->b) > 1.e-8
   || fabs(M->c) > 1.e-8 || fabs(M->d - 1.0) > 1.e-8
   || fabs(M->e) > 1.e-8 || fabs(M->f) > 1.e-8) {
    buf[len++] = ' ';
    len += pdf_sprint_matrix(buf + len, M);
    buf[len++] = ' ';
    buf[len++] = 'c';
    buf[len++] = 'm';
    pdf_doc_add_page_content(buf, len);  /* op: cm */

    pdf_concatmatrix(CTM, M);
  }
  inversematrix(&W, M);

  pdf_path__transform (cpa, &W);
  pdf_coord__transform(cpt, &W);

  return 0;
}

/*
 * num w        LW  linewidth (g.t. 0)
 * int J        LC  linecap
 * int j        LJ  linejoin
 * num M        ML  miter limit (g.t. 0)
 * array num d  D   line dash
 * int ri       RI  renderint intnet
 * int i        FL  flatness tolerance (0-100)
 * name gs      --  name: res. name of ExtGState dict.  
 */      
int
pdf_dev_setmiterlimit (double mlimit)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  int         len = 0;
  char       *buf = fmt_buf;

  if (gs->miterlimit != mlimit) {
    buf[len++] = ' ';
    len += pdf_sprint_length(buf + len, mlimit);
    buf[len++] = ' ';
    buf[len++] = 'M';
    pdf_doc_add_page_content(buf, len);  /* op: M */
    gs->miterlimit = mlimit;
  }

  return 0;
}

int
pdf_dev_setlinecap (int capstyle)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  int         len = 0;
  char       *buf = fmt_buf;

  if (gs->linecap != capstyle) {
    len = sprintf(buf, " %d J", capstyle);
    pdf_doc_add_page_content(buf, len);  /* op: J */
    gs->linecap = capstyle;
  }

  return 0;
}

int
pdf_dev_setlinejoin (int joinstyle)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  int         len = 0;
  char       *buf = fmt_buf;

  if (gs->linejoin != joinstyle) {
    len = sprintf(buf, " %d j", joinstyle);
    pdf_doc_add_page_content(buf, len);  /* op: j */
    gs->linejoin = joinstyle;
  }

  return 0;
}

int
pdf_dev_setlinewidth (double width)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);  
  int         len = 0;
  char       *buf = fmt_buf;

  if (gs->linewidth != width) {
    buf[len++] = ' ';
    len += pdf_sprint_length(buf + len, width);
    buf[len++] = ' ';
    buf[len++] = 'w';
    pdf_doc_add_page_content(buf, len);  /* op: w */
    gs->linewidth = width;
  }

  return 0;
}

int
pdf_dev_setdash (int count, double *pattern, double offset)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  int         len = 0;
  char       *buf = fmt_buf;
  int         i;

  gs->linedash.num_dash = count;
  gs->linedash.offset   = offset;
  pdf_doc_add_page_content(" [", 2);  /* op: */
  for (i = 0; i < count; i++) {
    buf[0] = ' ';
    len = pdf_sprint_length (buf + 1, pattern[i]);
    pdf_doc_add_page_content(buf, len + 1);  /* op: */
    gs->linedash.pattern[i] = pattern[i];
  }
  pdf_doc_add_page_content("] ", 2);  /* op: */
  len = pdf_sprint_length (buf, offset);
  pdf_doc_add_page_content(buf, len);  /* op: */
  pdf_doc_add_page_content(" d", 2);  /* op: d */

  return 0;
}

#if 0
int
pdf_dev_setflat (int flatness)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  int         len = 0;
  char       *buf = fmt_buf;

  if (flatness < 0 || flatness > 100)
    return -1;

  if (gs->flatness != flatness) {
    gs->flatness = flatness;
    len = sprintf(buf, " %d i", flatness);
    pdf_doc_add_page_content(buf, len);  /* op: i */
  }

  return 0;
}
#endif

/* ZSYUEDVEDEOF */
int
pdf_dev_clip (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;

  return pdf_dev__flushpath(cpa, 'W', PDF_FILL_RULE_NONZERO, 0);
}

int
pdf_dev_eoclip (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;

  return pdf_dev__flushpath(cpa, 'W', PDF_FILL_RULE_EVENODD, 0);
}

int
pdf_dev_flushpath (char p_op, int fill_rule)
{
  m_stack    *gss   = &gs_stack;
  pdf_gstate *gs    = m_stack_top(gss);
  pdf_path   *cpa   = &gs->path;
  int         error = 0;

  /* last arg 'ignore_rule' is only for single object
   * that can be converted to a rect where fill rule
   * is inessential.
   */
  error = pdf_dev__flushpath(cpa, p_op, fill_rule, 1);
  pdf_path__clearpath(cpa);

  gs->flags &= ~GS_FLAG_CURRENTPOINT_SET;

  return error;
}

int
pdf_dev_newpath (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *p   = &gs->path;

  if (PA_LENGTH(p) > 0) {
    pdf_path__clearpath (p);
  }
  /* The following is required for "newpath" operator in mpost.c. */
  pdf_doc_add_page_content(" n", 2);  /* op: n */

  return 0;
}

int
pdf_dev_moveto (double x, double y)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p;

  p.x = x; p.y = y;
  return pdf_path__moveto(cpa, cpt, &p); /* cpt updated */
}

int
pdf_dev_rmoveto (double x, double y)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p;

  p.x = cpt->x + x;
  p.y = cpt->y + y;
  return pdf_path__moveto(cpa, cpt, &p); /* cpt updated */
}

int
pdf_dev_lineto (double x, double y)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0;

  p0.x = x; p0.y = y;

  return pdf_path__lineto(cpa, cpt, &p0);
}

int
pdf_dev_rlineto (double x, double y)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0;

  p0.x = x + cpt->x; p0.y = y + cpt->y;

  return pdf_path__lineto(cpa, cpt, &p0);
}

int
pdf_dev_curveto (double x0, double y0,
                 double x1, double y1,
                 double x2, double y2)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0, p1, p2;

  p0.x = x0; p0.y = y0;
  p1.x = x1; p1.y = y1;
  p2.x = x2; p2.y = y2;

  return pdf_path__curveto(cpa, cpt, &p0, &p1, &p2);
}

int
pdf_dev_vcurveto (double x0, double y0,
                  double x1, double y1)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0, p1;

  p0.x = x0; p0.y = y0;
  p1.x = x1; p1.y = y1;

  return pdf_path__curveto(cpa, cpt, cpt, &p0, &p1);
}

int
pdf_dev_ycurveto (double x0, double y0,
                  double x1, double y1)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0, p1;

  p0.x = x0; p0.y = y0;
  p1.x = x1; p1.y = y1;

  return pdf_path__curveto(cpa, cpt, &p0, &p1, &p1);
}

int
pdf_dev_rcurveto (double x0, double y0,
                  double x1, double y1,
                  double x2, double y2)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   p0, p1, p2;

  p0.x = x0 + cpt->x; p0.y = y0 + cpt->y;
  p1.x = x1 + cpt->x; p1.y = y1 + cpt->y;
  p2.x = x2 + cpt->x; p2.y = y2 + cpt->y;

  return pdf_path__curveto(cpa, cpt, &p0, &p1, &p2);
}


int
pdf_dev_closepath (void)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_coord  *cpt = &gs->cp;
  pdf_path   *cpa = &gs->path;

  return pdf_path__closepath(cpa, cpt);
}


void
pdf_dev_dtransform (pdf_coord *p, const pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_tmatrix *CTM = &gs->matrix;

  ASSERT(p);

  pdf_coord__dtransform(p, (M ? M : CTM));

  return;
}

void
pdf_dev_idtransform (pdf_coord *p, const pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_tmatrix *CTM = &gs->matrix;

  ASSERT(p);

  pdf_coord__idtransform(p, (M ? M : CTM));

  return;
}

void
pdf_dev_transform (pdf_coord *p, const pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_tmatrix *CTM = &gs->matrix;

  ASSERT(p);

  pdf_coord__transform(p, (M ? M : CTM));

  return;
}

#if 0
void
pdf_dev_itransform (pdf_coord *p, const pdf_tmatrix *M)
{
  m_stack     *gss = &gs_stack;
  pdf_gstate  *gs  = m_stack_top(gss);
  pdf_tmatrix *CTM = &gs->matrix;

  ASSERT(p);

  pdf_coord__itransform(p, (M ? M : CTM));

  return;
}
#endif

int
pdf_dev_arc  (double c_x , double c_y, double r,
              double a_0 , double a_1)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   c;

  c.x = c_x; c.y = c_y;

  return  pdf_path__elliptarc(cpa, cpt, &c, r, r, 0.0, a_0, a_1, +1);
}

/* *negative* arc */
int
pdf_dev_arcn (double c_x , double c_y, double r,
              double a_0 , double a_1)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   c;

  c.x = c_x; c.y = c_y;

  return  pdf_path__elliptarc(cpa, cpt, &c, r, r, 0.0, a_0, a_1, -1);
}

int
pdf_dev_arcx (double c_x , double c_y,
              double r_x , double r_y,
              double a_0 , double a_1,
              int    a_d ,
              double xar)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;
  pdf_coord   c;

  c.x = c_x; c.y = c_y;

  return  pdf_path__elliptarc(cpa, cpt, &c, r_x, r_y, xar, a_0, a_1, a_d);
}

/* Required by Tpic */
int
pdf_dev_bspline (double x0, double y0,
                 double x1, double y1, double x2, double y2)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  pdf_path   *cpa = &gs->path;
  pdf_coord  *cpt = &gs->cp;  
  pdf_coord   p1, p2, p3;

  p1.x = x0 + 2.0 * (x1 - x0) / 3.0;
  p1.y = y0 + 2.0 * (y1 - y0) / 3.0;
  p2.x = x1 + (x2 - x1) / 3.0;
  p2.y = y1 + (y2 - y1) / 3.0;
  p3.x = x2;
  p3.y = y2;

  return  pdf_path__curveto(cpa, cpt, &p1, &p2, &p3);
}

#if 0
int
pdf_dev_rectstroke (double x, double y,
                    double w, double h,
                    const pdf_tmatrix *M  /* optional */
                   )
{
  pdf_rect r;

  r.llx = x;
  r.lly = y;
  r.urx = x + w;
  r.ury = y + h;

  return  pdf_dev__rectshape(&r, M, 'S');
}
#endif

int
pdf_dev_rectfill  (double x, double y,
                   double w, double h)
{
  pdf_rect r;

  r.llx = x;
  r.lly = y;
  r.urx = x + w;
  r.ury = y + h;

  return  pdf_dev__rectshape(&r, NULL, 'f');
}

int
pdf_dev_rectclip (double x, double y,
                  double w, double h)
{
  pdf_rect r;

  r.llx = x;
  r.lly = y;
  r.urx = x + w;
  r.ury = y + h;
  
  return  pdf_dev__rectshape(&r, NULL, 'W');
}

int
pdf_dev_rectadd (double x, double y,
                  double w, double h)
{
  pdf_rect r;

  r.llx = x;
  r.lly = y;
  r.urx = x + w;
  r.ury = y + h;
  path_added = 1;

  return  pdf_dev__rectshape(&r, NULL, ' ');
}

void
pdf_dev_set_fixed_point (double x, double y)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  gs->pt_fixee.x = x;
  gs->pt_fixee.y = y;
}

void
pdf_dev_get_fixed_point (pdf_coord *p)
{
  m_stack    *gss = &gs_stack;
  pdf_gstate *gs  = m_stack_top(gss);
  p->x = gs->pt_fixee.x;
  p->y = gs->pt_fixee.y;
}
