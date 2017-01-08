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

#include <string.h>
#include <math.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "numbers.h"

#include "pdfobj.h"
#include "pdffont.h"

#include "pdfencoding.h"
#include "unicode.h"

#include "dpxutil.h"

#include "pst_obj.h"
#include "pst.h"

#include "cff_limits.h"
#include "cff_types.h"
#include "cff_dict.h"
#include "cff.h"

#include "t1_char.h"

/*
 * Type 1 -> Type 1C
 */

/* Charstring decoder/encoder status codes */
#define CS_OP_NOSUPPORT -4
#define CS_BUFFER_ERROR -3
#define CS_STACK_ERROR  -2
#define CS_PARSE_ERROR  -1
#define CS_PARSE_OK      0
#define CS_PARSE_END     1
#define CS_SUBR_RETURN   2
#define CS_CHAR_END      3

static int status = CS_PARSE_ERROR;

#define DST_NEED(a,b) {if ((a) < (b)) { status = CS_BUFFER_ERROR ; return ; }}
#define SRC_NEED(a,b) {if ((a) < (b)) { status = CS_PARSE_ERROR  ; return ; }}
#define NEED(a,b)     {if ((a) < (b)) { status = CS_STACK_ERROR  ; return ; }}

#define T1_CS_PHASE_INIT 0
#define T1_CS_PHASE_HINT 1
#define T1_CS_PHASE_PATH 2
#define T1_CS_PHASE_FLEX 3

static int phase = -1;
static int nest  = -1;

#ifndef CS_STEM_ZONE_MAX
#define CS_STEM_ZONE_MAX 96
#endif
#ifndef CS_STEM_GROUP_MAX
#define CS_STEM_GROUP_MAX CS_STEM_ZONE_MAX
#endif
#ifndef CS_ARG_STACK_MAX
#define CS_ARG_STACK_MAX 48
#endif
#ifndef PS_ARG_STACK_MAX
/*
 * Counter control may have CS_STEM_ZONE_MAX*2+2 arguments.
 */
#define PS_ARG_STACK_MAX (CS_STEM_ZONE_MAX*2+2)
#endif

typedef struct t1_cpath {
  int     type;
  int     num_args;
  double  args[CS_ARG_STACK_MAX];
  struct t1_cpath *next;
} t1_cpath;

#define HSTEM 0
#define VSTEM 1
typedef struct {
  int id;
  int dir;
  double pos, del;
} t1_stem;

typedef struct {
  int    num_stems;
  double stems[CS_STEM_ZONE_MAX]; /* int */
} t1_stemgroup;

#define T1_CS_FLAG_NONE 0
#define T1_CS_FLAG_USE_HINTMASK (1 << 0)
#define T1_CS_FLAG_USE_CNTRMASK (1 << 1)
#define T1_CS_FLAG_USE_SEAC     (1 << 2)

typedef struct {
  int flags;
  struct {
    double sbx, sby, wx, wy;
  } sbw;
  struct {
    double llx, lly, urx, ury;
  } bbox;
  struct {
    double asb, adx, ady;
    card8 bchar, achar;
  } seac;
  int       num_stems;
  t1_stem   stems[CS_STEM_ZONE_MAX];
  t1_cpath *charpath;
  t1_cpath *lastpath;
} t1_chardesc;

static int cs_stack_top = 0;
static int ps_stack_top = 0;

/* [vh]stem support require one more stack size. */
static double cs_arg_stack[CS_ARG_STACK_MAX+1];
static double ps_arg_stack[PS_ARG_STACK_MAX];

#define CS_HINT_DECL -1
#define CS_FLEX_CTRL -2
#define CS_CNTR_CTRL -3

/*
 * Type 1/2 CharString encoding
 */

/*
 * 1-byte CharString operaotrs:
 *  cs_escape is first byte of two-byte operator
 */

/*      RESERVED      0 */
#define cs_hstem      1
/*      RESERVED      2 */
#define cs_vstem      3
#define cs_vmoveto    4
#define cs_rlineto    5
#define cs_hlineto    6
#define cs_vlineto    7
#define cs_rrcurveto  8
#define cs_closepath  9
#define cs_callsubr   10
#define cs_return     11
#define cs_escape     12
#define cs_hsbw       13
#define cs_endchar    14
/*      RESERVED      15 */
/*      RESERVED      16 */
/*      RESERVED      17 */
#define cs_hstemhm    18
#define cs_hintmask   19
#define cs_cntrmask   20
#define cs_rmoveto    21
#define cs_hmoveto    22
#define cs_vstemhm    23
#define cs_rcurveline 24
#define cs_rlinecurve 25
#define cs_vvcurveto  26
#define cs_hhcurveto  27
/*      SHORTINT      28 : first byte of shortint*/
#define cs_callgsubr  29
#define cs_vhcurveto  30
#define cs_hvcurveto  31

/* 2-byte op. flex 34-37 used. */

/*
 * 2-byte CharString operaotrs:
 *  "dotsection" is obsoleted in Type 2 charstring.
 */

#define cs_dotsection 0
#define cs_vstem3     1
#define cs_hstem3     2
#define cs_and        3
#define cs_or         4
#define cs_not        5
#define cs_seac       6
#define cs_sbw        7
/*      RESERVED      8  */
#define cs_abs        9
#define cs_add        10
#define cs_sub        11
#define cs_div        12
/*      RESERVED      13 */
#define cs_neg        14
#define cs_eq         15
#define cs_callothersubr 16
#define cs_pop        17
#define cs_drop       18
/*      RESERVED      19 */
#define cs_put        20
#define cs_get        21
#define cs_ifelse     22 
#define cs_random     23
#define cs_mul        24
/*      RESERVED      25 */
#define cs_sqrt       26
#define cs_dup        27
#define cs_exch       28
#define cs_index      29
#define cs_roll       30
/*      RESERVED      31 */
/*      RESERVED      32 */
#define cs_setcurrentpoint 33
#define cs_hflex      34
#define cs_flex       35
#define cs_hflex1     36
#define cs_flex1      37

#define IS_PATH_OPERATOR(o) (((o) >= cs_vmoveto && (o) <= cs_closepath) || \
                             ((o) >= cs_rmoveto && (o) <= cs_hvcurveto && \
                              (o) != cs_vstemhm && (o) != cs_callgsubr && (o) != 28)\
                            )

