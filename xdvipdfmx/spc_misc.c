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

#include "system.h"
#include "mem.h"
#include "error.h"

#include "mfileio.h"

#include "pdfparse.h"
#include "pdfobj.h"

#include "pdfcolor.h"
#include "pdfdraw.h"
#include "pdfximage.h"
#include "pdfdev.h"

#include "mpost.h"

#include "specials.h"

#include "spc_util.h"
#include "spc_misc.h"

static int
spc_handler_postscriptbox (struct spc_env *spe, struct spc_arg *ap)
{
  int            form_id, len;
  transform_info ti;
  load_options   options = {1, 0, NULL};
  char           filename[256], *fullname;
  char           buf[512];
  FILE          *fp;

  ASSERT(spe && ap);

  if (ap->curptr >= ap->endptr) {
    spc_warn(spe, "No width/height/filename given for postscriptbox special.");
    return  -1;
  }

  /* input is not NULL terminated */
  len = (int) (ap->endptr - ap->curptr);
  len = MIN(511, len);
  memcpy(buf, ap->curptr, len);
  buf[len] = '\0';

  transform_info_clear(&ti);

  spc_warn(spe, buf);
  if (sscanf(buf, "{%lfpt}{%lfpt}{%255[^}]}",
      &ti.width, &ti.height, filename) != 3) {
    spc_warn(spe, "Syntax error in postscriptbox special?");
    return  -1;
  }
  ap->curptr = ap->endptr;

  ti.width  *= 72.0 / 72.27;
  ti.height *= 72.0 / 72.27;

  fullname = kpse_find_pict(filename);
  if (!fullname) {
    spc_warn(spe, "Image file \"%s\" not found.", filename);
    return  -1;
  }

  fp = MFOPEN(fullname, FOPEN_R_MODE);
  if (!fp) {
    spc_warn(spe, "Could not open image file: %s", fullname);
    RELEASE(fullname);
    return  -1;
  }
  RELEASE(fullname);

  ti.flags |= (INFO_HAS_WIDTH|INFO_HAS_HEIGHT);

  for (;;) {
    const char *p = mfgets(buf, 512, fp);
    if (!p)
      break;
    if (mps_scan_bbox(&p, p + strlen(p), &ti.bbox) >= 0) {
      ti.flags |= INFO_HAS_USER_BBOX;
      break;
    }
  }
  MFCLOSE(fp);

  form_id = pdf_ximage_findresource(filename, options);
  if (form_id < 0) {
    spc_warn(spe, "Failed to load image file: %s", filename);
    return  -1;
  }

  pdf_dev_put_image(form_id, &ti, spe->x_user, spe->y_user);

  return  0;
}

static int
spc_handler_null (struct spc_env *spe, struct spc_arg *args)
{
  args->curptr = args->endptr;

  return 0;
}

static struct spc_handler misc_handlers[] = {
  {"postscriptbox", spc_handler_postscriptbox},
  {"landscape",     spc_handler_null}, /* handled at bop */
  {"papersize",     spc_handler_null}, /* handled at bop */
  {"src:",          spc_handler_null}, /* simply ignore  */
  {"pos:",          spc_handler_null}, /* simply ignore  */
  {"om:",           spc_handler_null}  /* simply ignore  */
};


int
spc_misc_check_special (const char *buffer, int size)
{
  const char *p, *endptr;
  int    i;

  p      = buffer;
  endptr = p + size;

  skip_white(&p, endptr);
  size   = (int) (endptr - p);
  for (i = 0;
       i < sizeof(misc_handlers)/sizeof(struct spc_handler); i++) {
    if (size >= strlen(misc_handlers[i].key) &&
	!strncmp(p, misc_handlers[i].key,
		 strlen(misc_handlers[i].key))) {
      return 1;
    }
  }

  return 0;
}

int
spc_misc_setup_handler (struct spc_handler *handle,
			struct spc_env *spe, struct spc_arg *args)
{
  const char *key;
  int    i, keylen;

  ASSERT(handle && spe && args);

  skip_white(&args->curptr, args->endptr);

  key = args->curptr;
  while (args->curptr < args->endptr &&
	 isalpha((unsigned char)args->curptr[0])) {
    args->curptr++;
  }

  if (args->curptr < args->endptr &&
      args->curptr[0] == ':') {
    args->curptr++;
  }

  keylen = (int) (args->curptr - key);
  if (keylen < 1) {
    return -1;
  }

  for (i = 0;
       i < sizeof(misc_handlers)/sizeof(struct spc_handler); i++) {
    if (keylen == strlen(misc_handlers[i].key) &&
	!strncmp(key, misc_handlers[i].key, keylen)) {

      skip_white(&args->curptr, args->endptr);

      args->command = misc_handlers[i].key;

      handle->key   = "???:";
      handle->exec  = misc_handlers[i].exec;

      return 0;
    }
  }

  return -1;
}
