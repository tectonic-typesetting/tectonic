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

#include "system.h"
#include "mem.h"
#include "error.h"
#include "dpxutil.h"

#include "pdfdraw.h"
#include "pdfdev.h"
#include "pdfximage.h"

#include "pdfdoc.h"

#include "specials.h"
#include "spc_util.h"

#include "spc_html.h"


#define  ENABLE_HTML_IMG_SUPPORT   1
#define  ENABLE_HTML_SVG_TRANSFORM 1
#define  ENABLE_HTML_SVG_OPACITY   1

/* _FIXME_
 * Please rewrite this or remove html special support
 */

#define  ANCHOR_TYPE_HREF  0
#define  ANCHOR_TYPE_NAME  1

struct spc_html_
{
  struct {
    int  extensions;
  } opts;

  pdf_obj  *link_dict;
  char     *baseurl;
  int       pending_type;
};

static struct spc_html_ _html_state = {
  { 0 },
  NULL, NULL, -1
};


#ifdef  ENABLE_HTML_SVG_TRANSFORM
static int cvt_a_to_tmatrix (pdf_tmatrix *M, const char *ptr, const char **nextptr);
#endif /* ENABLE_HTML_SVG_TRANSFORM */

#define \
downcasify(s) \
if ((s)) { \
  char  *_p = (char *) (s); \
  while (*(_p) != 0) { \
    if (*(_p) >= 'A' && *(_p) <= 'Z') { \
      *(_p) = (*(_p) - 'A') + 'a'; \
    } \
    _p++; \
  } \
}

static int
parse_key_val (const char **pp, const char *endptr, char **kp, char **vp)
{
  const char *q, *p;
  char  *k, *v;
  int    n, error = 0;

  for (p = *pp ; p < endptr && isspace((unsigned char)*p); p++)
    ;
#if  0
  while (!error && p < endptr &&
         ((*p >= 'a' && *p <= 'z') ||
          (*p >= 'A' && *p <= 'Z'))
        ) {
#endif
    k = v = NULL;
    for (q = p, n = 0;
         p < endptr &&
         ((*p >= 'a' && *p <= 'z') ||
          (*p >= 'A' && *p <= 'Z') ||
          (*p >= '0' && *p <= '9') ||
           *p == '-' || *p == ':'
         ); n++, p++);
    if (n == 0) {
#if  0
      break;
#else
      *kp = *vp = NULL;
      return  -1;
#endif
    }
    k = NEW(n + 1, char);
    memcpy(k, q, n); k[n] = '\0';
    if (p + 2 >= endptr || p[0] != '=' || (p[1] != '\"' && p[1] != '\'')) {
      RELEASE(k); k = NULL;
      *pp = p;
      error = -1;
    } else {
      char  qchr = p[1];
      p += 2; /* skip '="' */
      for (q = p, n = 0; p < endptr && *p != qchr; p++, n++);
      if (p == endptr || *p != qchr)
        error = -1;
      else {
        v = NEW(n + 1, char);
        memcpy(v, q, n); v[n] = '\0';
#if  0
        pdf_add_dict(t->attr,
                     pdf_new_name(k),
                     pdf_new_string(v, n));
        RELEASE(v);
#endif
        p++;
      }
    }
#if  0
    RELEASE(k);
    if (!error)
      for ( ; p < endptr && isspace(*p); p++);
  }
#endif

  *kp = k; *vp = v; *pp = p;
  return  error;
}

#define  HTML_TAG_NAME_MAX    127
#define  HTML_TAG_TYPE_EMPTY  1
#define  HTML_TAG_TYPE_OPEN   1
#define  HTML_TAG_TYPE_CLOSE  2