/*
 * Stem:
 *
 *   1. Stems must be sorted in the increasing bottom/left edge order.
 *   2. The encoded values are all relative; The value x(y) of the first
 *      stem is relative to 0 for Type 2 charstring and is relative to
 *      the left(bottom) side-bearing for Type 1 charstring.
 *   3. A width of -20(-21) specifies the top/right(bottom/left) edge
 *      of an edge hint in Type 2 charstring. But the width of 'ghost'
 *      hint in Type 1 charstring is positive with value 20 or 21.
 *   4. The h(v)stemhm MUST be used instead of h(v)stem if charstring
 *      contains hintmask operator.
 *
 * TODO:
 *
 *  Convert ghost hint to edge hint, Counter control for hstem3/vstem3.
 */

static inline int
stem_compare (const void *v1, const void *v2)
{
  int cmp = 0;
  const t1_stem *s1, *s2;

  s1 = (const t1_stem *) v1;
  s2 = (const t1_stem *) v2;
  if (s1->dir == s2->dir) {
    if (s1->pos == s2->pos) {
      if (s1->del == s2->del)
        cmp = 0;
      else
        cmp = (s1->del < s2->del) ? -1 : 1;
    } else {
      cmp = (s1->pos < s2->pos) ? -1 : 1;
    }
  } else {
    cmp = (s1->dir == HSTEM) ? -1 : 1;
  }

  return cmp;
}

#define SORT_STEMS(cd) qsort((cd)->stems,(cd)->num_stems,sizeof(t1_stem),stem_compare)

static int
get_stem (t1_chardesc *cd, int stem_id)
{
  int i;

  for (i = 0; i < cd->num_stems; i++) {
    if (cd->stems[i].id == stem_id)
      break;
  }

  return ((i < cd->num_stems) ? i : -1);
}

static int
add_stem (t1_chardesc *cd, double pos, double del, int dir)
{
  int i;

  ASSERT(cd);

  pos += (dir == HSTEM) ? cd->sbw.sby : cd->sbw.sbx;
  for (i = 0; i < cd->num_stems; i++) {
    if (cd->stems[i].dir == dir &&
        cd->stems[i].pos == pos &&
        cd->stems[i].del == del)
      break;
  }
  if (i == cd->num_stems) {
    if (cd->num_stems == CS_STEM_ZONE_MAX)
      return -1;
    cd->stems[i].dir = dir;
    cd->stems[i].pos = pos;
    cd->stems[i].del = del;
    cd->stems[i].id  = cd->num_stems;
    (cd->num_stems)++;
  }

  return cd->stems[i].id;
}


static void
copy_args (double *args1, double *args2, int count)
{
  while (count-- > 0) {
    *args1 = *args2;
    args1++; args2++;
  }
}

/*
 * Stack:
 */
#define LIMITCHECK(n) do {\
                           if (cs_stack_top+(n) > CS_ARG_STACK_MAX) {\
                             status = CS_STACK_ERROR;\
                             return;\
                           }\
                      } while (0)
#define CHECKSTACK(n) do {\
                           if (cs_stack_top < (n)) {\
                             status = CS_STACK_ERROR;\
                             return;\
                           }\
                      } while (0)
#define CLEARSTACK()  do {\
                           cs_stack_top = 0;\
                      } while (0)

/*
 * Path construction:
 */
/* Get operands from cs_arg_stack[] */
static void
add_charpath (t1_chardesc *cd, int type, double *argv, int argn)
{
  t1_cpath *p;

  ASSERT(cd);
  ASSERT(argn <= CS_ARG_STACK_MAX);

  p = NEW(1, t1_cpath);
  p->type     = type;
  p->num_args = argn;
  p->next     = NULL;

  while (argn-- > 0)
    p->args[argn] = argv[argn];

  if (!cd->charpath)
    cd->charpath = p;
  if (cd->lastpath)
    cd->lastpath->next = p;
  cd->lastpath = p;

  if (type >= 0 &&
      phase != T1_CS_PHASE_FLEX && IS_PATH_OPERATOR(type))
    phase = T1_CS_PHASE_PATH;
}

static void
init_charpath (t1_chardesc *cd)
{
  cd->flags = T1_CS_FLAG_NONE;
  cd->num_stems = 0;
  cd->sbw.wx  = cd->sbw.wy  = 0.0;
  cd->sbw.sbx = cd->sbw.sby = 0.0;
  cd->bbox.llx = cd->bbox.lly = cd->bbox.urx = cd->bbox.ury = 0.0;
  cd->charpath = cd->lastpath = NULL;
}

static void
release_charpath (t1_chardesc *cd)
{
  t1_cpath *curr, *next;

  ASSERT(cd);

  curr = cd->charpath;
  while (curr != NULL) {
    next = curr->next;
    RELEASE(curr);
    curr = next;
  }

  cd->charpath = cd->lastpath = NULL;
}

/*
 * Type 1 charstring operators:
 */
#define ADD_PATH(p,t,n) add_charpath((p),(t),&(cs_arg_stack[cs_stack_top-(n)]),(n))

/*
 * Single byte operators:
 */
