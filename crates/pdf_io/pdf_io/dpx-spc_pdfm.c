/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2007-2020 by Jin-Hwan Cho and Shunsaku Hirata,
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

#include "dpx-spc_pdfm.h"

#include <assert.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "dpx-dpxconf.h"
#include "dpx-dpxfile.h"
#include "dpx-dpxutil.h"
#include "dpx-dvipdfmx.h"
#include "dpx-error.h"
#include "dpx-fontmap.h"
#include "dpx-mem.h"
#include "dpx-mfileio.h"
#include "dpx-pdfcolor.h"
#include "dpx-pdfdev.h"
#include "dpx-pdfdoc.h"
#include "dpx-pdfdraw.h"
#include "dpx-pdfobj.h"
#include "dpx-pdfparse.h"
#include "dpx-pdfximage.h"
#include "dpx-spc_util.h"
#include "dpx-specials.h"
#include "dpx-unicode.h"


/* PLEASE REMOVE THIS */
struct resource_map {
  int   type;
  int   res_id;
};

struct tounicode {
  int       cmap_id;
  int       unescape_backslash;
  pdf_obj  *taintkeys; /* An array of PDF names. */
};

struct spc_pdf_
{
   pdf_obj          *annot_dict;   /* pending annotation dict       */
   int               lowest_level; /* current min level of outlines */
   struct ht_table  *resourcemap;  /* see remark below (somewhere)  */
   struct tounicode  cd;           /* For to-UTF16-BE conversion :( */
   pdf_obj          *pageresources; /* Add to all page resource dict */
};

static struct spc_pdf_  _pdf_stat = {
  NULL,
  255,
  NULL,
  { -1, 0, NULL },
  NULL
};

/* PLEASE REMOVE THIS */
static void
hval_free (void *vp)
{
  free(vp);
}


static int
addresource (struct spc_pdf_ *sd, const char *ident, int res_id)
{
  struct resource_map *r;

  if (!ident || res_id < 0)
    return  -1;

  r = NEW(1, struct resource_map);
  r->type   = 0; /* unused */
  r->res_id = res_id;

  ht_append_table(sd->resourcemap, ident, strlen(ident), r);
  spc_push_object(ident, pdf_ximage_get_reference(res_id));

  return 0;
}

static int
findresource (struct spc_pdf_ *sd, const char *ident)
{
  struct resource_map *r;

  if (!ident)
    return  -1;

  r = ht_lookup_table(sd->resourcemap, ident, strlen(ident));

  return (r ? r->res_id : -1);
}


static int
spc_handler_pdfm__init (void *dp)
{
  struct spc_pdf_ *sd = dp;
  /* The folllowing dictionary entry keys are considered as keys for
   * text strings. Be sure that string object is NOT always a text string.
   */
  static const char *default_taintkeys[] = {
    "Title",   "Author",   "Subject", "Keywords",
    "Creator", "Producer", "Contents", "Subj",
    "TU",      "T",        "TM",        NULL /* EOD */
  };
  int  i;

  sd->annot_dict   = NULL;
  sd->lowest_level = 255;
  sd->resourcemap  = NEW(1, struct ht_table);
  ht_init_table(sd->resourcemap, hval_free);

  sd->cd.taintkeys = pdf_new_array();
  for (i = 0; default_taintkeys[i] != NULL; i++) {
    pdf_add_array(sd->cd.taintkeys,
                  pdf_new_name(default_taintkeys[i]));
  }
  sd->pageresources = NULL;

  return 0;
}

static int
spc_handler_pdfm__clean (void *dp)
{
  struct spc_pdf_ *sd = dp;

  if (sd->annot_dict) {
    dpx_warning("Unbalanced bann and eann found.");
    pdf_release_obj(sd->annot_dict);
  }
  sd->lowest_level = 255;
  sd->annot_dict   = NULL;
  if (sd->resourcemap) {
    ht_clear_table(sd->resourcemap);
    free(sd->resourcemap);
  }
  sd->resourcemap = NULL;

  pdf_release_obj(sd->cd.taintkeys);
  sd->cd.taintkeys = NULL;
  if (sd->pageresources)
    pdf_release_obj(sd->pageresources);
  sd->pageresources = NULL;

  return 0;
}


int
spc_pdfm_at_begin_document (void)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  return  spc_handler_pdfm__init(sd);
}

int
spc_pdfm_at_end_document (void)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  return  spc_handler_pdfm__clean(sd);
}


/* Dvipdfm specials */
static int
spc_handler_pdfm_bop (struct spc_env *spe, struct spc_arg *args)
{
  if (args->curptr < args->endptr) {
    pdf_doc_set_bop_content(args->curptr,
                            (int) (args->endptr - args->curptr));
  }

  args->curptr = args->endptr;

  return 0;
}

static int
spc_handler_pdfm_eop (struct spc_env *spe, struct spc_arg *args)
{
  if (args->curptr < args->endptr) {
    pdf_doc_set_eop_content(args->curptr,
                            (int) (args->endptr - args->curptr));
  }

  args->curptr = args->endptr;

  return 0;
}

#define streamfiltered(o) \
  (pdf_lookup_dict(pdf_stream_dict((o)), "Filter") ? 1 : 0)

/* Why should we have this kind of things? */
static int
safeputresdent (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  char  *key;

  assert(kp && vp && dp);

  key = pdf_name_value(kp);
  if (pdf_lookup_dict(dp, key))
    dpx_warning("Object \"%s\" already defined in dict! (ignored)", key);
  else {
    pdf_add_dict(dp,
                 pdf_link_obj(kp), pdf_link_obj(vp));
  }
  return 0;
}

static int
safeputresdict (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  char    *key;
  pdf_obj *dict;

  assert(kp && vp && dp);

  key  = pdf_name_value(kp);
  dict = pdf_lookup_dict(dp, key);

  if (pdf_obj_typeof(vp) == PDF_INDIRECT) {
    pdf_add_dict(dp, pdf_new_name(key), pdf_link_obj(vp));
  } else if (pdf_obj_typeof(vp) == PDF_DICT) {
    if (dict)
      pdf_foreach_dict(vp, safeputresdent, dict);
    else {
      pdf_add_dict(dp, pdf_new_name(key), pdf_link_obj(vp));
    }
  } else {
    dpx_warning("Invalid type (not DICT) for page/form resource dict entry: key=\"%s\"", key);
    return  -1;
  }

  return 0;
}

static
int putpageresources (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  char *resource_name;

  assert(kp && vp);

  resource_name = pdf_name_value(kp);
  pdf_doc_add_page_resource(dp, resource_name, pdf_link_obj(vp));

  return 0;
}

static
int forallresourcecategory (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  int   r = -1;
  char *category;

  assert(kp && vp);

  category = pdf_name_value(kp);
  switch (pdf_obj_typeof(vp)) {
  case PDF_DICT:
    r = pdf_foreach_dict(vp, putpageresources, category);
    break;
  case PDF_INDIRECT:
    {
      /* In case pdf:pageresouces << /Category @res >> */
      pdf_obj *obj;
      obj = pdf_deref_obj(vp);
      if (!obj) {
        dpx_warning("Can't deref object for page resource: %s", category);
        r = -1;
      } else if (pdf_obj_typeof(obj) != PDF_DICT) {
        dpx_warning("Invalid object type for page resource: %s", category);
        r = -1;
      } else {
        pdf_obj *res_dict, *dict;

        res_dict = pdf_doc_current_page_resources();
        dict     = pdf_lookup_dict(res_dict, category);
        if (!dict) {
          pdf_add_dict(res_dict, pdf_new_name(category), pdf_link_obj(vp));
        } else {
          if (pdf_obj_typeof(dict) == PDF_INDIRECT) {
            dict = pdf_deref_obj(dict);
            pdf_release_obj(dict); /* FIXME: jus to decrement link counter */
          }
#if 1
          /* This will leave garbage (object "res") since object "res"
           * supplied as resource dictionary will have label but we copy the
           * content of res here and never use reference to it.
           */
          pdf_foreach_dict(obj, safeputresdent, dict);
#else
          /* With the below code resource dictionary is replaced by user
           * supplied one, @res. However, there is a problem that all
           * resources including internally generated one may go into single
           * dictionary referenced by @res, and will be visible from any
           * subsequent pages. 
           */
          pdf_foreach_dict(dict, safeputresdent, obj);
          pdf_add_dict(res_dict, pdf_new_name(category), pdf_link_obj(vp));
#endif
        }
        pdf_release_obj(obj);
      }
    }
    break;
  default:
    dpx_warning("Invalid object type for page resource specified for \"%s\"", category);
  }

  return r;
}