static int
read_html_tag (char *name, pdf_obj *attr, int *type, const char **pp, const char *endptr)
{
  const char *p = *pp;
  int    n = 0, error = 0;

  for ( ; p < endptr && isspace((unsigned char)*p); p++);
  if (p >= endptr || *p != '<')
    return  -1;

  *type = HTML_TAG_TYPE_OPEN;
  for (++p; p < endptr && isspace((unsigned char)*p); p++);
  if (p < endptr && *p == '/') {
    *type = HTML_TAG_TYPE_CLOSE;
    for (++p; p < endptr && isspace((unsigned char)*p); p++);
  }

#define ISDELIM(c) ((c) == '>' || (c) == '/' || isspace((unsigned char)c))
  for (n = 0; p < endptr && n < HTML_TAG_NAME_MAX && !ISDELIM(*p); n++, p++) {
    name[n] = *p;
  } 
  name[n] = '\0';
  if (n == 0 || p == endptr || !ISDELIM(*p)) {
    *pp = p;
    return  -1;
  }

  for ( ; p < endptr && isspace((unsigned char)*p); p++);
  while (p < endptr && !error && *p != '/' && *p != '>') {
    char  *kp = NULL, *vp = NULL;
    error = parse_key_val(&p, endptr, &kp, &vp);
    if (!error) {
      downcasify(kp);
      pdf_add_dict(attr,
                   pdf_new_name(kp),
                   pdf_new_string(vp, strlen(vp) + 1)); /* include trailing NULL here!!! */
      RELEASE(kp);
      RELEASE(vp);
    }
    for ( ; p < endptr && isspace((unsigned char)*p); p++);
  }
  if (error) {
    *pp = p;
    return  error;
  }

  if (p < endptr && *p == '/') {
    *type = HTML_TAG_TYPE_EMPTY;
    for (++p; p < endptr && isspace((unsigned char)*p); p++);
  }
  if (p == endptr || *p != '>') {
    *pp = p;
    return  -1;
  }
  p++;

  downcasify(name);
  *pp = p;
  return  0;
}


static int
spc_handler_html__init (void *dp)
{
  struct spc_html_ *sd = dp;

  sd->link_dict    = NULL;
  sd->baseurl      = NULL;
  sd->pending_type = -1;

  return  0;
}

static int
spc_handler_html__clean (struct spc_env *spe, void *dp)
{
  struct spc_html_ *sd = dp;

  if (sd->baseurl)
    RELEASE(sd->baseurl);

  if (sd->pending_type >= 0 || sd->link_dict)
    spc_warn(spe, "Unclosed html anchor found.");

  if (sd->link_dict)
    pdf_release_obj(sd->link_dict);

  sd->pending_type = -1;
  sd->baseurl      = NULL;
  sd->link_dict    = NULL;

  return  0;
}


static int
spc_handler_html__bophook (struct spc_env *spe, void *dp)
{
  struct spc_html_ *sd = dp;

  if (sd->pending_type >= 0) {
    spc_warn(spe, "...html anchor continues from previous page processed...");
  }

  return  0;
}

static int
spc_handler_html__eophook (struct spc_env *spe, void *dp)
{
  struct spc_html_ *sd = dp;

  if (sd->pending_type >= 0) {
    spc_warn(spe, "Unclosed html anchor at end-of-page!");
  }

  return  0;
}


static char *
fqurl (const char *baseurl, const char *name)
{
  char  *q;
  int    len = 0;

  len = strlen(name);
  if (baseurl)
    len += strlen(baseurl) + 1; /* we may want to add '/' */

  q = NEW(len + 1, char);
  *q = '\0';
  if (baseurl && baseurl[0]) {
    char  *p;
    strcpy(q, baseurl);
    p = q + strlen(q) - 1;
    if (*p == '/')
      *p = '\0';
    if (name[0] && name[0] != '/')
      strcat(q, "/");
  }
  strcat(q, name);

  return  q;
}