static void
do_operator1 (t1_chardesc *cd, card8 **data)
{
  card8 op = **data;

  *data += 1;

  switch (op) {
  case cs_closepath:
    /*
     * From T1 spec.:
     *  Note that, unlike the closepath command in the PostScript language,
     *  this command does not reposition the current point. Any subsequent
     *  rmoveto must be relative to the current point in force before the
     *  Type 1 font format closepath command was given.
     */
    /* noop */
    CLEARSTACK();
    break;
  case cs_hsbw:
    CHECKSTACK(2);
    cd->sbw.wx  = cs_arg_stack[--cs_stack_top];
    cd->sbw.wy  = 0;
    cd->sbw.sbx = cs_arg_stack[--cs_stack_top];
    cd->sbw.sby = 0;
    CLEARSTACK();
    /* hsbw does NOT set currentpoint. */
    break;
  case cs_hstem:
  case cs_vstem:
    CHECKSTACK(2);
    {
      int stem_id;
      stem_id = add_stem(cd,
                         cs_arg_stack[cs_stack_top-2],
                         cs_arg_stack[cs_stack_top-1],
                         ((op == cs_hstem) ? HSTEM : VSTEM));
      if (stem_id < 0) {
        WARN("Too many hints...");
        status = CS_PARSE_ERROR;
        return;
      }
      /* Put stem_id onto the stack... */
      cs_arg_stack[cs_stack_top++] = stem_id;
      ADD_PATH(cd, CS_HINT_DECL, 1);
    }
    CLEARSTACK();
    break;
  case cs_rmoveto:
    /*
     * Reference point is (0, 0) in Type 2 charstring.
     */
    CHECKSTACK(2);
    {
      if (phase < T1_CS_PHASE_PATH) {
        cs_arg_stack[cs_stack_top-2] += cd->sbw.sbx;
        cs_arg_stack[cs_stack_top-1] += cd->sbw.sby;
      }
      ADD_PATH(cd, op, 2);
    }
    CLEARSTACK();
    break;
  case cs_hmoveto:
  case cs_vmoveto:
    CHECKSTACK(1);
    {
      int argn = 1;
      if (phase < T1_CS_PHASE_PATH) {
        /*
         * The reference point for the first moveto operator is diferrent
         * between Type 1 charstring and Type 2 charstring. We compensate it.
         */
        if (op == cs_hmoveto) {
          cs_arg_stack[cs_stack_top-1] += cd->sbw.sbx;
          if (cd->sbw.sby != 0.0) {
            cs_arg_stack[cs_stack_top++] = cd->sbw.sby;
            argn = 2;
            op = cs_rmoveto;
          }
        } else {
          cs_arg_stack[cs_stack_top-1] += cd->sbw.sby;
          if (cd->sbw.sbx != 0.0) {
            cs_arg_stack[cs_stack_top]   = cs_arg_stack[cs_stack_top-1];
            cs_arg_stack[cs_stack_top-1] = cd->sbw.sbx;
            cs_stack_top++;
            argn = 2;
            op = cs_rmoveto;
          }
        }
      }
      ADD_PATH(cd, op, argn);
    }
    CLEARSTACK();
    break;
  case cs_endchar:
    status = CS_CHAR_END;
    CLEARSTACK();
    break;
  /* above oprators are candidate for first stack-clearing operator */
  case cs_rlineto:
    CHECKSTACK(2);
    ADD_PATH(cd, op, 2);
    CLEARSTACK();
    break;
  case cs_hlineto:
  case cs_vlineto:
    CHECKSTACK(1);
    ADD_PATH(cd, op, 1);
    CLEARSTACK();
    break;
  case cs_rrcurveto:
    CHECKSTACK(6);
    ADD_PATH(cd, op, 6);
    CLEARSTACK();
    break;
  case cs_vhcurveto:
  case cs_hvcurveto:
    CHECKSTACK(4);
    ADD_PATH(cd, op, 4);
    CLEARSTACK();
    break;
    /* all operotors above are stack-clearing operator */
    /* no output */
  case cs_return:
    break;
  case cs_callsubr:
    ERROR("Unexpected callsubr.");
    break;
  default:
    /* no-op ? */
    WARN("Unknown charstring operator: 0x%02x", op);
    status = CS_PARSE_ERROR;
    break;
  }

  return;
}

/*
 * OtherSubrs:
 *
 *  arg0 arg1 ... argn n othersubr# callothersubr
 *
 *   0: Build flex:
 *      fd x y 3 0 callothersubr
 *      Othersubr #0 pushes x and y to PostScript interpreter operand stack.
 *   1: Start flex
 *      0 1 callothersubr
 *   2: Mark flex control points
 *      0 2 callothersubr
 *   3: Discard hint
 *      subr# 1 3 callothersubr pop callsubr
 *      Subroutine subr# (only) contains stem declaration.
 *      Othersubr #3 pushes subr# to PostScript interpreter operand stack.
 *  12: Counter control
 *      A subr to avoid stack overflow.
 *  13: Counter control
 */

/*
 * Convert six control points marked as CS_FLEX_CTRL to a flex path.
 */
static void
do_othersubr0 (t1_chardesc *cd)
{
  t1_cpath *flex, *cur, *next;

  if (ps_stack_top < 1) {
    status = CS_PARSE_ERROR;
    return;
  }

  /* Seek first CS_FLEX_CTRL mark */
  for (cur = cd->charpath; cur != NULL && cur->type != CS_FLEX_CTRL; cur = cur->next);
  flex = cur;
  {
    int i;
    cur = cur->next;
    for (i = 1; i < 7; i++) {
      if (cur == NULL || cur->type != CS_FLEX_CTRL ||
          cur->num_args != 2) {
        status = CS_PARSE_ERROR;
        return;
      }
      if (i == 1) {
        flex->args[0] += cur->args[0];
        flex->args[1] += cur->args[1];
      } else {
        copy_args(&(flex->args[2*i-2]), cur->args, 2);
      }
      next = cur->next;
      RELEASE(cur);
      cur = next;
    }
  }
  if (cur != NULL) {
    status = CS_PARSE_ERROR;
    return;
  }
  /*
   * Now 'flex' have all six control points, the first pair is relative
   * from starting point.
   */
  flex->type = cs_flex;
  flex->args[12] = ps_arg_stack[--ps_stack_top]; /* flex depth */
  flex->num_args = 13;
  flex->next   = NULL;
  cd->lastpath = flex;

  phase = T1_CS_PHASE_PATH;
}

/* Start flex */
static void
do_othersubr1 (void)
{
  phase = T1_CS_PHASE_FLEX;
}

/* Mark flex control point */
static void
do_othersubr2 (t1_chardesc *cd)
{
  if (phase != T1_CS_PHASE_FLEX || !cd->lastpath) {
    status = CS_PARSE_ERROR;
    return;
  }

  switch (cd->lastpath->type) {
  case cs_rmoveto:
    break;
  case cs_hmoveto:
    cd->lastpath->num_args = 2;
    cd->lastpath->args[1] = 0.0;
    break;
  case cs_vmoveto:
    cd->lastpath->num_args = 2;
    cd->lastpath->args[1] = cd->lastpath->args[0];
    cd->lastpath->args[0] = 0.0;
    break;
  default:
    status = CS_PARSE_ERROR;
    return;
  }
  cd->lastpath->type = CS_FLEX_CTRL;
}