int
spc_pdfm_at_end_page (void)
{
  struct spc_pdf_ *sd = &_pdf_stat;

  if (sd->pageresources) {
    pdf_foreach_dict(sd->pageresources, forallresourcecategory, NULL);
  }

  return 0;
}

/* Think what happens if you do
 *
 *  pdf:put @resources << /Font << >> >>
 *
 */
static int
spc_handler_pdfm_put (struct spc_env *spe, struct spc_arg *ap)
{
  pdf_obj  *obj1, *obj2; /* put obj2 into obj1 */
  char     *ident;
  int       error = 0;

  skip_white(&ap->curptr, ap->endptr);

  ident = parse_opt_ident(&ap->curptr, ap->endptr);
  if (!ident) {
    spc_warn(spe, "Missing object identifier.");
    return  -1;
  }
  obj1 = spc_lookup_object(ident);
  if (!obj1) {
    spc_warn(spe, "Specified object not exist: %s", ident);
    free(ident);
    return  -1;
  }
  skip_white(&ap->curptr, ap->endptr);

  obj2 = parse_pdf_object(&ap->curptr, ap->endptr, NULL);
  if (!obj2) {
    spc_warn(spe, "Missing (an) object(s) to put into \"%s\"!", ident);
    free(ident);
    return  -1;
  }

  switch (pdf_obj_typeof(obj1)) {
  case  PDF_DICT:
    if (pdf_obj_typeof(obj2) != PDF_DICT) {
      spc_warn(spe, "Inconsistent object type for \"put\" (expecting DICT): %s", ident);
      error = -1;
    } else {
      if (streq_ptr(ident, "resources"))
        error = pdf_foreach_dict(obj2, safeputresdict, obj1);
      else {
        pdf_merge_dict(obj1, obj2);
      }
    }
    break;

  case  PDF_STREAM:
    if (pdf_obj_typeof(obj2) == PDF_DICT)
      pdf_merge_dict(pdf_stream_dict(obj1), obj2);
    else if (pdf_obj_typeof(obj2) == PDF_STREAM)
    {
      spc_warn(spe, "\"put\" operation not supported for STREAM <- STREAM: %s", ident);
      error = -1;
    }
    else {
      spc_warn(spe, "Invalid type: expecting a DICT or STREAM: %s", ident);
      error = -1;
    }
    break;

  case PDF_ARRAY:
    /* dvipdfm */
    pdf_add_array(obj1, pdf_link_obj(obj2));
    while (ap->curptr < ap->endptr) {
      pdf_obj *obj3 = parse_pdf_object(&ap->curptr, ap->endptr, NULL);
      if (!obj3)
        break;
      pdf_add_array(obj1, obj3);
      skip_white(&ap->curptr, ap->endptr);
    }
    break;

  default:
    spc_warn(spe, "Can't \"put\" object into non-DICT/STREAM/ARRAY type object: %s", ident);
    error = -1;
    break;
  }
  pdf_release_obj(obj2);
  free(ident);

  return  error;
}


/* For pdf:tounicode support
 * This feature is provided for convenience. TeX can't do
 * input encoding conversion.
 */
#include "dpx-cmap.h"

static int
reencodestring (CMap *cmap, pdf_obj *instring)
{
#define WBUF_SIZE 4096
  unsigned char  wbuf[WBUF_SIZE];
  unsigned char *obufcur;
  const unsigned char *inbufcur;
  size_t inbufleft, obufleft;

  if (!cmap || !instring)
    return 0;

  inbufleft = pdf_string_length(instring);
  inbufcur  = pdf_string_value (instring);

  wbuf[0]  = 0xfe;
  wbuf[1]  = 0xff;
  obufcur  = wbuf + 2;
  obufleft = WBUF_SIZE - 2;

  CMap_decode(cmap,
              &inbufcur, &inbufleft,
              &obufcur, &obufleft);

  if (inbufleft > 0) {
    return  -1;
  }

  pdf_set_string(instring, wbuf, WBUF_SIZE - obufleft);

  return 0;
}

static int
maybe_reencode_utf8(pdf_obj *instring)
{
  unsigned char* inbuf;
  int            inlen;
  int            non_ascii = 0;
  const unsigned char* cp;
  unsigned char* op;
  unsigned char  wbuf[WBUF_SIZE];

  if (!instring)
    return 0;

  inlen = pdf_string_length(instring);
  inbuf = pdf_string_value(instring);

  /* check if the input string is strictly ASCII */
  for (cp = inbuf; cp < inbuf + inlen; ++cp) {
    if (*cp > 127) {
      non_ascii = 1;
    }
  }
  if (non_ascii == 0)
    return 0; /* no need to reencode ASCII strings */

  /* Check if the input string is valid UTF8 string
   * This routine may be called against non-text strings.
   * We need to re-encode string only when string is a text string
   * endcoded in UTF8.
   */
  if (!UC_UTF8_is_valid_string(inbuf, inbuf + inlen))
    return 0;
  else if (inbuf[0] == 0xfe && inbuf[1] == 0xff &&
      UC_UTF16BE_is_valid_string(inbuf + 2, inbuf + inlen))
    return 0; /* no need to reencode UTF16BE with BOM */

  cp = inbuf;
  op = wbuf;
  *op++ = 0xfe;
  *op++ = 0xff;
  while (cp < inbuf + inlen) {
    int32_t usv;
    int     len;

    usv = UC_UTF8_decode_char(&cp, inbuf + inlen);
    if (!UC_is_valid(usv))
      return -1; /* out of valid Unicode range, give up (redundant) */
    len = UC_UTF16BE_encode_char(usv, &op, wbuf + WBUF_SIZE);
    if (len == 0)
      return -1;
  }

  pdf_set_string(instring, wbuf, op - wbuf);

  return 0;
}

/* The purpose of this routine is to check if given string object is
 * surely an object for *text* strings. It does not do a complete check
 * but does a quick check. Please add entries for taintkeys if you have found
 * additional dictionary entries which is considered as a text string.
 */
static int
needreencode (pdf_obj *kp, pdf_obj *vp, struct tounicode *cd)
{
  int      r = 0;
  unsigned int i;
  pdf_obj *tk;

  assert( cd && cd->taintkeys );
  assert( pdf_obj_typeof(kp) == PDF_NAME );
  assert( pdf_obj_typeof(vp) == PDF_STRING );

  for (i = 0; i < pdf_array_length(cd->taintkeys); i++) {
    tk = pdf_get_array(cd->taintkeys, i);
    assert( tk && pdf_obj_typeof(tk) == PDF_NAME );
    if (streq_ptr(pdf_name_value(kp), pdf_name_value(tk))) {
      r = 1;
      break;
    }
  }
  if (r) {
    /* Check UTF-16BE BOM. */
    if (pdf_string_length(vp) >= 2 &&
        !memcmp(pdf_string_value(vp), "\xfe\xff", 2))
      r = 0;
  }

  return  r;
}

static int
modstrings (pdf_obj *kp, pdf_obj *vp, void *dp)
{
  int               r = 0; /* continue */
  struct tounicode *cd = dp;

  assert( pdf_obj_typeof(kp) == PDF_NAME );

  switch (pdf_obj_typeof(vp)) {
  case  PDF_STRING:
    if (cd && cd->cmap_id >= 0 && cd->taintkeys) {
      CMap *cmap = CMap_cache_get(cd->cmap_id);
      if (needreencode(kp, vp, cd))
        r = reencodestring(cmap, vp);
    } else if ((dpx_conf.compat_mode == dpx_mode_xdv_mode) && cd && cd->taintkeys) {
      /* Please fix this... PDF string object is not always a text string.
       * needreencode() is assumed to do a simple check if given string
       * object is actually a text string.
       */
      if (needreencode(kp, vp, cd))
        r = maybe_reencode_utf8(vp);
    }
    if (r < 0) /* error occured... */
      dpx_warning("Failed to convert input string to UTF16...");
    break;
  case  PDF_DICT:
    r = pdf_foreach_dict(vp, modstrings, dp);
    break;
  case  PDF_STREAM:
    r = pdf_foreach_dict(pdf_stream_dict(vp), modstrings, dp);
    break;
  }

  return  r;
}