static int
html_open_link (struct spc_env *spe, const char *name, struct spc_html_ *sd)
{
  pdf_obj  *color;
  char     *url;

  ASSERT( name );
  ASSERT( sd->link_dict == NULL ); /* Should be checked somewhere else */

  sd->link_dict = pdf_new_dict();
  pdf_add_dict(sd->link_dict,
	       pdf_new_name("Type"),    pdf_new_name ("Annot"));
  pdf_add_dict(sd->link_dict,
	       pdf_new_name("Subtype"), pdf_new_name ("Link"));

  color = pdf_new_array ();
  pdf_add_array(color, pdf_new_number(0.0));
  pdf_add_array(color, pdf_new_number(0.0));
  pdf_add_array(color, pdf_new_number(1.0));
  pdf_add_dict(sd->link_dict, pdf_new_name("C"), color);

  url = fqurl(sd->baseurl, name);
  if (url[0] == '#') {
    /* url++; causes memory leak in RELEASE(url) */
    pdf_add_dict(sd->link_dict,
		 pdf_new_name("Dest"),
		 pdf_new_string(url+1, strlen(url+1)));
  } else { /* Assume this is URL */
    pdf_obj  *action = pdf_new_dict();
    pdf_add_dict(action,
		 pdf_new_name("Type"),
		 pdf_new_name("Action"));
    pdf_add_dict(action,
		 pdf_new_name("S"),
		 pdf_new_name("URI"));
    pdf_add_dict(action,
		 pdf_new_name("URI"),
		 pdf_new_string(url, strlen(url)));
    pdf_add_dict(sd->link_dict,
		 pdf_new_name("A"),
		 pdf_link_obj(action));
    pdf_release_obj(action);
  }
  RELEASE(url);

  spc_begin_annot(spe, sd->link_dict);

  sd->pending_type = ANCHOR_TYPE_HREF;

  return  0;
}

static int
html_open_dest (struct spc_env *spe, const char *name, struct spc_html_ *sd)
{
  int        error;
  pdf_obj   *array, *page_ref;
  pdf_coord  cp;

  cp.x = spe->x_user; cp.y = spe->y_user;
  pdf_dev_transform(&cp, NULL);

  page_ref = pdf_doc_this_page_ref();
  ASSERT( page_ref ); /* Otherwise must be bug */

  array = pdf_new_array();
  pdf_add_array(array, page_ref);
  pdf_add_array(array, pdf_new_name("XYZ"));
  pdf_add_array(array, pdf_new_null());
  pdf_add_array(array, pdf_new_number(cp.y + 24.0));
  pdf_add_array(array, pdf_new_null());

  error = pdf_doc_add_names("Dests",
			    name, strlen(name),
			    array);

  if (error)
    spc_warn(spe, "Failed to add named destination: %s", name);

  sd->pending_type = ANCHOR_TYPE_NAME;

  return  error;
}

#define ANCHOR_STARTED(s) ((s)->pending_type >= 0 || (s)->link_dict)

static int
spc_html__anchor_open (struct spc_env *spe, pdf_obj *attr, struct spc_html_ *sd)
{
  pdf_obj *href, *name;
  int      error = 0;

  if (ANCHOR_STARTED(sd)) {
    spc_warn(spe, "Nested html anchors found!");
    return  -1;
  }

  href = pdf_lookup_dict(attr, "href");
  name = pdf_lookup_dict(attr, "name");
  if (href && name) {
    spc_warn(spe, "Sorry, you can't have both \"href\" and \"name\" in anchor tag...");
    error = -1;
  } else if (href) {
    error = html_open_link(spe, pdf_string_value(href), sd);
  } else if (name) { /* name */
    error = html_open_dest(spe, pdf_string_value(name), sd);
  } else {
    spc_warn(spe, "You should have \"href\" or \"name\" in anchor tag!");
    error = -1;
  }

  return  error;
}

static int
spc_html__anchor_close (struct spc_env *spe, struct spc_html_ *sd)
{
  int  error = 0;

  switch (sd->pending_type) {
  case  ANCHOR_TYPE_HREF:
    if (sd->link_dict) {
      spc_end_annot(spe);
      pdf_release_obj(sd->link_dict);
      sd->link_dict    = NULL;
      sd->pending_type = -1;
    } else {
      spc_warn(spe, "Closing html anchor (link) without starting!");
      error = -1;
    }
    break;
  case  ANCHOR_TYPE_NAME:
    sd->pending_type = -1;
    break;
  default:
    spc_warn(spe, "No corresponding opening tag for html anchor.");
    error = -1;
    break;
  }

  return  error;
}

