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
#include "dpx-dpxutil.h"
#include "dpx-error.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-mpost.h"
#include "dpx-numbers.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-specials.h"


/* pdfcolorstack:
 * This special is provoded as a compatibility feature to pdftex.
 */
struct stack
{
    int       page;
    int       direct;
    dpx_stack stack;
};

#define PDFCOLORSTACK_MAX_STACK 256
struct spc_stack
{
    struct stack stacks[PDFCOLORSTACK_MAX_STACK];
};

static struct spc_stack spc_stack;

static int
pdfcolorstack__get_id (struct spc_env *spe, int *id, struct spc_arg *args)
{
    char *q;

    if (args->curptr >= args->endptr) {
        spc_warn(spe, "Stack ID number expected but not found.");
        return -1;
    }
    q = parse_number(&args->curptr, args->endptr);
    if (!q) {
        spc_warn(spe, "Stack ID number expected but not found.");
        return -1;
    }
    *id = atoi(q);
    free(q);

    skip_white(&args->curptr, args->endptr);

    return 0;
}

static int
pdfcolorstack__init (void *dp)
{
    struct spc_stack *sd = dp;
    int  i;

    for (i = 0; i < PDFCOLORSTACK_MAX_STACK; i++) {
        sd->stacks[i].page    = 0;
        sd->stacks[i].direct  = 0;
        dpx_stack_init(&sd->stacks[i].stack);
    }

    return 0;
}

static int
pdfcolorstack__clean (void *dp)
{
    struct spc_stack *sd = dp;
    int  i;

    for (i = 0; i < PDFCOLORSTACK_MAX_STACK; i++) {
        pdf_obj   *litstr;
        dpx_stack *stk = &sd->stacks[i].stack;

        while ((litstr = dpx_stack_pop(stk)) != NULL) {
            pdf_release_obj(litstr);
        }
    }

    return 0;
}

static void
pdfcolorstack__set_litstr (pdf_coord cp, pdf_obj *litstr, int direct)
{
    pdf_tmatrix M;

    if (!litstr)
        return;

    if (!direct) {
        M.a = M.d = 1.0; M.b = M.c = 0.0;
        M.e = cp.x; M.f = cp.y;
        pdf_dev_concat(&M);
    }
    pdf_doc_add_page_content(" ", 1);
    pdf_doc_add_page_content(pdf_string_value(litstr), pdf_string_length(litstr));
    if (!direct) {
        M.e = -cp.x; M.f = -cp.y;
        pdf_dev_concat(&M);
    }
}

static int
spc_handler_pdfcolorstackinit (struct spc_env *spe, struct spc_arg *args)
{
    int           id = -1;
    struct stack *st;
    char         *q;
    pdf_coord     cp = {0.0, 0.0};
    pdf_obj      *litstr;

    skip_white(&args->curptr, args->endptr);
    if (args->curptr >= args->endptr)
        return -1;

    if (pdfcolorstack__get_id(spe, &id, args) < 0)
        return -1;
    if (id < 0 || id >= PDFCOLORSTACK_MAX_STACK) {
        spc_warn(spe, "Invalid stack number specified: %d", id);
        return -1;
    }
    skip_white(&args->curptr, args->endptr);

    st = &spc_stack.stacks[id];
    if (dpx_stack_depth(&st->stack) > 0) {
        spc_warn(spe, "Stadk ID=%d already initialized?", id);
        return -1;
    }

    while ((q = parse_c_ident(&args->curptr, args->endptr)) != NULL) {
        if (!strcmp(q, "page")) {
            st->page = 1;
        } else if (!strcmp(q, "direct")) {
            st->direct = 1;
        } else {
            spc_warn(spe, "Ignoring unknown option for pdfcolorstack special (init): %s", q);
        }
        free(q);
        skip_white(&args->curptr, args->endptr);
    }

    if (args->curptr < args->endptr) {
        litstr = parse_pdf_string(&args->curptr, args->endptr);
        if (litstr) {
            dpx_stack_push(&st->stack, litstr);
            pdfcolorstack__set_litstr(cp, litstr, st->direct);
        }
        skip_white(&args->curptr, args->endptr);
    }  else {
        spc_warn(spe, "No valid PDF literal specified.");
        return -1;
    }

    return 0;
}

static int
pdfcolorstack__set (struct spc_env *spe, struct stack *st, pdf_coord cp, struct spc_arg *args)
{
    pdf_obj *litstr;

    skip_white(&args->curptr, args->endptr);
    if (args->curptr >= args->endptr)
        return -1;

    litstr = dpx_stack_pop(&st->stack);
    if (!litstr) {
        spc_warn(spe, "Stack empty!");
        return -1;
    }
    pdf_release_obj(litstr);

    litstr = parse_pdf_string(&args->curptr, args->endptr);
    if (litstr) {
        dpx_stack_push(&st->stack, litstr);
        pdfcolorstack__set_litstr(cp, litstr, st->direct);
        skip_white(&args->curptr, args->endptr);
    }

    return 0;
}