/*
 * Hint Replacement:
 *  "Adobe Type 1 Font Format", Chapter 8.
 */
static void
do_othersubr3 (t1_chardesc *cd)
{
  cd->flags |= T1_CS_FLAG_USE_HINTMASK;
}

static void
do_othersubr12 (void)
{
  /* Othersubr12 call must immediately follow the hsbw or sbw. */
  if (phase != T1_CS_PHASE_INIT) {
    status = CS_PARSE_ERROR;
    return;
  }
  /* noop */
}

static void
do_othersubr13 (t1_chardesc *cd)
{
  t1_stemgroup stemgroups[CS_STEM_GROUP_MAX];
  int num_hgroups, num_vgroups, n, stem_id;
  double pos, del;

  /* After #12 callothersubr or hsbw or sbw. */
  if (phase != T1_CS_PHASE_INIT) {
    status = CS_PARSE_ERROR;
    return;
  }
  for (n = 0; n < CS_STEM_GROUP_MAX; n++) {
    stemgroups[n].num_stems = 0;
  }

  num_hgroups = (int) ps_arg_stack[--ps_stack_top];
  if (num_hgroups < 0 || num_hgroups > CS_STEM_GROUP_MAX) {
    status = CS_PARSE_ERROR;
    return;
  }
  n = 0; pos = 0.0;
  while (ps_stack_top >= 2 && n < num_hgroups) {
    /* add_stem() add sidebearing */
    pos += ps_arg_stack[--ps_stack_top];
    del  = ps_arg_stack[--ps_stack_top];
    stem_id = add_stem(cd,
                       (del < 0.0) ? pos + del : pos,
                       (del < 0.0) ? -del : del,
                       HSTEM);
    stemgroups[n].stems[stemgroups[n].num_stems] = stem_id;
    stemgroups[n].num_stems += 1;
    pos += del;
    if (del < 0.0) {
      pos = 0.0;
      n++;
    }
  }
  if (n != num_hgroups) {
    status = CS_STACK_ERROR;
    return;
  }

  num_vgroups = (int) ps_arg_stack[--ps_stack_top];
  if (num_vgroups < 0 || num_vgroups > CS_STEM_GROUP_MAX) {
    status = CS_PARSE_ERROR;
    return;
  }
  n = 0; pos = 0.0;
  while (ps_stack_top >= 2 && n < num_vgroups) {
    /* add_stem() add sidebearing */
    pos += ps_arg_stack[--ps_stack_top];
    del  = ps_arg_stack[--ps_stack_top];
    stem_id = add_stem(cd,
                       (del < 0.0) ? pos + del : pos,
                       (del < 0.0) ? -del : del,
                       VSTEM);
    stemgroups[n].stems[stemgroups[n].num_stems] = stem_id;
    stemgroups[n].num_stems += 1;
    pos += del;
    if (del < 0.0) {
      pos = 0.0;
      n++;
    }
  }
  if (n != num_vgroups) {
    status = CS_STACK_ERROR;
    return;
  }

  for (n = 0; n < MAX(num_hgroups, num_vgroups); n++) {
    add_charpath(cd, cs_cntrmask,
                 stemgroups[n].stems, stemgroups[n].num_stems);
  }

  cd->flags |= T1_CS_FLAG_USE_CNTRMASK;
}

static void
do_callothersubr (t1_chardesc *cd)
{
  int argn, subrno;

  CHECKSTACK(2);
  subrno = (int) cs_arg_stack[--cs_stack_top];
  argn   = (int) cs_arg_stack[--cs_stack_top];

  CHECKSTACK(argn);
  if (ps_stack_top+argn > PS_ARG_STACK_MAX) {
    status = CS_PARSE_ERROR;
    return;
  }
  while (argn-- > 0)
    ps_arg_stack[ps_stack_top++] = cs_arg_stack[--cs_stack_top];

  switch (subrno) {
  case 0:  do_othersubr0(cd) ; break;
  case 1:  do_othersubr1()   ; break;
  case 2:  do_othersubr2(cd) ; break;
  case 3:  do_othersubr3(cd) ; break;
  case 12: do_othersubr12()  ; break;
  case 13: do_othersubr13(cd); break;
  default:
    ERROR("Unknown othersubr #%ld.", subrno);
    break;
  }
}

/*
 * Double byte operators:
 */