static int
spc_html__base_empty (struct spc_env *spe, pdf_obj *attr, struct spc_html_ *sd)
{
  pdf_obj *href;
  char    *vp;

  href = pdf_lookup_dict(attr, "href");
  if (!href) {
    spc_warn(spe, "\"href\" not found for \"base\" tag!");
    return  -1;
  }

  vp = (char *) pdf_string_value(href);
  if (sd->baseurl) {
    spc_warn(spe, "\"baseurl\" changed: \"%s\" --> \"%s\"", sd->baseurl, vp);
    RELEASE(sd->baseurl);
  }
  sd->baseurl = NEW(strlen(vp) + 1, char);
  strcpy(sd->baseurl, vp);

  return  0;
}


#ifdef  ENABLE_HTML_IMG_SUPPORT
/* This isn't completed.
 * Please think about placement of images.
 */
static double
atopt (const char *a)
{
  char   *q;
  const char *p = a;
  double  v, u = 1.0;
  const char *_ukeys[] = {
#define K_UNIT__PT  0
#define K_UNIT__IN  1
#define K_UNIT__CM  2
#define K_UNIT__MM  3
#define K_UNIT__BP  4
    "pt", "in", "cm", "mm", "bp",
#define K_UNIT__PX  5
    "px",
     NULL
  };
  int     k;

  q = parse_float_decimal(&p, p + strlen(p));
  if (!q) {
    WARN("Invalid length value: %s (%c)", a, *p);
    return  0.0;
  }

  v = atof(q);
  RELEASE(q);

  q = parse_c_ident(&p, p + strlen(p));
  if (q) {
    for (k = 0; _ukeys[k] && strcmp(_ukeys[k], q); k++);
    switch (k) {
    case K_UNIT__PT: u *= 72.0 / 72.27; break;
    case K_UNIT__IN: u *= 72.0; break;
    case K_UNIT__CM: u *= 72.0 / 2.54 ; break;
    case K_UNIT__MM: u *= 72.0 / 25.4 ; break;
    case K_UNIT__BP: u *= 1.0 ; break;
    case K_UNIT__PX: u *= 1.0 ; break; /* 72dpi */
    default:
      WARN("Unknown unit of measure: %s", q);
      break;
    }
    RELEASE(q);
  }

  return  v * u;
}


#ifdef  ENABLE_HTML_SVG_OPACITY
/* Replicated from spc_tpic */
static pdf_obj *
create_xgstate (double a /* alpha */, int f_ais /* alpha is shape */)
{
  pdf_obj  *dict;

  dict = pdf_new_dict();
  pdf_add_dict(dict,
               pdf_new_name("Type"),
               pdf_new_name("ExtGState"));
  if (f_ais) {
    pdf_add_dict(dict,
                 pdf_new_name("AIS"),
                 pdf_new_boolean(1));
  }
  pdf_add_dict(dict,
               pdf_new_name("ca"),
               pdf_new_number(a));

  return  dict;
}

static int
check_resourcestatus (const char *category, const char *resname)
{
  pdf_obj  *dict1, *dict2;

  dict1 = pdf_doc_current_page_resources();
  if (!dict1)
    return  0;

  dict2 = pdf_lookup_dict(dict1, category);
  if (dict2 &&
      pdf_obj_typeof(dict2) == PDF_DICT) {
    if (pdf_lookup_dict(dict2, resname))
      return  1;
  }
  return  0;
}
#endif /* ENABLE_HTML_SVG_OPACITY */