static pdf_obj *
parse_pdf_dict_with_tounicode (const char **pp, const char *endptr, struct tounicode *cd)
{
  pdf_obj  *dict;

  /* disable this test for XDV files, as we do UTF8 reencoding with no cmap */
  if ((dpx_conf.compat_mode != dpx_mode_xdv_mode) && cd->cmap_id < 0)
    return  parse_pdf_dict(pp, endptr, NULL);

  /* :( */
  if (cd && cd->unescape_backslash)
    dict = parse_pdf_tainted_dict(pp, endptr);
  else {
    dict = parse_pdf_dict(pp, endptr, NULL);
  }
  if (dict)
    pdf_foreach_dict(dict, modstrings, cd);

  return  dict;
}

#define SPC_PDFM_SUPPORT_ANNOT_TRANS 1
static int
spc_handler_pdfm_annot (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj       *annot_dict;
  pdf_rect       rect;
  char          *ident = NULL;
  transform_info ti;

  skip_white(&args->curptr, args->endptr);
  if (args->curptr[0] == '@') {
    ident = parse_opt_ident(&args->curptr, args->endptr);
    skip_white(&args->curptr, args->endptr);
  }

  transform_info_clear(&ti);
  if (spc_util_read_dimtrns(spe, &ti, args, 0) < 0) {
    free(ident);
    return  -1;
  }

  if ((ti.flags & INFO_HAS_USER_BBOX) &&
      ((ti.flags & INFO_HAS_WIDTH) || (ti.flags & INFO_HAS_HEIGHT))) {
    spc_warn(spe, "You can't specify both bbox and width/height.");
    free(ident);
    return  -1;
  }

  annot_dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!annot_dict) {
    spc_warn(spe, "Could not find dictionary object.");
    free(ident);
    return  -1;
  } else if (!PDF_OBJ_DICTTYPE(annot_dict)) {
    spc_warn(spe, "Invalid type: not dictionary object.");
    free(ident);
    pdf_release_obj(annot_dict);
    return  -1;
  }

#ifdef SPC_PDFM_SUPPORT_ANNOT_TRANS
  {
    pdf_coord cp1, cp2, cp3, cp4;
    /* QuadPoints not working? */
#ifdef USE_QUADPOINTS
    pdf_obj  *qpoints;
#endif
    if (ti.flags & INFO_HAS_USER_BBOX) {
      cp1.x = spe->x_user + ti.bbox.llx;
      cp1.y = spe->y_user + ti.bbox.lly;
      cp2.x = spe->x_user + ti.bbox.urx;
      cp2.y = spe->y_user + ti.bbox.lly;
      cp3.x = spe->x_user + ti.bbox.urx;
      cp3.y = spe->y_user + ti.bbox.ury;
      cp4.x = spe->x_user + ti.bbox.llx;
      cp4.y = spe->y_user + ti.bbox.ury;
    } else {
      cp1.x = spe->x_user;
      cp1.y = spe->y_user - spe->mag * ti.depth;
      cp2.x = spe->x_user + spe->mag * ti.width;
      cp2.y = spe->y_user - spe->mag * ti.depth;
      cp3.x = spe->x_user + spe->mag * ti.width;
      cp3.y = spe->y_user + spe->mag * ti.height;
      cp4.x = spe->x_user;
      cp4.y = spe->y_user + spe->mag * ti.height;
    }
    pdf_dev_transform(&cp1, NULL);
    pdf_dev_transform(&cp2, NULL);
    pdf_dev_transform(&cp3, NULL);
    pdf_dev_transform(&cp4, NULL);
    rect.llx = cp1.x;
    if (cp2.x < rect.llx)
      rect.llx = cp2.x;
    if (cp3.x < rect.llx)
      rect.llx = cp3.x;
    if (cp4.x < rect.llx)
      rect.llx = cp4.x;
    rect.urx = cp1.x;
    if (cp2.x > rect.urx)
      rect.urx = cp2.x;
    if (cp3.x > rect.urx)
      rect.urx = cp3.x;
    if (cp4.x > rect.urx)
      rect.urx = cp4.x;
    rect.lly = cp1.y;
    if (cp2.y < rect.lly)
      rect.lly = cp2.y;
    if (cp3.y < rect.lly)
      rect.lly = cp3.y;
    if (cp4.y < rect.lly)
      rect.lly = cp4.y;
    rect.ury = cp1.y;
    if (cp2.y > rect.ury)
      rect.ury = cp2.y;
    if (cp3.y > rect.ury)
      rect.ury = cp3.y;
    if (cp4.y > rect.ury)
      rect.ury = cp4.y;
#ifdef USE_QUADPOINTS
    qpoints = pdf_new_array();
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp1.x, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp1.y, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp2.x, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp2.y, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp3.x, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp3.y, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp4.x, 0.01)));
    pdf_add_array(qpoints, pdf_new_number(ROUND(cp4.y, 0.01)));
    pdf_add_dict(annot_dict, pdf_new_name("QuadPoints"), qpoints);
#endif    
  }
#else
  {
    pdf_coord cp;

    cp.x = spe->x_user; cp.y = spe->y_user;
    pdf_dev_transform(&cp, NULL);
    if (ti.flags & INFO_HAS_USER_BBOX) {
      rect.llx = ti.bbox.llx + cp.x;
      rect.lly = ti.bbox.lly + cp.y;
      rect.urx = ti.bbox.urx + cp.x;
      rect.ury = ti.bbox.ury + cp.y;
    } else {
      rect.llx = cp.x;
      rect.lly = cp.y - spe->mag * ti.depth;
      rect.urx = cp.x + spe->mag * ti.width;
      rect.ury = cp.y + spe->mag * ti.height;
    }
  }
#endif

  /* Order is important... */
  if (ident)
    spc_push_object(ident, pdf_link_obj(annot_dict));
  /* Add this reference. */
  pdf_doc_add_annot(pdf_doc_current_page_number(), &rect, annot_dict, 1);

  if (ident) {
    spc_flush_object(ident);
    free(ident);
  }
  pdf_release_obj(annot_dict);

  return 0;
}

/* NOTE: This can't have ident. See "Dvipdfm User's Manual". */
static int
spc_handler_pdfm_bann (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  int    error = 0;

  if (sd->annot_dict) {
    spc_warn(spe, "Can't begin an annotation when one is pending.");
    return  -1;
  }

  skip_white(&args->curptr, args->endptr);

  sd->annot_dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!sd->annot_dict) {
    spc_warn(spe, "Ignoring annotation with invalid dictionary.");
    return  -1;
  } else if (!PDF_OBJ_DICTTYPE(sd->annot_dict)) {
    spc_warn(spe, "Invalid type: not a dictionary object.");
    pdf_release_obj(sd->annot_dict);
    sd->annot_dict = NULL;
    return  -1;
  }

  error = spc_begin_annot(spe, sd->annot_dict);

  return  error;
}

static int
spc_handler_pdfm_eann (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  int    error = 0;

  if (!sd->annot_dict) {
    spc_warn(spe, "Tried to end an annotation without starting one!");
    return  -1;
  }

  error = spc_end_annot(spe);

  pdf_release_obj(sd->annot_dict);
  sd->annot_dict = NULL;

  return  error;
}


/* Color:.... */
static int
spc_handler_pdfm_bcolor (struct spc_env *spe, struct spc_arg *ap)
{
  int       error;
  pdf_color fc, sc;
  pdf_color *pfc, *psc;

  pdf_color_get_current(&psc, &pfc);
  error = spc_util_read_pdfcolor(spe, &fc, ap, pfc);
  if (!error) {
    if (ap->curptr < ap->endptr) {
      error = spc_util_read_pdfcolor(spe, &sc, ap, psc);
    } else {
      pdf_color_copycolor(&sc, &fc);
    }
  }

  if (error)
    spc_warn(spe, "Invalid color specification?");
  else {
    pdf_color_push(&sc, &fc); /* save currentcolor */
  }

  return  error;
}

