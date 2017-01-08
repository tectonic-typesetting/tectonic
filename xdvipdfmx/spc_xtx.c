/*  This is xdvipdfmx, an extended version of dvipdfmx,
    an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2013-2016 by the dvipdfmx project team.

    Copyright (c) 2006 SIL International
    Originally written by Jonathan Kew

    This file based on spc_pdfm.c, part of the dvipdfmx project:

    Copyright (C) 2002 by Jin-Hwan Cho and Shunsaku Hirata.    

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

#include <ctype.h>
#include <math.h>

#include "system.h"
#include "mem.h"
#include "error.h"
#include "mfileio.h"

#include "numbers.h"

#include "fontmap.h"
#include "dpxutil.h"

#include "pdfobj.h"
#include "pdfparse.h"

#include "pdfdoc.h"

#include "pdfdraw.h"
#include "pdfcolor.h"
#include "pdfdev.h"

#include "specials.h"

#include "spc_util.h"
#include "spc_xtx.h"


int
spc_handler_xtx_do_transform (double x_user, double y_user, double a, double b, double c, double d, double e, double f)
{
  pdf_tmatrix     M = { 0, 0, 0, 0, 0, 0 };
  pdf_coord       pt;

  /* Create transformation matrix */
  M.a = a;
  M.b = b;
  M.c = c;
  M.d = d;
  M.e = ((1.0 - M.a) * x_user - M.c * y_user) + e;
  M.f = ((1.0 - M.d) * y_user - M.b * x_user) + f;

  pdf_dev_concat(&M);
  pdf_dev_get_fixed_point(&pt);
  pdf_dev_set_fixed_point(x_user - pt.x, y_user - pt.y);

  return  0;
}

static int
spc_handler_xtx_scale (struct spc_env *spe, struct spc_arg *args)
{
  double          values[2];

  if (spc_util_read_numbers(&values[0], 2, args) < 2) {
    return -1;
  }
  args->curptr = args->endptr;

  return spc_handler_xtx_do_transform(spe->x_user, spe->y_user, values[0], 0, 0, values[1], 0, 0);
}

/* Scaling without gsave/grestore. */
static pdf_coord *scaleFactors = 0;
static int scaleFactorCount = -1;

static int
spc_handler_xtx_bscale (struct spc_env *spe, struct spc_arg *args)
{
  double          values[2];

  if (!(++scaleFactorCount & 0x0f))
    scaleFactors = realloc(scaleFactors, (scaleFactorCount + 16) * sizeof(pdf_coord));
  if (spc_util_read_numbers(&values[0], 2, args) < 2) {
    return -1;
  }
  if (fabs(values[0]) < 1.e-7 || fabs(values[1]) < 1.e-7) {
    return -1;
  }
  scaleFactors[scaleFactorCount].x = 1 / values[0];
  scaleFactors[scaleFactorCount].y = 1 / values[1];
  args->curptr = args->endptr;

  return  spc_handler_xtx_do_transform (spe->x_user, spe->y_user, values[0], 0, 0, values[1], 0, 0);
}

static int
spc_handler_xtx_escale (struct spc_env *spe, struct spc_arg *args)
{
  pdf_coord factor = scaleFactors[scaleFactorCount--];

  args->curptr = args->endptr;

  return  spc_handler_xtx_do_transform (spe->x_user, spe->y_user, factor.x, 0, 0, factor.y, 0, 0);
}

static int
spc_handler_xtx_rotate (struct spc_env *spe, struct spc_arg *args)
{
  double          value;

  if (spc_util_read_numbers(&value, 1, args) < 1) {
    return -1;
  }
  args->curptr = args->endptr;

  return  spc_handler_xtx_do_transform (spe->x_user, spe->y_user,
      cos(value * M_PI / 180), sin(value * M_PI / 180),
      -sin(value * M_PI / 180), cos(value * M_PI / 180),
      0, 0);
}

int
spc_handler_xtx_gsave (struct spc_env *spe, struct spc_arg *args)
{
  pdf_dev_gsave();

  return  0;
}

int
spc_handler_xtx_grestore (struct spc_env *spe, struct spc_arg *args)
{
  pdf_dev_grestore();

  /*
   * Unfortunately, the following line is necessary in case
   * of a font or color change inside of the save/restore pair.
   * Anything that was done there must be redone, so in effect,
   * we make no assumptions about what fonts. We act like we are
   * starting a new page.
   */
  pdf_dev_reset_fonts(0);
  pdf_dev_reset_color(0);

  return  0;
}