static void
do_operator2 (t1_chardesc *cd, card8 **data, card8 *endptr)
{
  card8 op;

  *data += 1;

  SRC_NEED(endptr, *data + 1);

  op = **data;
  *data += 1;

  switch(op) {
  case cs_sbw:
    CHECKSTACK(4);
    cd->sbw.wy  = cs_arg_stack[--cs_stack_top];
    cd->sbw.wx  = cs_arg_stack[--cs_stack_top];
    cd->sbw.sby = cs_arg_stack[--cs_stack_top];
    cd->sbw.sbx = cs_arg_stack[--cs_stack_top];
    CLEARSTACK();
    break;
  case cs_hstem3:
  case cs_vstem3:
    /*
     * TODO:
     *  The counter control can be used for hstem3 and vstem3
     *  operator if LanguageGroup is not equal to 1.
     */
    CHECKSTACK(6);
    {
      int i;
      for (i = 2; i >= 0; i--) {
        int stem_id;
        stem_id = add_stem(cd,
                           cs_arg_stack[cs_stack_top-2*i-2],
                           cs_arg_stack[cs_stack_top-2*i-1],
                           ((op == cs_hstem3) ? HSTEM : VSTEM));
        if (stem_id < 0) {
          WARN("Too many hints...");
          status = CS_PARSE_ERROR;
          return;
        }
        /* Put stem_id onto the stack... */
        cs_arg_stack[cs_stack_top++] = stem_id;
        ADD_PATH(cd, CS_HINT_DECL, 1);
        cs_stack_top--;
      }
    }
    CLEARSTACK();
    break;
  case cs_setcurrentpoint:
    CHECKSTACK(2);
    /* noop */
    CLEARSTACK();
    break;
    /* all operator above are stack-clearing */
  case cs_pop:
    /*
     * Transfer a operand from PS interpreter operand stack to BuildChar
     * operand stack.
     */
    if (ps_stack_top < 1) {
      status = CS_PARSE_ERROR;
      return;
    }
    LIMITCHECK(1);
    cs_arg_stack[cs_stack_top++] = ps_arg_stack[--ps_stack_top];
    break;
  case cs_dotsection:
#if 0
    /*
     * If the hint replacement feature is used in the font, the
     * "dotsection" operator exist only for compatibility to older
     * (more than 10 years old) Type 1 font rasterizer which can't
     * perform hint replacement. In this case, we silently ignore
     * the "dotsection" operator.
     *
     * The following code will wrongly warn about "dotsection" when
     * the charstring only contains dot (e.g., "bullet") where the
     * hint replacement is not necessary.
     *
     * Adobe ATM renderers always treat this operator as a no-op.
     * (See, Adobe Technical Note #5177, Appendix C)
     */
    if (!(cd->flags & T1_CS_FLAG_USE_HINTMASK)) {
      if (__verbose > 1)
        WARN("Obsolete Type 1 charstring operator \"dotsection\" not supported.");
    }
#endif
    /* noop */
    break;
  case cs_div: /* TODO: check overflow */
    CHECKSTACK(2);
    cs_arg_stack[cs_stack_top-2] /= cs_arg_stack[cs_stack_top-1];
    cs_stack_top--;
    break;
  case cs_callothersubr:
    do_callothersubr(cd);
    break;
  case cs_seac:
    CHECKSTACK(5);
    cd->flags |= T1_CS_FLAG_USE_SEAC;
    cd->seac.achar = (card8) cs_arg_stack[--cs_stack_top];
    cd->seac.bchar = (card8) cs_arg_stack[--cs_stack_top];
    cd->seac.ady   = cs_arg_stack[--cs_stack_top];
    cd->seac.adx   = cs_arg_stack[--cs_stack_top];
    /* We must compensate the difference of the glyph origin. */
    cd->seac.ady += cd->sbw.sby;
    cd->seac.adx += cd->sbw.sbx - cs_arg_stack[--cs_stack_top];
    CLEARSTACK();
    break;
  default:
    /* no-op ? */
    WARN("Unknown charstring operator: 0x0c%02x", op);
    status = CS_PARSE_ERROR;
    break;
  }

  return;
}

/*
 * Charstring encoding:
 *  Copied from cs_type2.c
 *  Note:
 *   The Type 2 interpretation of a number encoded in five-bytes (those with
 *   an initial byte value of 255) differs from how it is interpreted in the
 *   Type 1 format.
 */

/* Type 2 5-bytes encoding used. */
static void
put_numbers (double *argv, int argn, card8 **dest, card8 *limit)
{
  int i;

  for (i = 0; i < argn; i++) {
    double value;
    int    ivalue;
    value  = argv[i];
    /* Nearest integer value */
    ivalue = (int) floor(value+0.5);
    if (value >= 0x8000L || value <= (-0x8000L - 1)) {
      /*
       * This number cannot be represented as a single operand.
       * We must use `a b mul ...' or `a c div' to represent large values.
       */
      ERROR("Argument value too large. (This is bug)");
    } else if (fabs(value - ivalue) > 3.0e-5) {
      /* 16.16-bit signed fixed value  */
      DST_NEED(limit, *dest + 5);
      *(*dest)++ = 255;
      ivalue = (int) floor(value); /* mantissa */
      *(*dest)++ = (ivalue >> 8) & 0xff;
      *(*dest)++ = ivalue & 0xff;
      ivalue = (int)((value - ivalue) * 0x10000l); /* fraction */
      *(*dest)++ = (ivalue >> 8) & 0xff;
      *(*dest)++ = ivalue & 0xff;
      /* Everything else are integers. */
    } else if (ivalue >= -107 && ivalue <= 107) {
      DST_NEED(limit, *dest + 1);
      *(*dest)++ = ivalue + 139;
    } else if (ivalue >= 108 && ivalue <= 1131) {
      DST_NEED(limit, *dest + 2);
      ivalue = 0xf700u + ivalue - 108;
      *(*dest)++ = (ivalue >> 8) & 0xff;
      *(*dest)++ = ivalue & 0xff;
    } else if (ivalue >= -1131 && ivalue <= -108) {
      DST_NEED(limit, *dest + 2);
      ivalue = 0xfb00u - ivalue - 108;
      *(*dest)++ = (ivalue >> 8) & 0xff;
      *(*dest)++ = ivalue & 0xff;
    } else if (ivalue >= -32768 && ivalue <= 32767) { /* shortint */
      DST_NEED(limit, *dest + 3);
      *(*dest)++ = 28;
      *(*dest)++ = (ivalue >> 8) & 0xff;
      *(*dest)++ = (ivalue) & 0xff;
    } else { /* Shouldn't come here */
      ERROR("Unexpected error.");
    }
  }

  return;
}

static void
get_integer (card8 **data, card8 *endptr)
{
  int result = 0;
  card8 b0 = **data, b1, b2;

  *data += 1;

  if (b0 == 28) { /* shortint */
    SRC_NEED(endptr, *data + 2);
    b1 = **data;
    b2 = *(*data+1);
    result = b1*256+b2;
    if (result > 0x7fff)
      result -= 0x10000L;
    *data += 2;
  } else if (b0 >= 32 && b0 <= 246) { /* int (1) */
    result = b0 - 139;
  } else if (b0 >= 247 && b0 <= 250) { /* int (2) */
    SRC_NEED(endptr, *data + 1);
    b1 = **data;
    result = (b0-247)*256+b1+108;
    *data += 1;
  } else if (b0 >= 251 && b0 <= 254) {
    SRC_NEED(endptr, *data + 1);
    b1 = **data;
    result = -(b0-251)*256-b1-108;
    *data += 1;
  } else {
    status = CS_PARSE_ERROR;
    return;
  }

  LIMITCHECK(1);
  cs_arg_stack[cs_stack_top++] = (double) result;

  return;
}

/* Type 1 */
static void
get_longint (card8 **data, card8 *endptr)
{
  int  result = 0;
  int  i;

  *data += 1;
  SRC_NEED(endptr, *data + 4);
  result = **data;
  if (result >= 0x80L)
    result -= 0x100L;
  *data += 1;
  for (i = 1; i < 4; i++) {
    result = result*256 + (**data);
    *data += 1;
  }

  LIMITCHECK(1);
  cs_arg_stack[cs_stack_top++] = (double) result;

  return;
}