static int
spc_html__img_empty (struct spc_env *spe, pdf_obj *attr)
{
  pdf_obj       *src, *obj;
  transform_info ti;
  load_options   options = {1, 0, NULL};
  int            id, error = 0;
#ifdef  ENABLE_HTML_SVG_OPACITY
  double         alpha = 1.0; /* meaning fully opaque */
#endif /* ENABLE_HTML_SVG_OPACITY */
#ifdef  ENABLE_HTML_SVG_TRANSFORM
  pdf_tmatrix    M, M1;

  pdf_setmatrix(&M, 1.0, 0.0, 0.0, 1.0, spe->x_user, spe->y_user);
#endif /* ENABLE_HTML_SVG_TRANSFORM */

  spc_warn(spe, "html \"img\" tag found (not completed, plese don't use!).");

  src = pdf_lookup_dict(attr, "src");
  if (!src) {
    spc_warn(spe, "\"src\" attribute not found for \"img\" tag!");
    return  -1;
  }

  transform_info_clear(&ti);
  obj = pdf_lookup_dict(attr, "width");
  if (obj) {
    ti.width  = atopt(pdf_string_value(obj));
    ti.flags |= INFO_HAS_WIDTH;
  }
  obj = pdf_lookup_dict(attr, "height");
  if (obj) {
    ti.height = atopt(pdf_string_value(obj));
    ti.flags |= INFO_HAS_HEIGHT;
  }

#ifdef  ENABLE_HTML_SVG_OPACITY
  obj = pdf_lookup_dict(attr, "svg:opacity");
  if (obj) {
    alpha = atof(pdf_string_value(obj));
    if (alpha < 0.0 || alpha > 1.0) {
      spc_warn(spe, "Invalid opacity value: %s", pdf_string_value(obj));
      alpha = 1.0;
    }
  }
#endif /* ENABLE_HTML_SVG_OPCAITY */

#ifdef  ENABLE_HTML_SVG_TRANSFORM
  obj = pdf_lookup_dict(attr, "svg:transform");
  if (obj) {
    const char *p = pdf_string_value(obj);
    pdf_tmatrix  N;
    for ( ; *p && isspace((unsigned char)*p); p++);
    while (*p && !error) {
      pdf_setmatrix(&N, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0);
      error = cvt_a_to_tmatrix(&N, p, &p);
      if (!error) {
        N.f = -N.f;
        pdf_concatmatrix(&M, &N);
        for ( ; *p && isspace((unsigned char)*p); p++);
        if (*p == ',')
          for (++p; *p && isspace((unsigned char)*p); p++);
      }
    }
  }
#endif /* ENABLE_HTML_SVG_TRANSFORM */

  if (error) {
    spc_warn(spe, "Error in html \"img\" tag attribute.");
    return  error;
  }

  id = pdf_ximage_findresource(pdf_string_value(src), options);
  if (id < 0) {
    spc_warn(spe, "Could not find/load image: %s", pdf_string_value(src)); 
    error = -1;
  } else {
#if defined(ENABLE_HTML_SVG_TRANSFORM) || defined(ENABLE_HTML_SVG_OPACITY)
    {
      char     *res_name;
      pdf_rect  r;

      graphics_mode();

      pdf_dev_gsave();

#ifdef  ENABLE_HTML_SVG_OPACITY
      {
        pdf_obj *dict;
        int      a = round(100.0 * alpha);
        if (a != 0) {
          res_name = NEW(strlen("_Tps_a100_") + 1, char);
          sprintf(res_name, "_Tps_a%03d_", a); /* Not Tps prefix but... */
          if (!check_resourcestatus("ExtGState", res_name)) {
            dict = create_xgstate(round_at(0.01 * a, 0.01), 0);
            pdf_doc_add_page_resource("ExtGState",
                                      res_name, pdf_ref_obj(dict));
            pdf_release_obj(dict);
          }
          pdf_doc_add_page_content(" /", 2);  /* op: */
          pdf_doc_add_page_content(res_name, strlen(res_name));  /* op: */
          pdf_doc_add_page_content(" gs", 3);  /* op: gs */
          RELEASE(res_name);
        }
      }
#endif /* ENABLE_HTML_SVG_OPACITY */

      pdf_ximage_scale_image(id, &M1, &r, &ti);
      pdf_concatmatrix(&M, &M1);
      pdf_dev_concat(&M);

      pdf_dev_rectclip(r.llx, r.lly, r.urx - r.llx, r.ury - r.lly);

      res_name = pdf_ximage_get_resname(id);
      pdf_doc_add_page_content(" /", 2);  /* op: */
      pdf_doc_add_page_content(res_name, strlen(res_name));  /* op: */
      pdf_doc_add_page_content(" Do", 3);  /* op: Do */

      pdf_dev_grestore();

      pdf_doc_add_page_resource("XObject",
                                res_name,
                                pdf_ximage_get_reference(id));
    }
#else
    pdf_dev_put_image(id, &ti, spe->x_user, spe->y_user);
#endif /* ENABLE_HTML_SVG_XXX */
  }

  return  error;
}
#else
static int
spc_html__img_empty (struct spc_env *spe, pdf_obj *attr)
{
  spc_warn(spe, "IMG tag not yet supported yet...");
  return  -1;
}
#endif  /* ENABLE_HTML_IMG_SUPPORT */


