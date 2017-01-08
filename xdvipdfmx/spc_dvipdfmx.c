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

#ifdef HAVE_CONFIG_H
#include <config.h>
#endif

#include <string.h>

#include "system.h"
#include "mem.h"
#include "error.h"

#include "dpxutil.h"
#include "pdfparse.h"

#include "specials.h"

#include "spc_util.h"
#include "spc_dvipdfmx.h"

static int
spc_handler_null (struct spc_env *spe, struct spc_arg *args)
{
  args->curptr = args->endptr;

  return 0;
}

static struct spc_handler dvipdfmx_handlers[] = {
  {"config", spc_handler_null}, /* handled at bop */
};

int
spc_dvipdfmx_check_special (const char *buf, int len)
{
  int    r = 0;
  const char *p, *endptr;

  p      = buf;
  endptr = p + len;

  skip_white(&p, endptr);
  if (p + strlen("dvipdfmx:") <= endptr &&
      !memcmp(p, "dvipdfmx:", strlen("dvipdfmx:"))) {
    r = 1;
  }

  return  r;
}

int
spc_dvipdfmx_setup_handler (struct spc_handler *sph,
			    struct spc_env *spe, struct spc_arg *ap)
{
  int    error = -1, i;
  char  *q;

  ASSERT(sph && spe && ap);

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
      if (!strcmp(q, dvipdfmx_handlers[i].key)) {
        ap->command = dvipdfmx_handlers[i].key;
        sph->key   = "dvipdfmx:";
        sph->exec  = dvipdfmx_handlers[i].exec;
        skip_white(&ap->curptr, ap->endptr);
        error = 0;
        break;
      }
    }
    RELEASE(q);
  }

  return  error;
}