/*
 * This special changes the current color without clearing the color stack.
 * It therefore differs from "color rgb 1 0 0".
 */
static int
spc_handler_pdfm_scolor (struct spc_env *spe, struct spc_arg *ap)
{
  int       error;
  pdf_color fc, sc;
  pdf_color *pfc, *psc;

  pdf_color_get_current(&psc, &pfc);
  error = spc_util_read_pdfcolor(spe, &fc, ap, pfc);
  if (!error) {
    if (ap->curptr < ap->endptr) {
      error = spc_util_read_pdfcolor(spe, &sc, ap, psc);
    } else {
      pdf_color_copycolor(&sc, &fc);
    }
  }

  if (error)
    spc_warn(spe, "Invalid color specification?");
  else
    pdf_color_set(&sc, &fc);

  return  error;
}

static int
spc_handler_pdfm_ecolor (struct spc_env *spe, struct spc_arg *args)
{
  pdf_color_pop();
  return 0;
}


static int
spc_handler_pdfm_btrans (struct spc_env *spe, struct spc_arg *args)
{
  pdf_tmatrix     M;
  transform_info  ti;

  transform_info_clear(&ti);
  if (spc_util_read_dimtrns(spe, &ti, args, 0) < 0) {
    return -1;
  }

  /* Create transformation matrix */
  pdf_copymatrix(&M, &(ti.matrix));
  M.e += ((1.0 - M.a) * spe->x_user - M.c * spe->y_user);
  M.f += ((1.0 - M.d) * spe->y_user - M.b * spe->x_user);

  pdf_dev_gsave();
  pdf_dev_concat(&M);

  return 0;
}

static int
spc_handler_pdfm_etrans (struct spc_env *spe, struct spc_arg *args)
{
  pdf_dev_grestore();

  /*
   * Unfortunately, the following line is necessary in case
   * of a color change inside of the save/restore pair.
   * (Font changes are automatically corrected by pdf_dev_grestore().)
   * Anything that was done there must be redone, so in effect,
   * we make no assumptions about what fonts. We act like we are
   * starting a new page.
   */
  pdf_dev_reset_color(0);

  return 0;
}

static int
spc_handler_pdfm_outline (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj   *item_dict, *tmp;
  int        level, is_open = -1;
  int        current_depth;

  skip_white(&args->curptr, args->endptr);

  /*
   * pdf:outline is extended to support open/close feature
   *
   * pdf:outline 1 ... (as DVIPDFM)
   * pdf:outline [] 1 ... (open bookmark)
   * pdf:outline [-] 1 ... (closed bookmark)
   */
  if (args->curptr+3 < args->endptr && *args->curptr == '[') {
    args->curptr++;
    if (*args->curptr == '-') {
      args->curptr++;
    } else {
      is_open = 1;
    }
    args->curptr++;
  }
  skip_white(&args->curptr, args->endptr);

  tmp = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!tmp) {
    spc_warn(spe, "Missing number for outline item depth.");
    return  -1;
  } else if (!PDF_OBJ_NUMBERTYPE(tmp)) {
    pdf_release_obj(tmp);
    spc_warn(spe, "Expecting number for outline item depth.");
    return  -1;
  }

  item_dict = NULL;

  level = (int) pdf_number_value(tmp);
  pdf_release_obj(tmp);

  /* What is this? Starting at level 3 and can go down to level 1?
   *
   * Here is the original comment:
   *  Make sure we know where the starting level is
   *
   * NOTE: added
   *  We need this for converting pages from 3rd to... :(
   */
  sd->lowest_level = MIN(sd->lowest_level, level);

  level  +=  1 - sd->lowest_level;

  item_dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!item_dict) {
    spc_warn(spe, "Ignoring invalid dictionary.");
    return  -1;
  }
  current_depth = pdf_doc_bookmarks_depth();
  if (current_depth > level) {
    while (current_depth-- > level)
      pdf_doc_bookmarks_up();
  } else if (current_depth < level) {
    while (current_depth++ < level)
      pdf_doc_bookmarks_down();
  }

  pdf_doc_bookmarks_add(item_dict, is_open);

  return 0;
}

static int
spc_handler_pdfm_article (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  char    *ident;
  pdf_obj *info_dict;

  skip_white (&args->curptr, args->endptr);

  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (!ident) {
    spc_warn(spe,  "Article name expected but not found.");
    return -1;
  }

  info_dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!info_dict) {
    spc_warn(spe, "Ignoring article with invalid info dictionary.");
    free(ident);
    return  -1;
  }

  pdf_doc_begin_article(ident, pdf_link_obj(info_dict));
  spc_push_object(ident, info_dict);
  free(ident);

  return 0;
}

static int
spc_handler_pdfm_bead (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj         *article;
  pdf_obj         *article_info;
  char            *article_name;
  pdf_rect         rect;
  int              page_no;
  transform_info   ti;
  pdf_coord        cp;

  skip_white(&args->curptr, args->endptr);

  if (args->curptr[0] != '@') {
    spc_warn(spe, "Article identifier expected but not found.");
    return  -1;
  }

  article_name = parse_opt_ident(&args->curptr, args->endptr);
  if (!article_name) {
    spc_warn(spe, "Article reference expected but not found.");
    return  -1;
  }

  /* If okay so far, try to get a bounding box */
  transform_info_clear(&ti);
  if (spc_util_read_dimtrns(spe, &ti, args, 0) < 0) {
    free(article_name);
    return  -1;
  }

  if ((ti.flags & INFO_HAS_USER_BBOX) &&
      ((ti.flags & INFO_HAS_WIDTH) || (ti.flags & INFO_HAS_HEIGHT))) {
    spc_warn(spe, "You can't specify both bbox and width/height.");
    free(article_name);
    return -1;
  }

  cp.x = spe->x_user; cp.y = spe->y_user;
  pdf_dev_transform(&cp, NULL);
  if (ti.flags & INFO_HAS_USER_BBOX) {
    rect.llx = ti.bbox.llx + cp.x;
    rect.lly = ti.bbox.lly + cp.y;
    rect.urx = ti.bbox.urx + cp.x;
    rect.ury = ti.bbox.ury + cp.y;
  } else {
    rect.llx = cp.x;
    rect.lly = cp.y - spe->mag * ti.depth;
    rect.urx = cp.x + spe->mag * ti.width;
    rect.ury = cp.y + spe->mag * ti.height;
  }

  skip_white(&args->curptr, args->endptr);
  if (args->curptr[0] != '<') {
    article_info = pdf_new_dict();
  } else {
    article_info = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
    if (!article_info) {
      spc_warn(spe, "Error in reading dictionary.");
      free(article_name);
      return -1;
    }
  }

  /* Does this article exist yet */
  article = spc_lookup_object(article_name);
  if (article) {
    pdf_merge_dict (article, article_info);
    pdf_release_obj(article_info);
  } else {
    pdf_doc_begin_article(article_name, pdf_link_obj(article_info));
    spc_push_object(article_name, article_info);
  }
  page_no = pdf_doc_current_page_number();
  pdf_doc_add_bead(article_name, NULL, page_no, &rect);

  free(article_name);
  return 0;
}

static int
spc_handler_pdfm_image (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  int              xobj_id;
  char            *ident = NULL;
  pdf_obj         *fspec;
  transform_info   ti;
  load_options     options = {1, 0, NULL};

  skip_white(&args->curptr, args->endptr);
  if (args->curptr[0] == '@') {
    ident = parse_opt_ident(&args->curptr, args->endptr);
    xobj_id = findresource(sd, ident);
    if (xobj_id >= 0) {
      spc_warn(spe, "Object reference name for image \"%s\" already used.", ident);
      free(ident);
      return  -1;
    }
  }

  /* 2015/12/29
   * There should not be "page" and "pagebox" in read_dimtrns().
   * It is for reading "dimensions" and "transformations" and "page" is
   * completely unrelated.
   */
  transform_info_clear(&ti);
  if (spc_util_read_blahblah(spe, &ti,
                             &options.page_no, &options.bbox_type, args) < 0) {
    spc_warn(spe, "Reading option field in pdf:image failed.");
    free(ident);
    return  -1;
  }

  skip_white(&args->curptr, args->endptr);
  fspec = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!fspec) {
    spc_warn(spe, "Missing filename string for pdf:image.");
    free(ident);
    return  -1;
  } else if (!PDF_OBJ_STRINGTYPE(fspec)) {
    spc_warn(spe, "Missing filename string for pdf:image.");
    pdf_release_obj(fspec);
    free(ident);
    return  -1;
  }

  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {
    options.dict = parse_pdf_object(&args->curptr, args->endptr, NULL);
  }

  xobj_id = pdf_ximage_findresource(pdf_string_value(fspec), options);

  if (xobj_id < 0) {
    spc_warn(spe, "Could not find image resource...");
    pdf_release_obj(fspec);
    free(ident);
    return  -1;
  }

  if (!(ti.flags & INFO_DO_HIDE))
    pdf_dev_put_image(xobj_id, &ti, spe->x_user, spe->y_user);

  if (ident) {
    addresource(sd, ident, xobj_id);
    free(ident);
  }

  pdf_release_obj(fspec);

  return 0;
}