static int
spc_handler_html_default (struct spc_env *spe, struct spc_arg *ap)
{
  struct spc_html_ *sd = &_html_state;
  char      name[HTML_TAG_NAME_MAX + 1];
  pdf_obj  *attr;
  int       error = 0, type = HTML_TAG_TYPE_OPEN;

  if (ap->curptr >= ap->endptr)
    return  0;

  attr  = pdf_new_dict();
  error = read_html_tag(name, attr, &type, &ap->curptr, ap->endptr);
  if (error) {
    pdf_release_obj(attr);
    return  error;
  }
  if (!strcmp(name, "a")) {
    switch (type) {
    case  HTML_TAG_TYPE_OPEN:
      error = spc_html__anchor_open (spe, attr, sd);
      break;
    case  HTML_TAG_TYPE_CLOSE:
      error = spc_html__anchor_close(spe, sd);
      break;
    default:
      spc_warn(spe, "Empty html anchor tag???");
      error = -1;
      break;
    }
  } else if (!strcmp(name, "base")) {
    if (type == HTML_TAG_TYPE_CLOSE) {
      spc_warn(spe, "Close tag for \"base\"???");
      error = -1;
    } else { /* treat "open" same as "empty" */
      error = spc_html__base_empty(spe, attr, sd);
    }
  } else if (!strcmp(name, "img")) {
    if (type == HTML_TAG_TYPE_CLOSE) {
      spc_warn(spe, "Close tag for \"img\"???");
      error = -1;
    } else { /* treat "open" same as "empty" */
      error = spc_html__img_empty(spe, attr);
    }
  }
  pdf_release_obj(attr);

  for ( ; ap->curptr < ap->endptr && isspace((unsigned char)ap->curptr[0]); ap->curptr++);

  return  error;
}