/* Please remove this.
 * This should be handled before processing pages!
 */
static int
spc_handler_xtx_papersize (struct spc_env *spe, struct spc_arg *args)
{
  return  0;
}

static int
spc_handler_xtx_backgroundcolor (struct spc_env *spe, struct spc_arg *args)
{
  int       error;
  pdf_color colorspec;

  error = spc_util_read_colorspec(spe, &colorspec, args, 0);
  if (error)
    spc_warn(spe, "No valid color specified?");
  else {
    pdf_doc_set_bgcolor(&colorspec);
  }

  return  error;
}

/* FIXME: xdv2pdf's x:fontmapline and x:fontmapfile may have slightly different syntax/semantics */
static int
spc_handler_xtx_fontmapline (struct spc_env *spe, struct spc_arg *ap)
{
  fontmap_rec *mrec;
  char        *map_name, opchr;
  int          error = 0;
  static char  buffer[1024];
  const char  *p;
  char        *q;

  skip_white(&ap->curptr, ap->endptr);
  if (ap->curptr >= ap->endptr) {
    spc_warn(spe, "Empty fontmapline special?");
    return  -1;
  }

  opchr = ap->curptr[0];
  if (opchr == '-' || opchr == '+')
    ap->curptr++;

  skip_white(&ap->curptr, ap->endptr);

  switch (opchr) {
  case  '-':
    map_name = parse_ident(&ap->curptr, ap->endptr);
    if (map_name) {
      pdf_remove_fontmap_record(map_name);
      RELEASE(map_name);
    } else {
      spc_warn(spe, "Invalid fontmap line: Missing TFM name.");
      error = -1;
    }
    break;
  default:
    p = ap->curptr;
    q = buffer;
    while (p < ap->endptr)
      *q++ = *p++;
    *q = '\0';
    mrec = NEW(1, fontmap_rec);
    pdf_init_fontmap_record(mrec);
    error = pdf_read_fontmap_line(mrec, buffer, (int) (ap->endptr - ap->curptr), is_pdfm_mapline(buffer));
    if (error)
      spc_warn(spe, "Invalid fontmap line.");
    else if (opchr == '+')
      pdf_append_fontmap_record(mrec->map_name, mrec);
    else
      pdf_insert_fontmap_record(mrec->map_name, mrec);
    pdf_clear_fontmap_record(mrec);
    RELEASE(mrec);
    break;
  }
  if (!error)
    ap->curptr = ap->endptr;

  return 0;
}

static int
spc_handler_xtx_fontmapfile (struct spc_env *spe, struct spc_arg *args)
{
  char  *mapfile;
  int    mode, error = 0;

  skip_white(&args->curptr, args->endptr);
  if (args->curptr >= args->endptr)
    return 0;

  switch (args->curptr[0]) {
  case  '-':
    mode = FONTMAP_RMODE_REMOVE;
    args->curptr++;
    break;
  case  '+':
    mode = FONTMAP_RMODE_APPEND;
    args->curptr++;
    break;
  default:
    mode = FONTMAP_RMODE_REPLACE;
    break;
  }

  mapfile = parse_val_ident(&args->curptr, args->endptr);
  if (!mapfile) {
    spc_warn(spe, "No fontmap file specified.");
    return  -1;
  } else {
    error = pdf_load_fontmap_file(mapfile, mode);
  }

  return  error;
}

static char overlay_name[256];

static int
spc_handler_xtx_initoverlay (struct spc_env *spe, struct spc_arg *args)
{
  skip_white(&args->curptr, args->endptr);
  if (args->curptr >= args->endptr)
    return -1;
  strncpy(overlay_name, args->curptr, args->endptr - args->curptr);
  overlay_name[args->endptr - args->curptr] = 0;

  args->curptr = args->endptr;
  return 0;
}

static int
spc_handler_xtx_clipoverlay (struct spc_env *spe, struct spc_arg *args)
{
  skip_white(&args->curptr, args->endptr);
  if (args->curptr >= args->endptr)
    return -1;
  pdf_dev_grestore();
  pdf_dev_gsave();
  if (strncmp(overlay_name, args->curptr, strlen(overlay_name)) != 0
   && strncmp("all", args->curptr, strlen("all")) != 0)
    pdf_doc_add_page_content(" 0 0 m W n", 10);

  args->curptr = args->endptr;
  return 0;
}