/* Use do_names instead. */
static int
spc_handler_pdfm_dest (struct spc_env *spe, struct spc_arg *args)
{
  pdf_obj  *name, *array;

  skip_white(&args->curptr, args->endptr);

  name = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!name) {
    spc_warn(spe, "PDF string expected for destination name but not found.");
    return  -1;
  } else if (!PDF_OBJ_STRINGTYPE(name)) {
    spc_warn(spe, "PDF string expected for destination name but invalid type.");
    pdf_release_obj(name);
    return  -1;
  }

  array = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!array) {
    spc_warn(spe, "No destination specified for pdf:dest.");
    pdf_release_obj(name);
    return  -1;
  } else if (!PDF_OBJ_ARRAYTYPE(array)) {
    spc_warn(spe, "Destination not specified as an array object!");
    pdf_release_obj(name);
    pdf_release_obj(array);
    return  -1;
  }

  pdf_doc_add_names("Dests",
                    pdf_string_value (name),
                    pdf_string_length(name),
                    array);
  pdf_release_obj(name);

  return 0;
}

static int
spc_handler_pdfm_names (struct spc_env *spe, struct spc_arg *args)
{
  pdf_obj *category, *key, *value, *tmp;
  int      i, size;

  category = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!category) {
    spc_warn(spe, "PDF name expected but not found.");
    return  -1;
  } else if (!PDF_OBJ_NAMETYPE(category)) {
    spc_warn(spe, "PDF name expected but not found.");
    pdf_release_obj(category);
    return  -1;
  }

  tmp = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!tmp) {
    spc_warn(spe, "PDF object expected but not found.");
    pdf_release_obj(category);
    return  -1;
  } else if (PDF_OBJ_ARRAYTYPE(tmp)) {
    size = pdf_array_length(tmp);
    if (size % 2 != 0) {
      spc_warn(spe, "Array size not multiple of 2 for pdf:names.");
      pdf_release_obj(category);
      pdf_release_obj(tmp);
      return  -1;
    }

    for (i = 0; i < size / 2; i++) {
      key   = pdf_get_array(tmp, 2 * i);
      value = pdf_get_array(tmp, 2 * i + 1);
      if (!PDF_OBJ_STRINGTYPE(key)) {
        spc_warn(spe, "Name tree key must be string.");
        pdf_release_obj(category);
        pdf_release_obj(tmp);
        return -1;
      } else if (pdf_doc_add_names(pdf_name_value(category),
                                   pdf_string_value (key),
                                   pdf_string_length(key),
                                   pdf_link_obj(value)) < 0) {
        spc_warn(spe, "Failed to add Name tree entry...");
        pdf_release_obj(category);
        pdf_release_obj(tmp);
        return -1;
      }
    }
    pdf_release_obj(tmp);
  } else if (PDF_OBJ_STRINGTYPE(tmp)) {
    key   = tmp;
    value = parse_pdf_object(&args->curptr, args->endptr, NULL);
    if (!value) {
      pdf_release_obj(category);
      pdf_release_obj(key);
      spc_warn(spe, "PDF object expected but not found.");
      return -1;
    }
    if (pdf_doc_add_names(pdf_name_value(category),
                          pdf_string_value (key),
                          pdf_string_length(key),
                          value) < 0) {
      spc_warn(spe, "Failed to add Name tree entry...");
      pdf_release_obj(category);
      pdf_release_obj(key);
      return -1;
    }
    pdf_release_obj(key);
  } else {
    pdf_release_obj(tmp);
    pdf_release_obj(category);
    spc_warn(spe, "Invalid object type for pdf:names.");
    return  -1;
  }
  pdf_release_obj(category);

  return 0;
}

static int
spc_handler_pdfm_docinfo (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj *docinfo, *dict;

  dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!dict) {
    spc_warn(spe, "Dictionary object expected but not found.");
    return  -1;
  }

  docinfo = pdf_doc_docinfo();
  pdf_merge_dict(docinfo, dict);
  pdf_release_obj(dict);

  return 0;
}

static int
spc_handler_pdfm_docview (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj   *catalog,  *dict;
  pdf_obj   *pref_old, *pref_add;

  dict = parse_pdf_dict_with_tounicode(&args->curptr, args->endptr, &sd->cd);
  if (!dict) {
    spc_warn(spe, "Dictionary object expected but not found.");
    return  -1;
  }

  catalog  = pdf_doc_catalog();
  /* Avoid overriding whole ViewerPreferences */
  pref_old = pdf_lookup_dict(catalog, "ViewerPreferences");
  pref_add = pdf_lookup_dict(dict,    "ViewerPreferences");
  if (pref_old && pref_add) {
    pdf_merge_dict (pref_old, pref_add);
    pdf_remove_dict(dict, "ViewerPreferences");
  }
  pdf_merge_dict (catalog, dict);
  pdf_release_obj(dict);

  return 0;
}

static int
spc_handler_pdfm_close (struct spc_env *spe, struct spc_arg *args)
{
  char *ident;

  skip_white(&args->curptr, args->endptr);
  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (ident) {
    spc_flush_object(ident);
    free(ident);
  } else { /* Close all? */
    spc_clear_objects();
  }

  return 0;
}

static int
spc_handler_pdfm_object (struct spc_env *spe, struct spc_arg *args)
{
  char    *ident;
  pdf_obj *object;

  skip_white(&args->curptr, args->endptr);
  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (!ident) {
    spc_warn(spe, "Could not find a object identifier.");
    return  -1;
  }

  object = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!object) {
    spc_warn(spe, "Could not find an object definition for \"%s\".", ident);
    free(ident);
    return  -1;
  } else {
    spc_push_object(ident, object);
  }
  free(ident);

  return 0;
}

static int
spc_handler_pdfm_content (struct spc_env *spe, struct spc_arg *args)
{
  int  len = 0;

  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {
    pdf_tmatrix M;

    pdf_setmatrix(&M, 1.0, 0.0, 0.0, 1.0, spe->x_user, spe->y_user);
    work_buffer[len++] = ' ';
    work_buffer[len++] = 'q';
    work_buffer[len++] = ' ';
    len += pdf_sprint_matrix(work_buffer + len, &M);
    work_buffer[len++] = ' ';
    work_buffer[len++] = 'c';
    work_buffer[len++] = 'm';
    work_buffer[len++] = ' ';

    pdf_doc_add_page_content(work_buffer, len);  /* op: q cm */
    len = (int) (args->endptr - args->curptr);
    pdf_doc_add_page_content(args->curptr, len);  /* op: ANY */
    pdf_doc_add_page_content(" Q", 2);  /* op: Q */
  }
  args->curptr = args->endptr;

  return 0;
}