/*
 * TODO:
 *  Check "seac"
 *   We cannot do backword parsing due to subroutine, div etc.
 */

/* Parse charstring and build charpath. */
static void
t1char_build_charpath (t1_chardesc *cd,
                       card8 **data, card8 *endptr, cff_index *subrs)
{
  card8 b0 = 0, *subr;
  int len;

  if (nest > CS_SUBR_NEST_MAX)
    ERROR("Subroutine nested too deeply.");

  nest++;
  while (*data < endptr && status == CS_PARSE_OK) {
    b0 = **data;
    if (b0 == 255) {
      get_longint(data, endptr); /* Type 1 */
    } else if (b0 == cs_return) {
      status = CS_SUBR_RETURN;
    } else if (b0 == cs_callsubr) {
      if (cs_stack_top < 1) {
        status = CS_STACK_ERROR;
      } else {
        int idx;

        idx = cs_arg_stack[--cs_stack_top];
        if (!subrs || idx >= subrs->count)
          ERROR("Invalid Subr#.");
        subr = subrs->data + subrs->offset[idx] - 1;
        len  = subrs->offset[idx+1] - subrs->offset[idx];
        t1char_build_charpath(cd, &subr, subr+len, subrs);
        *data += 1;
      }
    } else if (b0 == cs_escape) {
      do_operator2(cd, data, endptr);
    } else if (b0 < 32 && b0 != 28) { /* 19, 20 need mask */
      do_operator1(cd, data);
    } else if ((b0 <= 22 && b0 >= 27) || b0 == 31) { /* reserved */
      status = CS_PARSE_ERROR; /* not an error ? */
    } else { /* integer */
      get_integer(data, endptr);
    }
  }

  if (status == CS_SUBR_RETURN) {
    status = CS_PARSE_OK;
  } else if (status == CS_CHAR_END && *data < endptr) {
    if (!(*data == endptr - 1 && **data == cs_return))
      WARN("Garbage after endchar. (%d bytes)", (int) (endptr - *data));
  } else if (status < CS_PARSE_OK) { /* error */
    ERROR("Parsing charstring failed: (status=%d, stack=%d)", status, cs_stack_top);
  }

  nest--;

  return;
}

/*
 * Calculate BoundingBox and compress path.
 *  The essentials of PDF size reduction is not Type 2 charstring compression
 *  but Type 1 charstring encryption. Encryption makes lossless compression
 *  useless. We will only do very simple charstring compression.
 */
