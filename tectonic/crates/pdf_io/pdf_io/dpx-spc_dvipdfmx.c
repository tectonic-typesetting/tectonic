/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

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

#include "dpx-spc_dvipdfmx.h"

#include <assert.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-pdfparse.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"

static int
spc_handler_null (struct spc_env *spe, struct spc_arg *args)
{
  args->curptr = args->endptr;

  return 0;
}

static int
spc_handler_dvipdfmx_catch_phantom (struct spc_env *spe, struct spc_arg *args)
{
  int mode, error;

  skip_white(&args->curptr, args->endptr);
  {
    pdf_obj *b = parse_pdf_boolean(&args->curptr, args->endptr);
    if (!b) {
      dpx_warning("A boolean value expected but not found...");
      return -1;
    }
    mode = pdf_boolean_value(b);
    pdf_release_obj(b);
  }
  spc_set_linkmode(spe, mode);

  skip_white(&args->curptr, args->endptr);
  if (mode == 1 && args->curptr < args->endptr) {
    transform_info ti;

    transform_info_clear(&ti);
    error = spc_util_read_dimtrns(spe, &ti, args, 0);
    if (error)
      return -1;
    if (ti.flags & INFO_HAS_HEIGHT) {
      spc_set_phantom(spe, ti.height, ti.depth);
    }
    skip_white(&args->curptr, args->endptr);
  }

  return 0;
}

static struct spc_handler dvipdfmx_handlers[] = {
  {"config",        spc_handler_null}, /* handled at bop */
  {"catch_phantom", spc_handler_dvipdfmx_catch_phantom},
};

bool
spc_dvipdfmx_check_special (const char *buf, int len)
{
  const char *p, *endptr;

  p      = buf;
  endptr = p + len;

  skip_white(&p, endptr);
  if (p + strlen("dvipdfmx:") <= endptr &&
      !memcmp(p, "dvipdfmx:", strlen("dvipdfmx:"))) {
    return true;
  }

  return false;
}

int
spc_dvipdfmx_setup_handler (struct spc_handler *sph,
                            struct spc_env *spe, struct spc_arg *ap)
{
  int    error = -1;
  size_t i;
  char  *q;

  assert(sph && spe && ap);

  skip_white(&ap->curptr, ap->endptr);
  if (ap->curptr + strlen("dvipdfmx:") >= ap->endptr ||
      memcmp(ap->curptr, "dvipdfmx:", strlen("dvipdfmx:"))) {
    spc_warn(spe, "Not dvipdfmx: special???");
    return  -1;
  }
  ap->curptr += strlen("dvipdfmx:");

  skip_white(&ap->curptr, ap->endptr);
  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (q) {
    for (i = 0;
         i < sizeof(dvipdfmx_handlers) / sizeof(struct spc_handler); i++) {
      if (streq_ptr(q, dvipdfmx_handlers[i].key)) {
        ap->command = dvipdfmx_handlers[i].key;
        sph->key   = "dvipdfmx:";
        sph->exec  = dvipdfmx_handlers[i].exec;
        skip_white(&ap->curptr, ap->endptr);
        error = 0;
        break;
      }
    }
    free(q);
  }

  return  error;
}
