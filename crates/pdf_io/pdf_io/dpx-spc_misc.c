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

#include "dpx-spc_misc.h"

#include <assert.h>
#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

#include "tectonic_bridge_core.h"
#include "dpx-mfileio.h"
#include "dpx-mpost.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-specials.h"


static int
spc_handler_postscriptbox (struct spc_env *spe, struct spc_arg *ap)
{
    int            form_id, len;
    transform_info ti;
    load_options   options = {1, 0, NULL};
    char           filename[256];
    char           buf[512];
    rust_input_handle_t handle;

    assert(spe && ap);

    if (ap->curptr >= ap->endptr) {
        spc_warn(spe, "No width/height/filename given for postscriptbox special.");
        return -1;
    }

    /* input is not NULL terminated */
    len = (int) (ap->endptr - ap->curptr);
    len = MIN(511, len);
    memcpy(buf, ap->curptr, len);
    buf[len] = '\0';

    transform_info_clear(&ti);

    spc_warn(spe, "%s", buf);
    if (sscanf(buf, "{%lfpt}{%lfpt}{%255[^}]}", &ti.width, &ti.height, filename) != 3) {
        spc_warn(spe, "Syntax error in postscriptbox special?");
        return -1;
    }

    ap->curptr = ap->endptr;

    ti.width  *= 72.0 / 72.27;
    ti.height *= 72.0 / 72.27;

    if ((handle = ttstub_input_open(filename, TTBC_FILE_FORMAT_PICT, 0)) == NULL) {
        spc_warn(spe, "Could not open image file: %s", filename);
        return -1;
    }

    ti.flags |= (INFO_HAS_WIDTH|INFO_HAS_HEIGHT);

    for (;;) {
        const char *p = tt_mfgets(buf, 512, handle);
        if (!p)
            break;

        if (mps_scan_bbox(&p, p + strlen(p), &ti.bbox) >= 0) {
            ti.flags |= INFO_HAS_USER_BBOX;
            break;
        }
    }

    ttstub_input_close(handle);

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


bool
spc_misc_check_special (const char *buffer, int size)
{
    const char *p, *endptr;
    size_t i;

    p      = buffer;
    endptr = p + size;

    skip_white(&p, endptr);
    size   = (int) (endptr - p);
    for (i = 0;
         i < sizeof(misc_handlers)/sizeof(struct spc_handler); i++) {
        if (size >= strlen(misc_handlers[i].key) &&
            !strncmp(p, misc_handlers[i].key,
                     strlen(misc_handlers[i].key))) {
            return true;
        }
    }

    return false;
}

int
spc_misc_setup_handler (struct spc_handler *handle,
                        struct spc_env *spe, struct spc_arg *args)
{
    const char *key;
    int keylen;
    size_t i;

    assert(handle && spe && args);

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