static void
do_postproc (t1_chardesc *cd)
{
  int i;
  t1_cpath *cur, *prev, *next;
  double x, y;

  if (!cd->charpath)
    return;

  /* Set dummy large value. */
  cd->bbox.llx = cd->bbox.lly =  100000.0;
  cd->bbox.urx = cd->bbox.ury = -100000.0;

  cur  = cd->charpath;
  prev = NULL;
  x = y = 0.0;

#define UPDATE_BBOX(b,x,y) do {\
  if ((b).llx > (x)) (b).llx = (x);\
  if ((b).urx < (x)) (b).urx = (x);\
  if ((b).lly > (y)) (b).lly = (y);\
  if ((b).ury < (y)) (b).ury = (y);\
} while (0)
#define TRY_COMPACT (prev && cur && ((prev->num_args + cur->num_args) < CS_ARG_STACK_MAX))

  while (cur != NULL) {
    next = cur->next;
    switch (cur->type) {
    case cs_rmoveto:
      x += cur->args[0]; y += cur->args[1];
      UPDATE_BBOX(cd->bbox, x, y);
      break;
    case cs_rlineto:
      x += cur->args[0]; y += cur->args[1];
      UPDATE_BBOX(cd->bbox, x, y);
      if (TRY_COMPACT) {
        if (prev->type == cs_rlineto) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        } else if (prev->type == cs_rrcurveto) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->type = cs_rcurveline;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_hmoveto:
      x += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      break;
    case cs_hlineto:
      x += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      if (TRY_COMPACT) {
        if ((prev->type == cs_vlineto && (prev->num_args % 2) == 1) ||
            (prev->type == cs_hlineto && (prev->num_args % 2) == 0)) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_vmoveto:
      y += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      break;
    case cs_vlineto:
      y += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      if (TRY_COMPACT) {
        if ((prev->type == cs_hlineto && (prev->num_args % 2) == 1) ||
            (prev->type == cs_vlineto && (prev->num_args % 2) == 0)) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_rrcurveto:
      for (i = 0; i < 3; i++) {
        x += cur->args[2*i]; y += cur->args[2*i+1];
        UPDATE_BBOX(cd->bbox, x, y);
      }
      if (TRY_COMPACT) {
        if (prev->type == cs_rrcurveto) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        } else if (prev->type == cs_rlineto) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->type = cs_rlinecurve;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_vhcurveto:
      y += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      x += cur->args[1]; y += cur->args[2];
      UPDATE_BBOX(cd->bbox, x, y);
      x += cur->args[3];
      UPDATE_BBOX(cd->bbox, x, y);
      if (TRY_COMPACT) {
        if ((prev->type == cs_hvcurveto && ((prev->num_args / 4) % 2) == 1) ||
            (prev->type == cs_vhcurveto && ((prev->num_args / 4) % 2) == 0)) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_hvcurveto:
      x += cur->args[0];
      UPDATE_BBOX(cd->bbox, x, y);
      x += cur->args[1]; y += cur->args[2];
      UPDATE_BBOX(cd->bbox, x, y);
      y += cur->args[3];
      UPDATE_BBOX(cd->bbox, x, y);
      if (TRY_COMPACT) {
        if ((prev->type == cs_vhcurveto && ((prev->num_args / 4) % 2) == 1) ||
            (prev->type == cs_hvcurveto && ((prev->num_args / 4) % 2) == 0)) {
          copy_args(prev->args+prev->num_args, cur->args, cur->num_args);
          prev->num_args += cur->num_args;
          prev->next = next;
          RELEASE(cur); cur = NULL;
        }
      }
      break;
    case cs_flex:
      for (i = 0; i < 6; i++) {
        x += cur->args[2*i]; y += cur->args[2*1+1];
        UPDATE_BBOX(cd->bbox, x, y);
      }
      if (cur->args[12] == 50.0) {
        if (cur->args[1] == 0.0 && cur->args[11] == 0.0 &&
            cur->args[5] == 0.0 && cur->args[7] == 0.0 &&
            cur->args[3] + cur->args[9] == 0.0) {
          /* cur->args[0] = cur->args[0];  dx1 */
          cur->args[1] = cur->args[2];  /* dx2 */
          cur->args[2] = cur->args[3];  /* dy2 */
          cur->args[3] = cur->args[4];  /* dx3 */
          cur->args[4] = cur->args[6];  /* dx4 */
          cur->args[5] = cur->args[8];  /* dx5 */
          cur->args[6] = cur->args[10]; /* dx6 */
          cur->num_args = 7;
          cur->type = cs_hflex;
        } else if (cur->args[5] == 0.0 && cur->args[7] == 0.0 &&
                   (cur->args[1] + cur->args[3] +
                    cur->args[9] + cur->args[11]) == 0) {
          /* cur->args[0] = cur->args[0];  dx1 */
          /* cur->args[1] = cur->args[1];  dy1 */
          /* cur->args[2] = cur->args[2];  dx2 */
          /* cur->args[3] = cur->args[3];  dy2 */
          /* cur->args[4] = cur->args[4];  dx3 */
          cur->args[5] = cur->args[6];  /* dx4 */
          cur->args[6] = cur->args[8];  /* dx5 */
          cur->args[7] = cur->args[9];  /* dy5 */
          cur->args[8] = cur->args[10]; /* dx6 */
          cur->num_args = 9;
          cur->type = cs_hflex1;
        }
      }
      break;
    case CS_HINT_DECL:
    case cs_cntrmask:
      /* noop */
      break;
    default:
      ERROR("Unexpected Type 2 charstring command %d.", cur->type);
      break;
    }
    if (cur != NULL)
      prev = cur;
    cur = next;
  }

  /* Had no path. Fix lower-left point. */
  if (cd->bbox.llx > cd->bbox.urx)
    cd->bbox.llx = cd->bbox.urx = cd->sbw.wx;
  if (cd->bbox.lly > cd->bbox.ury)
    cd->bbox.lly = cd->bbox.ury = cd->sbw.wy;

  return;
}

#define RESET_STATE() do {\
  status = CS_PARSE_OK;\
  phase  = T1_CS_PHASE_INIT;\
  nest   = 0;\
  ps_stack_top = 0;\
} while (0)

int
t1char_get_metrics (card8 *src, int srclen, cff_index *subrs, t1_ginfo *ginfo)
{
  t1_chardesc t1char, *cd;

  cd = &t1char;
  init_charpath(cd);
  RESET_STATE();
  CLEARSTACK();
  t1char_build_charpath(cd, &src, src+srclen, subrs);
  if (cs_stack_top != 0 || ps_stack_top != 0)
    WARN("Stack not empty. (%d, %d)", cs_stack_top, ps_stack_top);
  do_postproc(cd);
  if (ginfo) {
    ginfo->wx = cd->sbw.wx;
    ginfo->wy = cd->sbw.wy;
    ginfo->bbox.llx = cd->bbox.llx;
    ginfo->bbox.lly = cd->bbox.lly;
    ginfo->bbox.urx = cd->bbox.urx;
    ginfo->bbox.ury = cd->bbox.ury;
    if (cd->flags & T1_CS_FLAG_USE_SEAC) {
      ginfo->use_seac = 1;
      ginfo->seac.adx = cd->seac.adx;
      ginfo->seac.ady = cd->seac.ady;
      ginfo->seac.bchar = cd->seac.bchar;
      ginfo->seac.achar = cd->seac.achar;
    } else {
      ginfo->use_seac = 0;
    }
  }
  release_charpath(cd);

  return 0;
}

#define CHECK_BUFFER(n) if (dst+(n) >= endptr) {\
  ERROR("Buffer overflow.");\
}
#define CHECK_STATUS()  if (status != CS_PARSE_OK) {\
  ERROR("Charstring encoder error: %d", status);\
}

/*
 * Encode Charpath as a Type 2 Charstring
 */