static int
spc_handler_xtx_renderingmode (struct spc_env *spe, struct spc_arg *args)
{
  double value;

  if (spc_util_read_numbers(&value, 1, args) < 1) {
    return -1;
  }
  if ((int) value < 0 || (int) value > 7) {
    spc_warn(spe, "Invalid text rendering mode %d.\n", (int) value);
    return -1;
  }
  sprintf(work_buffer, " %d Tr", (int) value);
  pdf_doc_add_page_content(work_buffer, strlen(work_buffer));
  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {
    pdf_doc_add_page_content(" ", 1);
    pdf_doc_add_page_content(args->curptr, args->endptr - args->curptr);
  }

  args->curptr = args->endptr;
  return 0;
}

static int
spc_handler_xtx_unsupportedcolor (struct spc_env *spe, struct spc_arg *args)
{
  spc_warn(spe, "xetex-style \\special{x:%s} is not supported by this driver;\n"
                "update document or driver to use \\special{color} instead.", args->command);

  args->curptr = args->endptr;

  return 0;
}

static int
spc_handler_xtx_unsupported (struct spc_env *spe, struct spc_arg *args)
{
  spc_warn(spe, "xetex-style \\special{x:%s} is not supported by this driver.", args->command);

  args->curptr = args->endptr;

  return 0;
}

static struct spc_handler xtx_handlers[] = {
  {"textcolor",       spc_handler_xtx_unsupportedcolor},
  {"textcolorpush",   spc_handler_xtx_unsupportedcolor},
  {"textcolorpop",    spc_handler_xtx_unsupportedcolor},

  {"rulecolor",       spc_handler_xtx_unsupportedcolor},
  {"rulecolorpush",   spc_handler_xtx_unsupportedcolor},
  {"rulecolorpop",    spc_handler_xtx_unsupportedcolor},

  {"papersize",       spc_handler_xtx_papersize},
  {"backgroundcolor", spc_handler_xtx_backgroundcolor},

  {"gsave",           spc_handler_xtx_gsave},
  {"grestore",        spc_handler_xtx_grestore},

  {"scale",           spc_handler_xtx_scale},
  {"bscale",           spc_handler_xtx_bscale},
  {"escale",           spc_handler_xtx_escale},
  {"rotate",          spc_handler_xtx_rotate},

  {"fontmapline",     spc_handler_xtx_fontmapline},
  {"fontmapfile",     spc_handler_xtx_fontmapfile},

  {"shadow",          spc_handler_xtx_unsupported},
  {"colorshadow",     spc_handler_xtx_unsupported},
  {"renderingmode",   spc_handler_xtx_renderingmode},

  {"initoverlay",     spc_handler_xtx_initoverlay},
  {"clipoverlay",     spc_handler_xtx_clipoverlay},
};

int
spc_xtx_check_special (const char *buf, int len)
{
  int    r = 0;
  const char *p, *endptr;

  p      = buf;
  endptr = p + len;

  skip_white(&p, endptr);
  if (p + strlen("x:") <= endptr &&
      !memcmp(p, "x:", strlen("x:"))) {
    r = 1;
  }

  return  r;
}

int
spc_xtx_setup_handler (struct spc_handler *sph,
                        struct spc_env *spe, struct spc_arg *ap)
{
  int    error = -1, i;
  char  *q;

  ASSERT(sph && spe && ap);

  skip_white(&ap->curptr, ap->endptr);
  if (ap->curptr + strlen("x:") >= ap->endptr ||
      memcmp(ap->curptr, "x:", strlen("x:"))) {
    spc_warn(spe, "Not x: special???");
    return  -1;
  }
  ap->curptr += strlen("x:");

  skip_white(&ap->curptr, ap->endptr);
  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (q) {
    for (i = 0;
         i < sizeof(xtx_handlers) / sizeof(struct spc_handler); i++) {
      if (!strcmp(q, xtx_handlers[i].key)) {
        ap->command = xtx_handlers[i].key;
        sph->key   = "x:";
        sph->exec  = xtx_handlers[i].exec;
        skip_white(&ap->curptr, ap->endptr);
        error = 0;
        break;
      }
    }
    RELEASE(q);
  }

  return  error;
}