static int
spc_handler_pdfm_literal (struct spc_env *spe, struct spc_arg *args)
{
  int       direct = 0;

  skip_white(&args->curptr, args->endptr);
  while (args->curptr < args->endptr) {
    if (args->curptr + 7 <= args->endptr &&
        strstartswith(args->curptr, "reverse")) {
      args->curptr += 7;
      dpx_warning("The special \"pdf:literal reverse ...\" is no longer supported.\nIgnore the \"reverse\" option.");
    } else if (args->curptr + 6 <= args->endptr &&
               strstartswith(args->curptr, "direct")) {
      direct      = 1;
      args->curptr += 6;
    } else {
      break;
    }
    skip_white(&args->curptr, args->endptr);
  }

  if (args->curptr < args->endptr) {
    pdf_tmatrix M;
    if (!direct) {
      M.a = M.d = 1.0; M.b = M.c = 0.0;
      M.e = spe->x_user; M.f = spe->y_user;
      pdf_dev_concat(&M);
    }
    pdf_doc_add_page_content(" ", 1);  /* op: */
    pdf_doc_add_page_content(args->curptr, (int) (args->endptr - args->curptr));  /* op: ANY */
    if (!direct) {
      M.e = -spe->x_user; M.f = -spe->y_user;
      pdf_dev_concat(&M);
    }
  }

  args->curptr = args->endptr;

  return 0;
}

static int
spc_handler_pdfm_bcontent (struct spc_env *spe, struct spc_arg *args)
{
  pdf_tmatrix M;
  double xpos, ypos;

  pdf_dev_gsave();
  pdf_dev_get_coord(&xpos, &ypos);
  pdf_setmatrix(&M, 1.0, 0.0, 0.0, 1.0, spe->x_user - xpos, spe->y_user - ypos);
  pdf_dev_concat(&M);
  pdf_dev_push_coord(spe->x_user, spe->y_user);
  return 0;
}

static int
spc_handler_pdfm_econtent (struct spc_env *spe, struct spc_arg *args)
{
  pdf_dev_pop_coord();
  pdf_dev_grestore();
  pdf_dev_reset_color(0);

  return 0;
}

static int
spc_handler_pdfm_code (struct spc_env *spe, struct spc_arg *args)
{
  skip_white(&args->curptr, args->endptr);

  if (args->curptr < args->endptr) {
    pdf_doc_add_page_content(" ", 1);  /* op: */
    pdf_doc_add_page_content(args->curptr, (int) (args->endptr - args->curptr));  /* op: ANY */
    args->curptr = args->endptr;
  }

  return 0;
}

static int
spc_handler_pdfm_do_nothing (struct spc_env *spe, struct spc_arg *args)
{
  args->curptr = args->endptr;
  return 0;
}

#define STRING_STREAM 0
#define FILE_STREAM   1

static int
spc_handler_pdfm_stream_with_type (struct spc_env *spe, struct spc_arg *args, int type)
{
  pdf_obj *fstream;
  ssize_t nb_read;
  char    *ident, *instring, *fullname;
  pdf_obj *tmp;
  rust_input_handle_t handle = NULL;

  skip_white(&args->curptr, args->endptr);

  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (!ident) {
    spc_warn(spe, "Missing objname for pdf:(f)stream.");
    return  -1;
  }

  skip_white(&args->curptr, args->endptr);

  tmp = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!tmp) {
    spc_warn(spe, "Missing input string for pdf:(f)stream.");
    free(ident);
    return  -1;
  } else if (!PDF_OBJ_STRINGTYPE(tmp)) {
    spc_warn(spe, "Invalid type of input string for pdf:(f)stream.");
    pdf_release_obj(tmp);
    free(ident);
    return  -1;
  }

  instring = pdf_string_value(tmp);

  switch (type) {
  case FILE_STREAM:
    if (!instring) {
      spc_warn(spe, "Missing filename for pdf:fstream.");
      pdf_release_obj(tmp);
      free(ident);
      return  -1;
    }
    fullname = NULL; /*kpse_find_pict(instring);*/
    if (!fullname) {
      spc_warn(spe, "File \"%s\" not found.", instring);
      pdf_release_obj(tmp);
      free(ident);
      return  -1;
    }
    handle = ttstub_input_open(fullname, TTBC_FILE_FORMAT_PICT, 0);
    if (handle == NULL) {
      spc_warn(spe, "Could not open file: %s", instring);
      pdf_release_obj(tmp);
      free(ident);
      free(fullname);
      return -1;
    }
    fstream = pdf_new_stream(STREAM_COMPRESS);
    while ((nb_read =
            ttstub_input_read(handle, work_buffer, WORK_BUFFER_SIZE)) > 0)
      pdf_add_stream(fstream, work_buffer, nb_read);
    ttstub_input_close(handle);
    free(fullname);
    break;
  case STRING_STREAM:
    fstream = pdf_new_stream(STREAM_COMPRESS);
    pdf_add_stream(fstream, pdf_string_value(tmp), pdf_string_length(tmp));
    break;
  default:
    pdf_release_obj(tmp);
    free(ident);
    return -1;
  }
  pdf_release_obj(tmp);

  /*
   * Optional dict.
   *
   *  TODO: check Length, Filter...
   */
  skip_white(&args->curptr, args->endptr);

  if (args->curptr[0] == '<') {
    pdf_obj *stream_dict;

    stream_dict = pdf_stream_dict(fstream);

    tmp = parse_pdf_dict(&args->curptr, args->endptr, NULL);
    if (!tmp) {
      spc_warn(spe, "Parsing dictionary failed.");
      pdf_release_obj(fstream);
      free(ident);
      return -1;
    }
    if (pdf_lookup_dict(tmp, "Length")) {
      pdf_remove_dict(tmp, "Length");
    } else if (pdf_lookup_dict(tmp, "Filter")) {
      pdf_remove_dict(tmp, "Filter");
    }
    pdf_merge_dict(stream_dict, tmp);
    pdf_release_obj(tmp);
  }

  /* Users should explicitly close this. */
  spc_push_object(ident, fstream);
  free(ident);

  return 0;
}

/*
 * STREAM: Create a PDF stream object from an input string.
 *
 *  pdf: stream @objname (input_string) [PDF_DICT]
 */
static int
spc_handler_pdfm_stream (struct spc_env *spe, struct spc_arg *args)
{
  return spc_handler_pdfm_stream_with_type (spe, args, STRING_STREAM);
}

/*
 * FSTREAM: Create a PDF stream object from an existing file.
 *
 *  pdf: fstream @objname (filename) [PDF_DICT]
 */
static int
spc_handler_pdfm_fstream (struct spc_env *spe, struct spc_arg *args)
{
  return spc_handler_pdfm_stream_with_type (spe, args, FILE_STREAM);
}

/* Grab page content as follows:
 *
 * Reference point = (x_user, y_user)
 *
 * Case 1. \special{pdf:bxobj @obj width WD height HT depth DP}
 *
 *     Grab the box with the lower-left corner (x_user, y_user-DP)
 *     and the upper right corner (x_user+WD, y_user+HT).
 *
 * Case 2. \special{pdf:bxobj @obj bbox LLX LLY URX, URY}
 *
 *     Grab the box with the lower-left corner (x_user+LLX, y_user+LLY)
 *     and the upper right corner (x_user+URX, y_user+URY).
 *
 * Note that scale, xscale, yscale, xoffset, yoffset options are ignored.
 */
static int
spc_handler_pdfm_bform (struct spc_env *spe, struct spc_arg *args)
{
  int             xobj_id;
  char           *ident;
  pdf_rect        cropbox;
  transform_info  ti;

  skip_white(&args->curptr, args->endptr);

  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (!ident) {
    spc_warn(spe, "A form XObject must have name.");
    return  -1;
  }

  transform_info_clear(&ti);
  if (spc_util_read_dimtrns(spe, &ti, args, 0) < 0) {
    free(ident);
    return  -1;
  }

  /* A XForm with zero dimension results in a non-invertible transformation
   * matrix. And it may result in unpredictable behaviour. It might be an
   * error in Acrobat. Bounding box with zero dimension may cause division
   * by zero.
   */
  if (ti.flags & INFO_HAS_USER_BBOX) {
    if (ti.bbox.urx - ti.bbox.llx == 0.0 ||
        ti.bbox.ury - ti.bbox.lly == 0.0) {
      spc_warn(spe, "Bounding box has a zero dimension.");
      free(ident);
      return -1;
    }
    cropbox.llx = ti.bbox.llx;
    cropbox.lly = ti.bbox.lly;
    cropbox.urx = ti.bbox.urx;
    cropbox.ury = ti.bbox.ury;
  } else {
    if (ti.width == 0.0 ||
        ti.depth + ti.height == 0.0) {
      spc_warn(spe, "Bounding box has a zero dimension.");
      free(ident);
      return -1;
    }
    cropbox.llx = 0.0;
    cropbox.lly = -ti.depth;
    cropbox.urx = ti.width;
    cropbox.ury = ti.height;
  }

  xobj_id = pdf_doc_begin_grabbing(ident, spe->x_user, spe->y_user, &cropbox);

  if (xobj_id < 0) {
    free(ident);
    spc_warn(spe, "Couldn't start form object.");
    return -1;
  }

  spc_push_object(ident, pdf_ximage_get_reference(xobj_id));
  free(ident);

  return 0;
}

