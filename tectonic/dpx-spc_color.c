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

#include "dpx-spc_color.h"

#include <assert.h>
#include <stdlib.h>

#include "dpx-dpxutil.h"
#include "dpx-pdfcolor.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"


/* Color stack is actually placed into pdfcolor.c.
 * The reason why we need to place stack there is
 * that we must reinstall color after grestore and
 * other operations that can change current color
 * implicitely.
 */

static int
spc_handler_color_push (struct spc_env *spe, struct spc_arg *args)
{
  int        error;
  pdf_color  colorspec;

  error = spc_util_read_colorspec(spe, &colorspec, args, 1);
  if (!error) {
    pdf_color_push(&colorspec, &colorspec);
  }

  return  error;
}

static int
spc_handler_color_pop  (struct spc_env *spe, struct spc_arg *args)
{
  pdf_color_pop();

  return  0;
}

/* Invoked by the special command "color rgb .625 0 0".
 * DVIPS clears the color stack, and then saves and sets the given color.
 */
static int
spc_handler_color_default (struct spc_env *spe, struct spc_arg *args)
{
  int        error;
  pdf_color  colorspec;

  error = spc_util_read_colorspec(spe, &colorspec, args, 1);
  if (!error) {
    pdf_color_clear_stack();
    pdf_color_set(&colorspec, &colorspec);
  }

  return  error;
}


/* This is from color special? */
#include "dpx-pdfdoc.h"

static int
spc_handler_background (struct spc_env *spe, struct spc_arg *args)
{
  int        error;
  pdf_color  colorspec;

  error = spc_util_read_colorspec(spe, &colorspec, args, 1);
  if (!error)
    pdf_doc_set_bgcolor(&colorspec);

  return  error;
}


#ifndef ISBLANK
#define ISBLANK(c) ((c) == ' ' || (c) == '\t' || (c) == '\v')
#endif
static void
skip_blank (const char **pp, const char *endptr)
{
  const char  *p = *pp;
  for ( ; p < endptr && ISBLANK(*p); p++);
  *pp = p;
}

bool
spc_color_check_special (const char *buf, int len)
{
  bool r = false;
  const char *p, *endptr;
  char *q;

  p      = buf;
  endptr = p + len;

  skip_blank(&p, endptr);
  q = parse_c_ident(&p, endptr);
  if (!q)
    return false;
  else if (streq_ptr(q, "color"))
    r = true;
  else if (streq_ptr(q, "background")) {
    r = true;
  }
  free(q);

  return r;
}

int
spc_color_setup_handler (struct spc_handler *sph,
                         struct spc_env *spe, struct spc_arg *ap)
{
  const char *p;
  char *q;

  assert(sph && spe && ap);

  skip_blank(&ap->curptr, ap->endptr);
  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (!q)
    return  -1;
  skip_blank(&ap->curptr, ap->endptr);

  if (streq_ptr(q, "background")) {
    ap->command = "background";
    sph->exec   = &spc_handler_background;
    free(q);
  } else if (streq_ptr(q, "color")) { /* color */
    free(q);
    p = ap->curptr;

    q = parse_c_ident(&p, ap->endptr);
    if (!q)
      return  -1;
    else if (streq_ptr(q, "push")) {
      ap->command = "push";
      sph->exec   = &spc_handler_color_push;
      ap->curptr  = p;
    } else if (streq_ptr(q, "pop")) {
      ap->command = "pop";
      sph->exec   = &spc_handler_color_pop;
      ap->curptr  = p;
    } else { /* cmyk, rgb, ... */
      ap->command = "";
      sph->exec   = &spc_handler_color_default;
    }
    free(q);
  } else {
    spc_warn(spe, "Not color/background special?");
    free(q);
    return  -1;
  }

  skip_blank(&ap->curptr, ap->endptr);
  return  0;
}