#ifdef  ENABLE_HTML_SVG_TRANSFORM
/* translate wsp* '(' wsp* number (comma-wsp number)? wsp* ')' */
static int
cvt_a_to_tmatrix (pdf_tmatrix *M, const char *ptr, const char **nextptr)
{
  char        *q;
  const char  *p = ptr;
  int          n;
  double       v[6];
  static const char *_tkeys[] = {
#define  K_TRNS__MATRIX     0
    "matrix",    /* a b c d e f */
#define  K_TRNS__TRANSLATE  1
    "translate", /* tx [ty] : dflt. tf = 0 */
#define  K_TRNS__SCALE      2
    "scale",     /* sx [sy] : dflt. sy = sx */
#define  K_TRNS__ROTATE     3
    "rotate",    /* ang [cx cy] : dflt. cx, cy = 0 */
#define  K_TRNS__SKEWX      4
#define  K_TRNS__SKEWY      5
    "skewX",     /* ang */
    "skewY",     /* ang */
    NULL
  };
  int          k;

  for ( ; *p && isspace((unsigned char)*p); p++);

  q = parse_c_ident(&p, p + strlen(p));
  if (!q)
    return -1;
  /* parsed transformation key */
  for (k = 0; _tkeys[k] && strcmp(q, _tkeys[k]); k++);
  RELEASE(q);

  /* handle args */
  for ( ; *p && isspace((unsigned char)*p); p++);
  if (*p != '(' || *(p + 1) == 0)
    return  -1;
  for (++p; *p && isspace((unsigned char)*p); p++);
  for (n = 0; n < 6 && *p && *p != ')'; n++) {
    q = parse_float_decimal(&p, p + strlen(p));
    if (!q)
      break;
    else {
      v[n] = atof(q);
      if (*p == ',')
        p++;
      for ( ; *p && isspace((unsigned char)*p); p++);
      if (*p == ',')
        for (++p; *p && isspace((unsigned char)*p); p++);
      RELEASE(q);
    }
  }
  if (*p != ')')
    return  -1;
  p++;

  switch (k) {
  case  K_TRNS__MATRIX:
    if (n != 6)
      return  -1;
    M->a = v[0]; M->c = v[1];
    M->b = v[2]; M->d = v[3];
    M->e = v[4]; M->f = v[5];
    break;
  case  K_TRNS__TRANSLATE:
    if (n != 1 && n != 2)
      return  -1;
    M->a = M->d = 1.0;
    M->c = M->b = 0.0;
    M->e = v[0]; M->f = (n == 2) ? v[1] : 0.0;
    break;
  case  K_TRNS__SCALE:
    if (n != 1 && n != 2)
      return  -1;
    M->a = v[0]; M->d = (n == 2) ? v[1] : v[0];
    M->c = M->b = 0.0;
    M->e = M->f = 0.0;
    break;
  case  K_TRNS__ROTATE:
    if (n != 1 && n != 3)
      return  -1;
    M->a = cos(v[0] * M_PI / 180.0);
    M->c = sin(v[0] * M_PI / 180.0);
    M->b = -M->c; M->d = M->a;
    M->e = (n == 3) ? v[1] : 0.0;
    M->f = (n == 3) ? v[2] : 0.0;
    break;
  case  K_TRNS__SKEWX:
    if (n != 1)
       return  -1;
    M->a = M->d = 1.0;
    M->c = 0.0;
    M->b = tan(v[0] * M_PI / 180.0);
    break;
  case  K_TRNS__SKEWY:
    if (n != 1)
       return  -1;
    M->a = M->d = 1.0;
    M->c = tan(v[0] * M_PI / 180.0);
    M->b = 0.0;
    break;
  }

  if (nextptr)
    *nextptr = p;
  return  0;
}    
#endif /* ENABLE_HTML_SVG_TRANSFORM */

int
spc_html_at_begin_document (void)
{
  struct spc_html_ *sd = &_html_state;
  return  spc_handler_html__init(sd);
}

int
spc_html_at_begin_page (void)
{
  struct spc_html_ *sd = &_html_state;
  return  spc_handler_html__bophook(NULL, sd);
}

int
spc_html_at_end_page (void)
{
  struct spc_html_ *sd = &_html_state;
  return  spc_handler_html__eophook(NULL, sd);
}

int
spc_html_at_end_document (void)
{
  struct spc_html_ *sd = &_html_state;
  return  spc_handler_html__clean(NULL, sd);
}


int
spc_html_check_special (const char *buffer, int size)
{
  const char *p, *endptr;

  p      = buffer;
  endptr = p + size;

  for ( ; p < endptr && isspace((unsigned char)*p); p++);
  size   = (int) (endptr - p);
  if (size >= strlen("html:") &&
      !memcmp(p, "html:", strlen("html:"))) {
    return  1;
  }

  return  0;
}


int
spc_html_setup_handler (struct spc_handler *sph,
                        struct spc_env *spe, struct spc_arg *ap)
{
  ASSERT(sph && spe && ap);

  for ( ; ap->curptr < ap->endptr && isspace((unsigned char)ap->curptr[0]); ap->curptr++);
  if (ap->curptr + strlen("html:") > ap->endptr ||
      memcmp(ap->curptr, "html:", strlen("html:"))) {
    return  -1;
  }

  ap->command = "";

  sph->key    = "html:";
  sph->exec   = &spc_handler_html_default;

  ap->curptr += strlen("html:");
  for ( ; ap->curptr < ap->endptr && isspace((unsigned char)ap->curptr[0]); ap->curptr++);

  return  0;
}