/* An extra dictionary after exobj must be merged to the form dictionary,
 * not resource dictionary.
 * Please use pdf:put @resources (before pdf:exobj) instead.
 */
static int
spc_handler_pdfm_eform (struct spc_env *spe, struct spc_arg *args)
{
  pdf_obj         *attrib = NULL;
  struct spc_pdf_ *sd     = &_pdf_stat;

  skip_white(&args->curptr, args->endptr);

  if (args->curptr < args->endptr) {
    attrib = parse_pdf_dict(&args->curptr, args->endptr, NULL);
    if (attrib && !PDF_OBJ_DICTTYPE(attrib)) {
      pdf_release_obj(attrib);
      attrib = NULL;
    }
  }
  /* pageresources here too */
  if (sd->pageresources) {
    pdf_foreach_dict(sd->pageresources, forallresourcecategory, NULL);
  }
  pdf_doc_end_grabbing(attrib);

  return 0;
}

/* Saved XObjects can be used as follows:
 *
 * Reference point = (x_user, y_user)
 *
 * Case 1. \special{pdf:uxobj @obj width WD height HT depth DP}
 *
 *     Scale the XObject to fit in the box
 *     [x_user, y_user-DP, x_user+WD, y_user+HT].
 *
 * Case 2. \special{pdf:uxobj @obj xscale XS yscale YS}
 *
 *     Scale the XObject with XS and YS. Note that width and xscale
 *     or height and yscale cannot be used together.
 *
 * Case 3. \special{pdf:bxobj @obj bbox LLX LLY URX, URY}
 *
 *     Scale the XObject to fit in the box
 *     [x_user+LLX, y_user+LLY, x_user+URX, y_user+URY].
 *
 * Note that xoffset and yoffset moves the reference point where the
 * lower-left corner of the XObject will be put.
 */
static int
spc_handler_pdfm_uxobj (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  int              xobj_id;
  char            *ident;
  transform_info   ti;
  load_options     options = {1, 0, NULL};

  skip_white(&args->curptr, args->endptr);

  ident = parse_opt_ident(&args->curptr, args->endptr);
  if (!ident) {
    spc_warn(spe, "No object identifier given.");
    return  -1;
  }

  transform_info_clear(&ti);
  if (args->curptr < args->endptr) {
    if (spc_util_read_dimtrns(spe, &ti, args, 0) < 0) {
      free(ident);
      return  -1;
    }
  }

  /* Dvipdfmx was suddenly changed to use file name to identify
   * external images. We can't use ident to find image resource
   * here.
   */
  xobj_id = findresource(sd, ident);
  if (xobj_id < 0) {
    xobj_id = pdf_ximage_findresource(ident, options);
    if (xobj_id < 0) {
      spc_warn(spe, "Specified (image) object doesn't exist: %s", ident);
      free(ident);
      return  -1;
    }
  }

  pdf_dev_put_image(xobj_id, &ti, spe->x_user, spe->y_user);
  free(ident);

  return 0;
}

static int
spc_handler_pdfm_link (struct spc_env *spe, struct spc_arg *args)
{
  return  spc_resume_annot(spe);
}

static int
spc_handler_pdfm_nolink (struct spc_env *spe, struct spc_arg *args)
{
  return  spc_suspend_annot(spe);
}



/* Handled at BOP */
static int
spc_handler_pdfm_pagesize (struct spc_env *spe, struct spc_arg *args)
{
  args->curptr = args->endptr;

  return 0;
}

/* Please remove this.
 * This should be handled before processing pages!
 */
static int
spc_handler_pdfm_bgcolor (struct spc_env *spe, struct spc_arg *args)
{
  int       error;
  pdf_color colorspec;

  error = spc_util_read_pdfcolor(spe, &colorspec, args, NULL);
  if (error)
    spc_warn(spe, "No valid color specified?");
  else {
    pdf_doc_set_bgcolor(&colorspec);
  }

  return  error;
}

#define THEBUFFLENGTH 1024
static int
spc_handler_pdfm_mapline (struct spc_env *spe, struct spc_arg *ap)
{
  fontmap_rec *mrec;
  char        *map_name, opchr;
  int          error = 0;
  static char  buffer[THEBUFFLENGTH];
  const char  *p;
  char        *q;
  int         count;

  skip_white(&ap->curptr, ap->endptr);
  if (ap->curptr >= ap->endptr) {
    spc_warn(spe, "Empty mapline special?");
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
      free(map_name);
    } else {
      spc_warn(spe, "Invalid fontmap line: Missing TFM name.");
      error = -1;
    }
    break;
  default:
    p = ap->curptr;
    q = buffer;
    count = 0;
    while (p < ap->endptr && count < THEBUFFLENGTH - 1) {
      *q++ = *p++;
      count++;
    }
    if (count == THEBUFFLENGTH - 1) {
      spc_warn(spe, "Invalid fontmap line: Too long a line.");
      *q = 0;
      return -1;
    }
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
    free(mrec);
    break;
  }
  if (!error)
    ap->curptr = ap->endptr;

  return 0;
}

static int
spc_handler_pdfm_mapfile (struct spc_env *spe, struct spc_arg *args)
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
  free(mapfile);

  return  error;
}


static int
spc_handler_pdfm_tounicode (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  char *cmap_name;
  pdf_obj *taint_keys;

  /* First clear */
  sd->cd.cmap_id = -1;
  sd->cd.unescape_backslash = 0;

  skip_white(&args->curptr, args->endptr);
  if (args->curptr >= args->endptr) {
    spc_warn(spe, "Missing CMap name for pdf:tounicode.");
    return  -1;
  }

  /* _FIXME_
   * Any valid char allowed for PDF name object should be allowed here.
   * The argument to this special should be a PDF name obejct.
   * But it's too late to change this special.
   */
  cmap_name = parse_ident(&args->curptr, args->endptr);
  if (!cmap_name) {
    spc_warn(spe, "Missing ToUnicode mapping name...");
    return -1;
  }

  sd->cd.cmap_id = CMap_cache_find(cmap_name);
  if (sd->cd.cmap_id < 0) {
    spc_warn(spe, "Failed to load ToUnicode mapping: %s", cmap_name);
    free(cmap_name);
    return -1;
  }

  /* Shift-JIS like encoding may contain backslash in 2nd byte.
   * WARNING: This will add nasty extension to PDF parser.
   */
  if (sd->cd.cmap_id >= 0) {
    if (strstr(cmap_name, "RKSJ") ||
        strstr(cmap_name, "B5")   ||
        strstr(cmap_name, "GBK")  ||
        strstr(cmap_name, "KSC"))
      sd->cd.unescape_backslash = 1;
  }
  free(cmap_name);

  /* Additional "taint key"
   * An array of PDF name objects can be supplied optionally.
   * Dictionary entries specified by this option will be added to the list
   * of dictionary keys to be treated as the target of "ToUnicode" conversion.
   */
  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {
    taint_keys = parse_pdf_object(&args->curptr, args->endptr, NULL);
    if (taint_keys) {
      if (PDF_OBJ_ARRAYTYPE(taint_keys)) {
        int i;
        for (i = 0; i < pdf_array_length(taint_keys); i++) {
          pdf_obj *key;
        
          key = pdf_get_array(taint_keys, i);
          if (PDF_OBJ_NAMETYPE(key))
            pdf_add_array(sd->cd.taintkeys, pdf_link_obj(key));
          else {
            spc_warn(spe, "Invalid argument specified in pdf:tounicode special.");
          }
        }
      } else {
        spc_warn(spe, "Invalid argument specified in pdf:unicode special.");
      }
      pdf_release_obj(taint_keys);
    }
  }

  return 0;
}