static int
pdfcolorstack__push (struct spc_env *spe, struct stack *st, pdf_coord cp, struct spc_arg *args)
{
    pdf_obj *litstr;

    skip_white(&args->curptr, args->endptr);
    if (args->curptr >= args->endptr)
        return -1;

    litstr = parse_pdf_string(&args->curptr, args->endptr);
    if (litstr) {
        dpx_stack_push(&st->stack, litstr);
        pdfcolorstack__set_litstr(cp, litstr, st->direct);
        skip_white(&args->curptr, args->endptr);
    }

    return 0;
}

static int
pdfcolorstack__current (struct spc_env *spe, struct stack *st, pdf_coord cp, struct spc_arg *args)
{
    pdf_obj *litstr;

    litstr = dpx_stack_top(&st->stack);
    if (litstr) {
        pdfcolorstack__set_litstr(cp, litstr, st->direct);
        skip_white(&args->curptr, args->endptr);
    } else {
        spc_warn(spe, "Stack empty!");
        return -1;
    }

    return 0;
}

static int
pdfcolorstack__pop (struct spc_env *spe, struct stack *st, pdf_coord cp, struct spc_arg *args)
{
    int      error = 0;
    pdf_obj *litstr;

    /* "default" at the bottom */
    if (dpx_stack_depth(&st->stack) < 2) {
        spc_warn(spe, "Stack underflow");
        return -1;
    }
    litstr = dpx_stack_pop(&st->stack);
    if (litstr) {
        pdf_release_obj(litstr);
    }
    litstr = dpx_stack_top(&st->stack);
    if (litstr) {
        pdfcolorstack__set_litstr(cp, litstr, st->direct);
    }

    return error;
}

static int
spc_handler_pdfcolorstack (struct spc_env *spe, struct spc_arg *args)
{
    int           error = 0;
    int           id;
    char          *command;
    struct stack *st;
    pdf_coord     cp;

    skip_white(&args->curptr, args->endptr);
    if (args->curptr >= args->endptr)
        return -1;

    if (pdfcolorstack__get_id(spe, &id, args) < 0)
        return -1;
    if (id < 0 || id >= PDFCOLORSTACK_MAX_STACK) {
        spc_warn(spe, "Invalid stack ID specified: %d", id);
        return -1;
    }
    skip_white(&args->curptr, args->endptr);

    st = &spc_stack.stacks[id];
    if (dpx_stack_depth(&st->stack) < 1) {
        spc_warn(spe, "Stack ID=%d not properly initialized?", id);
        return -1;
    }

    command = parse_c_ident(&args->curptr, args->endptr);
    if (!command)
        return -1;

    spc_get_current_point(spe, &cp);
    if (!strcmp(command, "set")) {
        error = pdfcolorstack__set(spe, st, cp, args);
    } else if (!strcmp(command, "push")) {
        error = pdfcolorstack__push(spe, st, cp, args);
    } else if (!strcmp(command, "pop")) {
        error = pdfcolorstack__pop(spe, st, cp, args);
    } else if (!strcmp(command, "current")) {
        error = pdfcolorstack__current(spe, st, cp, args);
    } else {
        spc_warn(spe, "Unknown action: %s", command);
    }

    if (error) {
        spc_warn(spe, "Error occurred while processing pdfcolorstack: id=%d command=\"%s\"", id, command);
    }

    free(command);

    return error;
}

/* Duplicate from spc_pdfm */
static pdf_obj *
parse_pdf_reference (const char **start, const char *end, void *user_data)
{
    pdf_obj *result = NULL;
    char    *name;

    skip_white(start, end);
    name = parse_opt_ident(start, end);
    if (name) {
        result = spc_lookup_reference(name);
        if (!result) {
            dpx_warning("Could not find the named reference (@%s).", name);
        }
        free(name);
    } else {
            dpx_warning("Could not find a reference name.");
        result = NULL;
    }

  return result;
}

/* pdffontattr */
#include "dpx-pdffont.h"

struct fontattr {
    char    *ident;
    double   size;
    pdf_obj *attr;
};

struct fontattr *fontattrs = NULL;
int num_fontattrs = 0;
int max_fontattrs = 0;

static int
process_fontattr (const char *ident, double size, pdf_obj *attr)
{
    int      font_id;
    pdf_obj *fontdict;

    assert(ident && attr);

    font_id = pdf_font_findresource(ident, size);
    if (font_id < 0) {
        dpx_warning("Could not find specified font resource: %s (%gpt)", ident, size);
        return -1;
    }

    fontdict = pdf_get_font_resource(font_id);
    if (!fontdict) {
        dpx_warning("Specified object not exist: %s (%gpt)", ident, size);
        return  -1;
    }

    pdf_merge_dict(fontdict, attr);

    return 0;
}