static int
t1char_encode_charpath (t1_chardesc *cd,
                        double default_width, double nominal_width,
                        card8 *dst, card8 *endptr)
{
  card8    *save;
  t1_cpath *curr;

  ASSERT(cd);

  save = dst;
  curr = cd->charpath;

  RESET_STATE();
  CLEARSTACK(); 
  /*
   * Advance Width
   */
  if (cd->sbw.wx != default_width) {
    double wx = cd->sbw.wx - nominal_width;
    put_numbers(&wx, 1, &dst, endptr);
    CHECK_STATUS();
  }
  /*
   * Hint Declaration
   */
  {
    int num_hstems = 0, num_vstems = 0;
    int i, reset = 1;
    double stem[2];

    for (i = 0; i < cd->num_stems && cd->stems[i].dir == HSTEM; i++) {
      num_hstems++;
      stem[0] = (reset ?
                 (cd->stems[i].pos) :
                 (cd->stems[i].pos - (cd->stems[i-1].pos + cd->stems[i-1].del)));
      stem[1] = cd->stems[i].del;
      put_numbers(stem, 2, &dst, endptr);
      CHECK_STATUS();
      reset = 0;
      if (2*num_hstems > CS_ARG_STACK_MAX - 3) {
        CHECK_BUFFER(1);
        *dst++ = (card8) ((cd->flags & T1_CS_FLAG_USE_HINTMASK) ? cs_hstemhm : cs_hstem);
        reset = 1;
      }
    }
    if (reset == 0) {
      CHECK_BUFFER(1);
      *dst++ = (card8) ((cd->flags & T1_CS_FLAG_USE_HINTMASK) ? cs_hstemhm : cs_hstem);
    }
    reset = 1;
    if (cd->num_stems - num_hstems > 0) {
      for (i = num_hstems; i < cd->num_stems; i++) {
        num_vstems++;
        stem[0] = (reset ?
                   (cd->stems[i].pos) :
                   (cd->stems[i].pos - (cd->stems[i-1].pos + cd->stems[i-1].del)));
        stem[1] = cd->stems[i].del;
        put_numbers(stem, 2, &dst, endptr);
        CHECK_STATUS();
        reset = 0;
        if (2*num_vstems > CS_ARG_STACK_MAX - 3) {
          CHECK_BUFFER(1);
          *dst++ = (card8) ((cd->flags & T1_CS_FLAG_USE_HINTMASK) ? cs_vstemhm : cs_vstem);
          reset = 1;
        }
      }
      if (reset == 0) {
        CHECK_BUFFER(1);
        if ((cd->flags & T1_CS_FLAG_USE_HINTMASK) ||
            (cd->flags & T1_CS_FLAG_USE_CNTRMASK)) {
          /*
           * The vstem hint operator can be ommited if hstem and vstem hints
           * are both declared at the beginning of a charstring, and is
           * followed directly by the hintmask or cntrmask operators.
           */
          if (curr->type != CS_HINT_DECL &&
              curr->type != cs_cntrmask) {
            *dst++ = (card8) cs_vstemhm;
          }
        } else {
          *dst++ = (card8) cs_vstem;
        }
      }
    }
  }
  /*
   * Path Construction and Hint Replacement
   */
  while (curr != NULL && curr->type != cs_endchar) {
    switch (curr->type) {
    case CS_HINT_DECL:
      {
        card8 hintmask[(CS_STEM_ZONE_MAX+7)/8];

        memset(hintmask, 0, (cd->num_stems+7)/8);
        while (curr != NULL && curr->type == CS_HINT_DECL) {
          int stem_idx;

          stem_idx = get_stem(cd, (int) curr->args[0]);
          ASSERT(stem_idx < cd->num_stems);
          hintmask[stem_idx/8] |= (1 << (7 - (stem_idx % 8)));
          curr = curr->next;
        }
        if (cd->flags & T1_CS_FLAG_USE_HINTMASK) {
          CHECK_BUFFER((cd->num_stems+7)/8 + 1);
          *dst++ = (card8) cs_hintmask;
          memcpy(dst, hintmask, (cd->num_stems+7)/8);
          dst += (cd->num_stems+7)/8;
        }
      }
      break;
    case cs_cntrmask:
      {
        card8 cntrmask[(CS_STEM_ZONE_MAX+7)/8];
        int   stem_idx, i;

        memset(cntrmask, 0, (cd->num_stems+7)/8);
        for (i = 0; i < curr->num_args; i++) {
          stem_idx = get_stem(cd, (int) curr->args[i]);
          ASSERT(stem_idx < cd->num_stems);
          cntrmask[stem_idx/8] |= (1 << (7 - (stem_idx % 8)));
        }
        CHECK_BUFFER((cd->num_stems+7)/8 + 1);
        *dst++ = (card8) cs_cntrmask;
        memcpy(dst, cntrmask, (cd->num_stems+7)/8);
        dst += (cd->num_stems+7)/8;
        curr = curr->next;
      }
      break;
    case cs_rmoveto: case cs_hmoveto: case cs_vmoveto:
    case cs_rlineto: case cs_hlineto: case cs_vlineto:
    case cs_rrcurveto:  case cs_hvcurveto: case cs_vhcurveto:
    case cs_rlinecurve: case cs_rcurveline:
      {
        put_numbers(curr->args, curr->num_args, &dst, endptr);
        CHECK_STATUS();
        CHECK_BUFFER(1);
        *dst++ = (card8) curr->type;
        curr = curr->next;
      }
      break;
    case cs_flex: case cs_hflex:
    case cs_hflex1:
      {
        put_numbers(curr->args, curr->num_args, &dst, endptr);
        CHECK_STATUS();
        CHECK_BUFFER(2);
        *dst++ = (card8) cs_escape;
        *dst++ = (card8) curr->type;
        curr = curr->next;
      }
      break;
    default:
      ERROR("Unknown Type 2 charstring command: %d", curr->type);
      break;
    }
  }

  /*
   * (adx ady bchar achar) endchar
   */
  if (cd->flags & T1_CS_FLAG_USE_SEAC) {
    double seac[4];
    seac[0] = cd->seac.adx;
    seac[1] = cd->seac.ady;
    seac[2] = cd->seac.bchar;
    seac[3] = cd->seac.achar;
    put_numbers(seac, 4, &dst, endptr);
    CHECK_STATUS();
    CHECK_BUFFER(2);
    WARN("Obsolete four arguments of \"endchar\" will be used for Type 1 \"seac\" operator.");
  }
  CHECK_BUFFER(1);
  *dst++ = (card8) cs_endchar;

  return (int) (dst - save);
}

int
t1char_convert_charstring (card8 *dst, int dstlen,
                           card8 *src, int srclen, cff_index *subrs,
                           double default_width, double nominal_width,
                           t1_ginfo *ginfo)
{
  int length;
  t1_chardesc t1char, *cd;

  cd = &t1char;
  init_charpath(cd);
  RESET_STATE();
  CLEARSTACK();
  t1char_build_charpath(cd, &src, src+srclen, subrs);
  if (cs_stack_top != 0 || ps_stack_top != 0)
    WARN("Stack not empty. (%d, %d)", cs_stack_top, ps_stack_top);
  do_postproc(cd);
  SORT_STEMS(cd);

  length = t1char_encode_charpath(cd, default_width, nominal_width, dst, dst+dstlen);

  if (ginfo) {
    ginfo->wx = cd->sbw.wx;
    ginfo->wy = cd->sbw.wy;
    ginfo->bbox.llx = cd->bbox.llx;
    ginfo->bbox.lly = cd->bbox.lly;
    ginfo->bbox.urx = cd->bbox.urx;
    ginfo->bbox.ury = cd->bbox.ury;
    if (cd->flags & T1_CS_FLAG_USE_SEAC) {
      ginfo->use_seac = 1;
      ginfo->seac.adx = cd->seac.adx;
      ginfo->seac.ady = cd->seac.ady;
      ginfo->seac.bchar = cd->seac.bchar;
      ginfo->seac.achar = cd->seac.achar;
    } else {
      ginfo->use_seac = 0;
    }
  }
  release_charpath(cd);

  return length;
}