static int
spc_handler_pdfm_pageresources (struct spc_env *spe, struct spc_arg *args)
{
  struct spc_pdf_ *sd = &_pdf_stat;
  pdf_obj *dict;

  dict = parse_pdf_object(&args->curptr, args->endptr, NULL);
  if (!dict) {
    spc_warn(spe, "Dictionary object expected but not found.");
    return  -1;
  }

  if (sd->pageresources)
    pdf_release_obj(sd->pageresources);
  sd->pageresources = dict;

  return 0;
}

static int
spc_handler_pdft_compat_page (struct spc_env *spe, struct spc_arg *args)
{
  skip_white(&args->curptr, args->endptr);
  if (args->curptr < args->endptr) {
    pdf_doc_add_page_content(" ", 1);  /* op: */
    pdf_doc_add_page_content(args->curptr, (int) (args->endptr - args->curptr));  /* op: ANY */
  }

  args->curptr = args->endptr;

  return 0;
}

static struct spc_handler pdfm_handlers[] = {
  {"annotation", spc_handler_pdfm_annot},
  {"annotate",   spc_handler_pdfm_annot},
  {"annot",      spc_handler_pdfm_annot},
  {"ann",        spc_handler_pdfm_annot},

  {"outline",    spc_handler_pdfm_outline},
  {"out",        spc_handler_pdfm_outline},

  {"article",    spc_handler_pdfm_article},
  {"art",        spc_handler_pdfm_article},

  {"bead",       spc_handler_pdfm_bead},
  {"thread",     spc_handler_pdfm_bead},

  {"destination", spc_handler_pdfm_dest},
  {"dest",        spc_handler_pdfm_dest},


  {"object",      spc_handler_pdfm_object},
  {"obj",         spc_handler_pdfm_object},


  {"docinfo",     spc_handler_pdfm_docinfo},
  {"docview",     spc_handler_pdfm_docview},

  {"content",     spc_handler_pdfm_content},
  {"put",         spc_handler_pdfm_put},
  {"close",       spc_handler_pdfm_close},
  {"bop",         spc_handler_pdfm_bop},
  {"eop",         spc_handler_pdfm_eop},

  {"image",       spc_handler_pdfm_image},
  {"img",         spc_handler_pdfm_image},
  {"epdf",        spc_handler_pdfm_image},

  {"link",        spc_handler_pdfm_link},
  {"nolink",      spc_handler_pdfm_nolink},

  {"begincolor",  spc_handler_pdfm_bcolor},
  {"bcolor",      spc_handler_pdfm_bcolor},
  {"bc",          spc_handler_pdfm_bcolor},

  {"setcolor",    spc_handler_pdfm_scolor},
  {"scolor",      spc_handler_pdfm_scolor},
  {"sc",          spc_handler_pdfm_scolor},

  {"endcolor",    spc_handler_pdfm_ecolor},
  {"ecolor",      spc_handler_pdfm_ecolor},
  {"ec",          spc_handler_pdfm_ecolor},

  {"begingray",   spc_handler_pdfm_bcolor},
  {"bgray",       spc_handler_pdfm_bcolor},
  {"bg",          spc_handler_pdfm_bcolor},

  {"endgray",     spc_handler_pdfm_ecolor},
  {"egray",       spc_handler_pdfm_ecolor},
  {"eg",          spc_handler_pdfm_ecolor},

  {"bgcolor",     spc_handler_pdfm_bgcolor},
  {"bgc",         spc_handler_pdfm_bgcolor},
  {"bbc",         spc_handler_pdfm_bgcolor},
  {"bbg",         spc_handler_pdfm_bgcolor},

  {"pagesize",    spc_handler_pdfm_pagesize},

  {"bannot",      spc_handler_pdfm_bann},
  {"beginann",    spc_handler_pdfm_bann},
  {"bann",        spc_handler_pdfm_bann},

  {"eannot",      spc_handler_pdfm_eann},
  {"endann",      spc_handler_pdfm_eann},
  {"eann",        spc_handler_pdfm_eann},

  {"btrans",         spc_handler_pdfm_btrans},
  {"begintransform", spc_handler_pdfm_btrans},
  {"begintrans",     spc_handler_pdfm_btrans},
  {"bt",             spc_handler_pdfm_btrans},

  {"etrans",         spc_handler_pdfm_etrans},
  {"endtransform",   spc_handler_pdfm_etrans},
  {"endtrans",       spc_handler_pdfm_etrans},
  {"et",             spc_handler_pdfm_etrans},

  {"bform",          spc_handler_pdfm_bform},
  {"beginxobj",      spc_handler_pdfm_bform},
  {"bxobj",          spc_handler_pdfm_bform},

  {"eform",          spc_handler_pdfm_eform},
  {"endxobj",        spc_handler_pdfm_eform},
  {"exobj",          spc_handler_pdfm_eform},

  {"usexobj",        spc_handler_pdfm_uxobj},
  {"uxobj",          spc_handler_pdfm_uxobj},

  {"tounicode",  spc_handler_pdfm_tounicode},
  {"literal",    spc_handler_pdfm_literal},
  {"stream",     spc_handler_pdfm_stream},
  {"fstream",    spc_handler_pdfm_fstream},
  {"names",      spc_handler_pdfm_names},
  {"mapline",    spc_handler_pdfm_mapline},
  {"mapfile",    spc_handler_pdfm_mapfile},

  {"bcontent",   spc_handler_pdfm_bcontent},
  {"econtent",   spc_handler_pdfm_econtent},
  {"code",       spc_handler_pdfm_code},

  {"minorversion", spc_handler_pdfm_do_nothing},
  {"majorversion", spc_handler_pdfm_do_nothing},
  {"encrypt",      spc_handler_pdfm_do_nothing},

  {"pageresources", spc_handler_pdfm_pageresources},
  {"trailerid", spc_handler_pdfm_do_nothing},
};

static struct spc_handler pdft_compat_handlers[] = {
  /* Text supplied to "direct" command should go inside of BT/ET block
   * but dvipdfmx currently can't be implemented so.
   * Here, "direct" is for the moment just an alias of "page".
   */
  {"direct", spc_handler_pdft_compat_page},
  {"page", spc_handler_pdft_compat_page},
};

bool
spc_pdfm_check_special (const char *buf, int len)
{
  const char *p, *endptr;

  p      = buf;
  endptr = p + len;

  skip_white(&p, endptr);
  if (p + strlen("pdf:") <= endptr &&
      !memcmp(p, "pdf:", strlen("pdf:"))) {
    return true;
  }

  return false;
}

int
spc_pdfm_setup_handler (struct spc_handler *sph,
                        struct spc_env *spe, struct spc_arg *ap)
{
  int    error = -1;
  size_t i;
  char  *q;

  assert(sph && spe && ap);

  skip_white(&ap->curptr, ap->endptr);
  if (ap->curptr + strlen("pdf:") >= ap->endptr ||
      memcmp(ap->curptr, "pdf:", strlen("pdf:"))) {
    spc_warn(spe, "Not pdf: special???");
    return  -1;
  }
  ap->curptr += strlen("pdf:");

  skip_white(&ap->curptr, ap->endptr);
  q = parse_c_ident(&ap->curptr, ap->endptr);
  if (q) {
    int is_pdft_compat = 0;
    if (ap->curptr < ap->endptr) {
      if (ap->curptr[0] == ':') {
        is_pdft_compat = 1;
        ap->curptr++;
      }
    }
    if (is_pdft_compat) {
      for (i = 0; i < sizeof(pdft_compat_handlers) / sizeof(struct spc_handler); i++) {
        if (!strcmp(q, pdft_compat_handlers[i].key)) {
          ap->command = pdft_compat_handlers[i].key;
          sph->key   = "pdf:";
          sph->exec  = pdft_compat_handlers[i].exec;
          skip_white(&ap->curptr, ap->endptr);
          error = 0;
          break;
        }
      }
    } else {
      for (i = 0; i < sizeof(pdfm_handlers) / sizeof(struct spc_handler); i++) {
        if (!strcmp(q, pdfm_handlers[i].key)) {
          ap->command = pdfm_handlers[i].key;
          sph->key   = "pdf:";
          sph->exec  = pdfm_handlers[i].exec;
          skip_white(&ap->curptr, ap->endptr);
          error = 0;
          break;
        }
      }
    }
    free(q);
  }

  return  error;
}