static int
spc_handler_pdffontattr (struct spc_env *spe, struct spc_arg *ap)
{
    struct fontattr *fontattr;
    char            *ident = NULL;
    double           size  = 0.0;
    pdf_obj         *attr  = NULL;

    skip_white(&ap->curptr, ap->endptr);
    if (ap->curptr >= ap->endptr)
        return -1;

    ident = parse_ident(&ap->curptr, ap->endptr);
    if (!ident) {
        spc_warn(spe, "Missing a font name.");
        return -1;
    }
    skip_white(&ap->curptr, ap->endptr);

    if (ap->curptr < ap->endptr && ap->curptr[0] != '<') {
        int error = dpx_util_read_length(&size, 1.0, &ap->curptr, ap->endptr);
        if (error) {
            spc_warn(spe, "Font size expected but not found.");
            free(ident);
            return -1;
        }
        skip_white(&ap->curptr, ap->endptr);
    }

    attr = parse_pdf_object_extended(&ap->curptr, ap->endptr, NULL, parse_pdf_reference, spe);
    if (!attr) {
        spc_warn(spe, "Failed to parse a PDF dictionary object: %s", ident);
        free(ident);
        return -1;
    } else if (!PDF_OBJ_DICTTYPE(attr)) {
        spc_warn(spe, "PDF dict expected but non-dict object found: %s", ident);
        free(ident);
        pdf_release_obj(attr);
        return -1;
    }
    skip_white(&ap->curptr, ap->endptr);

    if (num_fontattrs >= max_fontattrs) {
        fontattrs = RENEW(fontattrs, max_fontattrs + 256, struct fontattr);
        max_fontattrs += 256;
    }
    fontattr = &fontattrs[num_fontattrs];
    num_fontattrs += 1;

    fontattr->ident = ident;
    fontattr->size  = size;
    fontattr->attr  = attr;

    return 0;
}

int
spc_misc_at_begin_document (void)
{
    struct spc_stack *sd = &spc_stack;

    if (!fontattrs) {
        fontattrs = NEW(256, struct fontattr);
        num_fontattrs = 0;
        max_fontattrs = 256;
    }

    return pdfcolorstack__init(sd);
}

int
spc_misc_at_end_document (void)
{
    struct spc_stack *sd = &spc_stack;

    if (fontattrs) {
        int i;

        for (i = 0; i < num_fontattrs; i++) {
            process_fontattr(fontattrs[i].ident, fontattrs[i].size, fontattrs[i].attr);
            free(fontattrs[i].ident);
            pdf_release_obj(fontattrs[i].attr);
        }
        free(fontattrs);
        fontattrs = NULL;
        max_fontattrs = num_fontattrs = 0;
    }

    return pdfcolorstack__clean(sd);
}

int
spc_misc_at_begin_page (void)
{
    struct spc_stack *sd = &spc_stack;
    int  i;

    for (i = 0; i < PDFCOLORSTACK_MAX_STACK; i++) {
        dpx_stack *stk = &sd->stacks[i].stack;

        if (sd->stacks[i].page) {
            pdf_obj   *litstr = dpx_stack_top(stk);
            pdf_coord  cp     = {0.0, 0.0};

            if (litstr)
                pdfcolorstack__set_litstr(cp, litstr, sd->stacks[i].direct);
        }
    }

    return 0;
}

int
spc_misc_at_begin_form (void)
{
    return spc_misc_at_begin_page();
}

int
spc_misc_at_end_form (void)
{
    return spc_misc_at_begin_page();
}

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

    form_id = pdf_ximage_load_image(NULL, filename, options);
    if (form_id < 0) {
        spc_warn(spe, "Failed to load image file: %s", filename);
        return  -1;
    }

    spc_put_image(spe, form_id, &ti, spe->x_user, spe->y_user);
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
    {"pdfcolorstackinit", spc_handler_pdfcolorstackinit},
    {"pdfcolorstack", spc_handler_pdfcolorstack},
    {"pdffontattr", spc_handler_pdffontattr},
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
    for (i = 0; i < sizeof(misc_handlers)/sizeof(struct spc_handler); i++) {
        if (size >= strlen(misc_handlers[i].key) &&
            !strncmp(p, misc_handlers[i].key, strlen(misc_handlers[i].key))) {
            return true;
        }
    }

    return false;
}

int
spc_misc_setup_handler (struct spc_handler *handle, struct spc_env *spe, struct spc_arg *args)
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

    for (i = 0; i < sizeof(misc_handlers)/sizeof(struct spc_handler); i++) {
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
